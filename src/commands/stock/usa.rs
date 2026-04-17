//! 해외주식 커맨드.

use anyhow::{anyhow, Result};

use crate::api::overseas_stock::order_account::{inquire_balance, inquire_ccnl, order};
use crate::api::overseas_stock::quotations::{inquire_daily_chartprice, price};
use crate::client::KisClient;
use crate::commands::helpers::{format_number, resolve_symbol};
use crate::symbols::{Market, ResolveMode, ResolvedSymbol};
use crate::ws;

/// KIS 해외시장 코드 변환
fn excd(market: Market) -> &'static str {
    match market {
        Market::Nasdaq => "NAS",
        Market::Nyse => "NYS",
        Market::Amex => "AMS",
        _ => "NAS",
    }
}

/// 잔고/주문용 거래소 코드 (OVRS_EXCG_CD). 해외주식 거래 API에서 사용.
fn ovrs_excg(market: Market) -> &'static str {
    match market {
        Market::Nasdaq => "NASD",
        Market::Nyse => "NYSE",
        Market::Amex => "AMEX",
        _ => "NASD",
    }
}

fn order_market(_m: Market) -> order::Market {
    order::Market::Usa
}

pub async fn run_price(client: &KisClient, symbol: &str, pick: Option<usize>) -> Result<()> {
    let sym = resolve_symbol(symbol, ResolveMode::Overseas, pick)?;
    let req = price::Request {
        auth: "".into(),
        excd: excd(sym.market).into(),
        symb: sym.code.clone(),
    };
    let r = price::call(client, &req).await?;
    println!("종목: {} [{}] {}", sym.code, sym.name_en, sym.market.as_str());
    println!("{}", "─".repeat(40));
    println!("현재가:   {:>14}", format_number(&r.last));
    println!("전일대비: {:>14} ({}%)", format_number(&r.diff), r.rate);
    println!("거래량:   {:>14}", format_number(&r.tvol));
    println!("거래대금: {:>14}", format_number(&r.tamt));
    Ok(())
}

pub async fn run_chart(
    client: &KisClient,
    symbol: &str,
    period: char,
    pick: Option<usize>,
) -> Result<()> {
    let sym = resolve_symbol(symbol, ResolveMode::Overseas, pick)?;
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    let from = (chrono::Local::now() - chrono::Duration::days(150))
        .format("%Y%m%d")
        .to_string();
    let req = inquire_daily_chartprice::Request {
        fid_cond_mrkt_div_code: "N".into(),
        fid_input_iscd: sym.code.clone(),
        fid_input_date_1: from,
        fid_input_date_2: today,
        fid_period_div_code: period.to_string(),
    };
    let r = inquire_daily_chartprice::call(client, &req).await?;
    println!("[{}] {} — {}봉", sym.code, sym.name_en, match period {
        'D' => "일", 'W' => "주", 'M' => "월", _ => "?"
    });
    println!("{:<10} {:>12} {:>12} {:>12} {:>12}", "일자", "시가", "고가", "저가", "종가");
    println!("{}", "─".repeat(70));
    for c in r.candles.iter().take(30) {
        println!(
            "{:<10} {:>12} {:>12} {:>12} {:>12}",
            c.stck_bsop_date,
            format_number(&c.ovrs_nmix_oprc),
            format_number(&c.ovrs_nmix_hgpr),
            format_number(&c.ovrs_nmix_lwpr),
            format_number(&c.ovrs_nmix_prpr),
        );
    }
    Ok(())
}

