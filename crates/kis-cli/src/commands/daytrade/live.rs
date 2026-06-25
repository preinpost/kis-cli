//! `LiveExecutor` — KIS 실주문 실행기.
//!
//! 국내(`order_cash`) / 해외(`order`) 주문 발주 + 체결 확인 폴링
//! (`inquire_daily_ccld` / `inquire_ccnl`, ODNO 필터, 30초 타임아웃) +
//! `inquire_balance` 기반 포지션 동기화.
//!
//! 주문 가격: 분봉 종가 ± `tick_offset` 호가 단위. 해외는 지정가만 가능 (시장가 X).
//! 체결 확인 실패(타임아웃·부분체결)는 경고 로그 + 잔고 동기화로 위치 보정.

use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::api::domestic_stock::order_account::{
    inquire_balance as dome_balance, inquire_daily_ccld as dome_ccld, order_cash,
};
use crate::api::overseas_stock::order_account::{
    inquire_balance as usa_balance, inquire_ccnl as usa_ccnl, order as usa_order,
};
use crate::client::KisClient;
use crate::symbols::Market as SymMarket;

use super::engine::{Executor, Fill, Position};
use super::session::{self, Market};
use super::storage::Mode;

/// 한 호가 단위 (대략값). 정확한 호가는 가격대별로 다르지만,
/// 데이트레이드 5m봉 기준이라 단순화.
fn tick_size_krx(price: f64) -> f64 {
    match price {
        p if p < 2_000.0 => 1.0,
        p if p < 5_000.0 => 5.0,
        p if p < 20_000.0 => 10.0,
        p if p < 50_000.0 => 50.0,
        p if p < 200_000.0 => 100.0,
        p if p < 500_000.0 => 500.0,
        _ => 1_000.0,
    }
}

fn tick_size_usa(_price: f64) -> f64 {
    0.01 // USD 0.01 (단순화 — 실제 페니/서브페니 무시)
}

pub struct LiveExecutor {
    client: Arc<KisClient>,
    /// resolve된 시장 (Nasdaq/Nyse/Amex/Domestic 등). 해외 주문 시 OVRS_EXCG_CD 결정용.
    sym_market: SymMarket,
    /// 호가 오프셋 — 매수 시 +offset 틱, 매도 시 -offset 틱. 0이면 종가 그대로.
    pub tick_offset: i32,
    /// 체결 확인 폴링 타임아웃 (초).
    pub fill_timeout_secs: u64,
    /// 폴링 간격 (초).
    pub poll_interval_secs: u64,
}

impl LiveExecutor {
    pub fn new(client: Arc<KisClient>, sym_market: SymMarket) -> Self {
        Self {
            client,
            sym_market,
            tick_offset: 0,
            fill_timeout_secs: 30,
            poll_interval_secs: 2,
        }
    }

    fn ovrs_excg_cd(&self) -> &'static str {
        match self.sym_market {
            SymMarket::Nasdaq => "NASD",
            SymMarket::Nyse => "NYSE",
            SymMarket::Amex => "AMEX",
            _ => "NASD",
        }
    }

    fn limit_price(&self, market: Market, side: order_cash::Side, ref_price: f64) -> f64 {
        let tick = if matches!(market, Market::Usa) { tick_size_usa(ref_price) } else { tick_size_krx(ref_price) };
        let signed = match side {
            order_cash::Side::Buy => self.tick_offset as f64,
            order_cash::Side::Sell => -(self.tick_offset as f64),
        };
        ref_price + tick * signed
    }
}

impl Executor for LiveExecutor {
    fn mode(&self) -> Mode { Mode::Run }
    fn start_prefix(&self) -> &'static str {
        "daytrade run 시작 (실주문)"
    }
    fn extra_start_info(&self) -> String {
        format!(" · order=limit · tick_off={} · fill_to={}s", self.tick_offset, self.fill_timeout_secs)
    }

    async fn buy(&self, code: &str, market: Market, qty: u64, ref_price: f64) -> Result<Fill> {
        place_and_poll(self, code, market, OrderSide::Buy, qty, ref_price).await
    }

    async fn sell(&self, code: &str, market: Market, qty: u64, ref_price: f64) -> Result<Fill> {
        place_and_poll(self, code, market, OrderSide::Sell, qty, ref_price).await
    }

    async fn sync_position(&self, code: &str, market: Market) -> Result<Option<Position>> {
        match market {
            Market::Krx => fetch_dome_position(&self.client, code).await,
            Market::Usa => fetch_usa_position(&self.client, code, self.ovrs_excg_cd()).await,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum OrderSide { Buy, Sell }

impl OrderSide {
    fn dome(self) -> order_cash::Side {
        match self { Self::Buy => order_cash::Side::Buy, Self::Sell => order_cash::Side::Sell }
    }
    fn usa(self) -> usa_order::Side {
        match self { Self::Buy => usa_order::Side::Buy, Self::Sell => usa_order::Side::Sell }
    }
    fn dome_filter(self) -> &'static str {
        match self { Self::Buy => "02", Self::Sell => "01" }
    }
    fn usa_filter(self) -> &'static str {
        match self { Self::Buy => "02", Self::Sell => "01" }
    }
}

