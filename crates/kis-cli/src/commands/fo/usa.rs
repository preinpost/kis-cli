//! 해외선물옵션 커맨드. (모의투자 미지원)

use anyhow::Result;

use crate::api::futureoption_overseas::order_account::{inquire_deposit, order};
use crate::api::futureoption_overseas::quotations::inquire_price;
use crate::client::KisClient;
use crate::commands::helpers::format_number;

pub async fn run_price(client: &KisClient, symbol: &str) -> Result<()> {
    let req = inquire_price::Request { srs_cd: symbol.into() };
    let r = inquire_price::call(client, &req).await?;
    println!("해외선물옵션: {}", symbol);
    println!("{}", "─".repeat(40));
    println!("현재가:   {:>14}", r.last_price);
    println!("시가/고/저: {} / {} / {}", r.open_price, r.high_price, r.low_price);
    println!("전일대비: {} ({}%)", r.prev_diff_price, r.prev_diff_rate);
    println!("거래량:   {:>14}", format_number(&r.vol));
    println!("매도/매수호가: {} / {}", r.ask_price, r.bid_price);
    Ok(())
}

pub async fn run_balance(client: &KisClient, currency: &str) -> Result<()> {
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    let req = inquire_deposit::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        crcy_cd: currency.into(),
        inqr_dt: today,
    };
    let r = inquire_deposit::call(client, &req).await?;
    println!("해외선물옵션 예수금 ({})", currency);
    println!("{}", "─".repeat(50));
    println!("총평가자산: {:>15}", format_number(&r.fm_tot_asst_evlu_amt));
    println!("예수금잔액: {:>15}", format_number(&r.fm_dnca_rmnd));
    println!("주문가능:   {:>15}", format_number(&r.fm_ord_psbl_amt));
    println!("인출가능:   {:>15}", format_number(&r.fm_drwg_psbl_amt));
    println!("평가손익:   {:>15}", format_number(&r.fm_fuop_evlu_pfls_amt));
    println!("유지증거금: {:>15}", format_number(&r.fm_mntn_mgn_amt));
    println!("리스크율:   {:>15}%", r.fm_risk_rt);
    Ok(())
}

pub async fn run_order(
    client: &KisClient,
    side: Side,
    symbol: &str,
    qty: u64,
    price: f64,
) -> Result<()> {
    let req = order::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        ovrs_futr_fx_pdno: symbol.into(),
        sll_buy_dvsn_cd: match side { Side::Buy => "02".into(), Side::Sell => "01".into() },
        pric_dvsn_cd: "1".into(),          // 지정가
        fm_limit_ord_pric: format!("{}", price),
        fm_stop_ord_pric: "0".into(),
        fm_ord_qty: qty.to_string(),
        ccld_cndt_cd: "6".into(),           // 일반
        cplx_ord_dvsn_cd: "0".into(),
        ecis_rsvn_ord_yn: "N".into(),
        fm_hdge_ord_scrn_yn: "N".into(),
        fm_lqd_ustl_ccld_dt: None,
        fm_lqd_ustl_ccno: None,
        fm_lqd_lmt_ord_pric: None,
        fm_lqd_stop_ord_pric: None,
    };
    let r = order::call(client, &req).await?;
    let label = match side { Side::Buy => "매수", Side::Sell => "매도" };
    println!("{} 주문 접수", label);
    println!("  종목:     {}", symbol);
    println!("  수량:     {}", qty);
    println!("  지정가:   {}", price);
    println!("  주문번호: {}", r.odno);
    println!("  주문일자: {}", r.ord_dt);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum Side { Buy, Sell }
