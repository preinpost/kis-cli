//! 국내선물옵션 커맨드.

use anyhow::{anyhow, Result};

use crate::api::futureoption_domestic::order_account::{inquire_balance, order};
use crate::api::futureoption_domestic::quotations::inquire_price;
use crate::client::KisClient;
use crate::commands::helpers::{format_number, resolve_symbol};
use crate::symbols::ResolveMode;
use crate::ws;

/// `F` 지수선물, `O` 지수옵션, `JF` 주식선물, `JO` 주식옵션 등.
/// 단순화를 위해 지수선물(`F`) 기본값.
const MARKET: &str = "F";

pub async fn run_price(client: &KisClient, symbol: &str, market: Option<&str>) -> Result<()> {
    let req = inquire_price::Request {
        fid_cond_mrkt_div_code: market.unwrap_or(MARKET).into(),
        fid_input_iscd: symbol.into(),
    };
    let r = inquire_price::call(client, &req).await?;
    let q = r.quote.ok_or_else(|| anyhow!("시세 데이터 없음"))?;
    println!("선물옵션: {} [{}]", symbol, q.hts_kor_isnm);
    println!("{}", "─".repeat(40));
    println!("현재가:   {:>14}", format_number(&q.futs_prpr));
    println!("전일대비: {:>14} ({}%)",
        format_number(&q.futs_prdy_vrss), q.futs_prdy_ctrt);
    println!("시가/고/저: {} / {} / {}",
        format_number(&q.futs_oprc),
        format_number(&q.futs_hgpr),
        format_number(&q.futs_lwpr));
    println!("미결제약정: {:>12}", format_number(&q.hts_otst_stpl_qty));
    println!("거래량:   {:>14}", format_number(&q.acml_vol));
    Ok(())
}

pub async fn run_balance(client: &KisClient) -> Result<()> {
    let req = inquire_balance::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        mgna_dvsn: "01".into(),
        excc_stat_cd: "1".into(),
        ctx_area_fk200: "".into(),
        ctx_area_nk200: "".into(),
    };
    let r = inquire_balance::call(client, &req).await?;
    println!("선물옵션 잔고");
    println!("{}", "═".repeat(95));
    if r.holdings.is_empty() {
        println!("  보유 종목 없음");
    } else {
        println!(
            "{:<12} {:<22} {:<6} {:>8} {:>12} {:>14} {:>14}",
            "코드", "종목명", "구분", "수량", "평균가", "평가금액", "평가손익"
        );
        println!("{}", "─".repeat(95));
        for h in &r.holdings {
            println!(
                "{:<12} {:<22} {:<6} {:>8} {:>12} {:>14} {:>14}",
                h.pdno, h.prdt_name, h.sll_buy_dvsn_name,
                h.cblc_qty,
                format_number(&h.ccld_avg_unpr1),
                format_number(&h.evlu_amt),
                format_number(&h.evlu_pfls_amt),
            );
        }
    }
    if let Some(s) = r.summary {
        println!("{}", "═".repeat(95));
        println!("예탁금현금:   {:>15}", format_number(&s.dnca_cash));
        println!("총예탁자산:   {:>15}", format_number(&s.tot_dncl_amt));
        println!("주문가능총액: {:>15}", format_number(&s.ord_psbl_tota));
        println!("평가손익합계: {:>15}", format_number(&s.evlu_pfls_amt_smtl));
    }
    Ok(())
}

pub async fn run_order(
    client: &KisClient,
    side: Side,
    symbol: &str,
    qty: u64,
    price: Option<f64>,
) -> Result<()> {
    // 지정가 01, 시장가 02
    let (dvsn_cd, unit_price) = match price {
        Some(p) => ("01".to_string(), format!("{:.2}", p)),
        None => ("02".to_string(), "0".to_string()),
    };
    let req = order::Request {
        ord_prcs_dvsn_cd: "02".into(),
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        sll_buy_dvsn_cd: match side { Side::Buy => "02".into(), Side::Sell => "01".into() },
        shtn_pdno: symbol.into(),
        ord_qty: qty.to_string(),
        unit_price,
        ord_dvsn_cd: dvsn_cd,
    };
    let r = order::call(client, order::Session::Day, &req).await?;
    let label = match side { Side::Buy => "매수", Side::Sell => "매도" };
    println!("{} 주문 접수", label);
    println!("  종목:     {} {}", symbol, r.item_name);
    println!("  수량:     {}", qty);
    if let Some(p) = price { println!("  지정가:   {:.2}", p); } else { println!("  시장가"); }
    println!("  주문번호: {}", r.odno);
    println!("  주문시각: {}", r.ord_tmd);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum Side { Buy, Sell }

/// KRX 야간선물 실시간 체결 (H0MFCNT0)
///
/// `symbol`이 코드가 아니면 선물 마스터(FoIdx/FoStk)에서 이름으로 검색 후 단축코드 사용.
/// 주간 단축코드가 야간에도 통용되는지는 KIS 서버 응답에 따라 다름 — 실패 시 HTS 코드 직접 입력.
pub async fn run_watch_night(client: &KisClient, symbol: &str, pick: Option<usize>) -> Result<()> {
    // 숫자+영문 9자 이하면 그대로 코드로 간주, 아니면 마스터 검색
    let code = if looks_like_fo_code(symbol) {
        symbol.to_string()
    } else {
        let sym = resolve_symbol(symbol, ResolveMode::FutureOption, pick)?;
        println!("해석: {} → {} ({})", symbol, sym.code, sym.name_kr);
        sym.code
    };
    ws::run_night_futures(client.token_manager.clone(), &code).await
}

fn looks_like_fo_code(s: &str) -> bool {
    let t = s.trim();
    !t.is_empty() && t.len() <= 12 && t.chars().all(|c| c.is_ascii_alphanumeric())
}