async fn place_and_poll(
    exec: &LiveExecutor,
    code: &str,
    market: Market,
    side: OrderSide,
    qty: u64,
    ref_price: f64,
) -> Result<Fill> {
    let limit = exec.limit_price(market, side.dome(), ref_price);
    match market {
        Market::Krx => {
            let req = order_cash::Request {
                cano: exec.client.cano().into(),
                acnt_prdt_cd: exec.client.product_code().into(),
                pdno: code.into(),
                sll_type: matches!(side, OrderSide::Sell).then(|| "01".into()),
                ord_dvsn: "00".into(),
                ord_qty: qty.to_string(),
                ord_unpr: format!("{:.0}", limit.round()),
                cndt_pric: None,
                excg_id_dvsn_cd: None,
            };
            let resp = order_cash::call(exec.client.as_ref(), side.dome(), &req).await?;
            poll_dome_fill(&exec.client, &resp.odno, code, side, qty, exec.fill_timeout_secs, exec.poll_interval_secs).await
        }
        Market::Usa => {
            let req = usa_order::Request {
                cano: exec.client.cano().into(),
                acnt_prdt_cd: exec.client.product_code().into(),
                ovrs_excg_cd: exec.ovrs_excg_cd().into(),
                pdno: code.into(),
                ord_qty: qty.to_string(),
                ovrs_ord_unpr: format!("{:.4}", limit),
                ord_svr_dvsn_cd: "0".into(),
                ord_dvsn: "00".into(),
            };
            let resp = usa_order::call(exec.client.as_ref(), usa_order::Market::Usa, side.usa(), &req).await?;
            poll_usa_fill(&exec.client, &resp.odno, exec.ovrs_excg_cd(), side, qty, exec.fill_timeout_secs, exec.poll_interval_secs).await
        }
    }
}

async fn poll_dome_fill(
    client: &KisClient,
    odno: &str,
    code: &str,
    side: OrderSide,
    expected_qty: u64,
    timeout_secs: u64,
    poll_secs: u64,
) -> Result<Fill> {
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    let start = std::time::Instant::now();
    loop {
        let req = dome_ccld::Request {
            cano: client.cano().into(),
            acnt_prdt_cd: client.product_code().into(),
            inqr_strt_dt: today.clone(),
            inqr_end_dt: today.clone(),
            sll_buy_dvsn_cd: side.dome_filter().into(),
            pdno: code.into(),
            ord_gno_brno: "".into(),
            odno: odno.into(),
            ccld_dvsn: "00".into(),
            inqr_dvsn: "00".into(),
            inqr_dvsn_1: "".into(),
            inqr_dvsn_3: "00".into(),
            excg_id_dvsn_cd: "".into(),
            ctx_area_fk100: "".into(),
            ctx_area_nk100: "".into(),
        };
        let resp = dome_ccld::call(client, dome_ccld::Period::Within3Months, &req).await?;
        let mut filled_qty: u64 = 0;
        let mut weighted: f64 = 0.0;
        for r in resp.rows.iter().filter(|r| r.odno == odno) {
            let q: u64 = r.tot_ccld_qty.parse().unwrap_or(0);
            let p: f64 = r.avg_prvs.parse().unwrap_or(0.0);
            filled_qty += q;
            weighted += p * q as f64;
            if r.cncl_yn == "Y" {
                return Err(anyhow!("주문 취소됨 (odno={})", odno));
            }
        }
        if filled_qty >= expected_qty {
            let avg = if filled_qty > 0 { weighted / filled_qty as f64 } else { 0.0 };
            return Ok(Fill { qty: filled_qty, price: avg });
        }
        if start.elapsed().as_secs() >= timeout_secs {
            if filled_qty > 0 {
                let avg = weighted / filled_qty as f64;
                return Err(anyhow!(
                    "체결 확인 타임아웃 ({}초): 부분체결 {}/{}주 @ avg {:.0}원",
                    timeout_secs, filled_qty, expected_qty, avg
                ));
            }
            return Err(anyhow!("체결 확인 타임아웃 ({}초): 미체결 (odno={})", timeout_secs, odno));
        }
        tokio::time::sleep(std::time::Duration::from_secs(poll_secs)).await;
    }
}