pub async fn run_balance(client: &KisClient, exchange: &str) -> Result<()> {
    let req = inquire_balance::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        ovrs_excg_cd: exchange.into(),
        tr_crcy_cd: "USD".into(),
        ctx_area_fk200: "".into(),
        ctx_area_nk200: "".into(),
    };
    let r = inquire_balance::call(client, &req).await?;
    println!("해외주식 잔고 ({}) — {}", exchange, "USD");
    println!("{}", "═".repeat(80));
    if r.holdings.is_empty() {
        println!("  보유 종목 없음");
    } else {
        println!(
            "{:<8} {:<20} {:>8} {:>12} {:>14} {:>10}",
            "코드", "종목명", "수량", "평균단가", "평가금액", "수익률"
        );
        println!("{}", "─".repeat(80));
        for h in &r.holdings {
            let rate = h.evlu_pfls_rt.parse::<f64>().unwrap_or(0.0);
            println!(
                "{:<8} {:<20} {:>8} {:>12} {:>14} {:>9.2}%",
                h.ovrs_pdno,
                h.ovrs_item_name,
                h.ovrs_cblc_qty,
                format_number(&h.pchs_avg_pric),
                format_number(&h.ovrs_stck_evlu_amt),
                rate,
            );
        }
    }
    if let Some(s) = r.summary {
        println!("{}", "═".repeat(80));
        println!("총 평가손익: {:>15}", format_number(&s.tot_evlu_pfls_amt));
        println!("총 수익률:   {:>15}%", s.tot_pftrt);
    }
    Ok(())
}

pub async fn run_order(
    client: &KisClient,
    side: order::Side,
    symbol: &str,
    qty: u64,
    price_v: f64,
    pick: Option<usize>,
) -> Result<()> {
    if price_v <= 0.0 {
        return Err(anyhow!("해외주식은 시장가 주문 제한 — 지정가 --price 필요"));
    }
    let sym: ResolvedSymbol = resolve_symbol(symbol, ResolveMode::Overseas, pick)?;
    let req = order::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        ovrs_excg_cd: ovrs_excg(sym.market).into(),
        pdno: sym.code.clone(),
        ord_qty: qty.to_string(),
        ovrs_ord_unpr: format!("{:.4}", price_v),
        ord_svr_dvsn_cd: "0".into(),
        ord_dvsn: "00".into(),
    };
    let r = order::call(client, order_market(sym.market), side, &req).await?;
    let side_label = match side { order::Side::Buy => "매수", order::Side::Sell => "매도" };
    println!("{} 주문 접수", side_label);
    println!("  종목:     {} {}", sym.code, sym.name_en);
    println!("  수량:     {}", qty);
    println!("  지정가:   {:.4}", price_v);
    println!("  주문번호: {}", r.odno);
    println!("  주문시각: {}", r.ord_tmd);
    Ok(())
}

pub async fn run_watch(client: &KisClient, symbol: &str, pick: Option<usize>) -> Result<()> {
    let sym = resolve_symbol(symbol, ResolveMode::Overseas, pick)?;
    ws::run_overseas(client.token_manager.clone(), excd(sym.market), &sym.code).await
}

pub async fn run_history(client: &KisClient, exchange: &str) -> Result<()> {
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    let from = (chrono::Local::now() - chrono::Duration::days(30))
        .format("%Y%m%d")
        .to_string();
    let req = inquire_ccnl::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        pdno: "".into(),
        ord_strt_dt: from,
        ord_end_dt: today,
        sll_buy_dvsn: "00".into(),
        ccld_nccs_dvsn: "00".into(),
        ovrs_excg_cd: exchange.into(),
        sort_sqn: "DS".into(),
        ord_dt: "".into(),
        ord_gno_brno: "".into(),
        odno: "".into(),
        ctx_area_nk200: "".into(),
        ctx_area_fk200: "".into(),
    };
    let rows = inquire_ccnl::call(client, &req).await?;
    println!("해외주식 주문/체결 내역 ({}, 최근 30일)", exchange);
    println!(
        "{:<10} {:<12} {:<10} {:<4} {:>8} {:>12} {:<12}",
        "일자", "주문번호", "종목", "구분", "수량", "단가", "상태"
    );
    println!("{}", "─".repeat(80));
    for o in rows.iter().take(30) {
        println!(
            "{:<10} {:<12} {:<10} {:<4} {:>8} {:>12} {:<12}",
            o.ord_dt, o.odno, o.pdno, o.sll_buy_dvsn_cd_name,
            o.ft_ord_qty, format_number(&o.ft_ord_unpr3), o.prcs_stat_name,
        );
    }
    Ok(())
}