async fn poll_usa_fill(
    client: &KisClient,
    odno: &str,
    excg: &str,
    side: OrderSide,
    expected_qty: u64,
    timeout_secs: u64,
    poll_secs: u64,
) -> Result<Fill> {
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    let start = std::time::Instant::now();
    loop {
        let req = usa_ccnl::Request {
            cano: client.cano().into(),
            acnt_prdt_cd: client.product_code().into(),
            pdno: "".into(),
            ord_strt_dt: today.clone(),
            ord_end_dt: today.clone(),
            sll_buy_dvsn: side.usa_filter().into(),
            ccld_nccs_dvsn: "00".into(),
            ovrs_excg_cd: excg.into(),
            sort_sqn: "DS".into(),
            ord_dt: "".into(),
            ord_gno_brno: "".into(),
            odno: odno.into(),
            ctx_area_nk200: "".into(),
            ctx_area_fk200: "".into(),
        };
        let rows = usa_ccnl::call(client, &req).await?;
        let mut filled_qty: u64 = 0;
        let mut weighted: f64 = 0.0;
        for r in rows.iter().filter(|r| r.odno == odno) {
            let q: u64 = r.ft_ccld_qty.parse().unwrap_or(0);
            let p: f64 = r.ft_ccld_unpr3.parse().unwrap_or(0.0);
            filled_qty += q;
            weighted += p * q as f64;
            if r.rvse_cncl_dvsn == "02" {
                return Err(anyhow!("주문 취소됨 (odno={})", odno));
            }
        }
        if filled_qty >= expected_qty {
            let avg = if filled_qty > 0 { weighted / filled_qty as f64 } else { 0.0 };
            return Ok(Fill { qty: filled_qty, price: avg });
        }
        if start.elapsed().as_secs() >= timeout_secs {
            if filled_qty > 0 {
                let avg = weighted / filled_qty as f64;
                return Err(anyhow!(
                    "체결 확인 타임아웃 ({}초): 부분체결 {}/{}주 @ avg {:.4} USD",
                    timeout_secs, filled_qty, expected_qty, avg
                ));
            }
            return Err(anyhow!("체결 확인 타임아웃 ({}초): 미체결 (odno={})", timeout_secs, odno));
        }
        tokio::time::sleep(std::time::Duration::from_secs(poll_secs)).await;
    }
}

async fn fetch_dome_position(client: &KisClient, code: &str) -> Result<Option<Position>> {
    let req = dome_balance::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        afhr_flpr_yn: "N".into(),
        ofl_yn: "".into(),
        inqr_dvsn: "02".into(),
        unpr_dvsn: "01".into(),
        fund_sttl_icld_yn: "N".into(),
        fncg_amt_auto_rdpt_yn: "N".into(),
        prcs_dvsn: "01".into(),
        ctx_area_fk100: "".into(),
        ctx_area_nk100: "".into(),
    };
    let resp = dome_balance::call(client, &req).await?;
    let now = session::now_kst();
    for h in resp.holdings.iter().filter(|h| h.pdno == code) {
        let qty: u64 = h.hldg_qty.parse().unwrap_or(0);
        if qty == 0 { continue; }
        let avg_price: f64 = h.pchs_avg_pric.parse().unwrap_or(0.0);
        return Ok(Some(Position { qty, avg_price, entry_time: now }));
    }
    Ok(None)
}

async fn fetch_usa_position(client: &KisClient, code: &str, excg: &str) -> Result<Option<Position>> {
    let req = usa_balance::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        ovrs_excg_cd: excg.into(),
        tr_crcy_cd: "USD".into(),
        ctx_area_fk200: "".into(),
        ctx_area_nk200: "".into(),
    };
    let resp = usa_balance::call(client, &req).await?;
    let now = session::now_kst();
    for h in resp.holdings.iter().filter(|h| h.ovrs_pdno == code) {
        let qty: u64 = h.ovrs_cblc_qty.parse().unwrap_or(0);
        if qty == 0 { continue; }
        let avg_price: f64 = h.pchs_avg_pric.parse().unwrap_or(0.0);
        return Ok(Some(Position { qty, avg_price, entry_time: now }));
    }
    Ok(None)
}
