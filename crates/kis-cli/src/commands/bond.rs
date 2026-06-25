//! 장내채권 커맨드. (모의투자 미지원 — 실전 전용)

use anyhow::Result;

use crate::api::bond::order_account::{buy, inquire_balance, sell};
use crate::api::bond::quotations::{inquire_daily_itemchartprice, inquire_price};
use crate::client::KisClient;
use crate::commands::helpers::format_number;

const MARKET: &str = "B";

pub async fn run_price(client: &KisClient, symbol: &str) -> Result<()> {
    let req = inquire_price::Request {
        market: MARKET.into(),
        symbol: symbol.into(),
    };
    let r = inquire_price::call(client, &req).await?;
    println!("채권: {} [{}]", symbol, r.hts_kor_isnm);
    println!("{}", "─".repeat(40));
    println!("현재가:   {:>12}", format_number(&r.bond_prpr));
    println!("전일대비: {:>12} ({}%)", format_number(&r.bond_prdy_vrss), r.prdy_ctrt);
    println!("시가/고/저: {} / {} / {}",
        format_number(&r.bond_oprc),
        format_number(&r.bond_hgpr),
        format_number(&r.bond_lwpr));
    println!("수익률:   {:>12}%", r.ernn_rate);
    println!("거래량:   {:>12}", format_number(&r.acml_vol));
    Ok(())
}

pub async fn run_chart(client: &KisClient, symbol: &str) -> Result<()> {
    let req = inquire_daily_itemchartprice::Request {
        market: MARKET.into(),
        symbol: symbol.into(),
    };
    let r = inquire_daily_itemchartprice::call(client, &req).await?;
    println!("[{}] 채권 일봉", symbol);
    println!("{:<10} {:>12} {:>12} {:>12} {:>12} {:>14}",
        "일자", "시가", "고가", "저가", "종가", "거래량");
    println!("{}", "─".repeat(80));
    for row in r.rows.iter().take(30) {
        println!(
            "{:<10} {:>12} {:>12} {:>12} {:>12} {:>14}",
            row.stck_bsop_date,
            format_number(&row.bond_oprc),
            format_number(&row.bond_hgpr),
            format_number(&row.bond_lwpr),
            format_number(&row.bond_prpr),
            format_number(&row.acml_vol),
        );
    }
    Ok(())
}

pub async fn run_balance(client: &KisClient) -> Result<()> {
    let req = inquire_balance::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        inqr_cndt: "00".into(),
        pdno: "".into(),
        buy_dt: "".into(),
        ctx_area_fk200: "".into(),
        ctx_area_nk200: "".into(),
    };
    let r = inquire_balance::call(client, &req).await?;
    println!("장내채권 잔고");
    println!("{}", "═".repeat(100));
    if r.holdings.is_empty() {
        println!("  보유 종목 없음");
        return Ok(());
    }
    println!("{:<14} {:<20} {:<10} {:>10} {:>14} {:>10}",
        "코드", "종목명", "매수일", "잔량", "매수금액", "수익률");
    println!("{}", "─".repeat(100));
    for h in &r.holdings {
        println!(
            "{:<14} {:<20} {:<10} {:>10} {:>14} {:>9}%",
            h.pdno, h.prdt_name, h.buy_dt, h.cblc_qty,
            format_number(&h.buy_amt), h.buy_erng_rt,
        );
    }
    Ok(())
}

pub async fn run_buy(client: &KisClient, symbol: &str, qty: u64, price: f64) -> Result<()> {
    let req = buy::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        pdno: symbol.into(),
        ord_qty2: qty.to_string(),
        bond_ord_unpr: format!("{:.4}", price),
        samt_mket_ptci_yn: "N".into(),
        bond_rtl_mket_yn: "N".into(),
        idcr_stfno: "".into(),
        mgco_aptm_odno: "".into(),
        ord_svr_dvsn_cd: "0".into(),
        ctac_tlno: "".into(),
    };
    let r = buy::call(client, &req).await?;
    println!("채권 매수 주문 접수");
    println!("  종목:     {}", symbol);
    println!("  수량:     {}", qty);
    println!("  지정가:   {:.4}", price);
    println!("  주문번호: {}", r.odno);
    Ok(())
}

pub async fn run_sell(client: &KisClient, symbol: &str, qty: u64, price: f64) -> Result<()> {
    let req = sell::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        ord_dvsn: "00".into(),
        pdno: symbol.into(),
        ord_qty2: qty.to_string(),
        bond_ord_unpr: format!("{:.4}", price),
        sprx_yn: "N".into(),
        buy_dt: "".into(),
        buy_seq: "".into(),
        samt_mket_ptci_yn: "N".into(),
        sll_agco_opps_sll_yn: "N".into(),
        bond_rtl_mket_yn: "N".into(),
        mgco_aptm_odno: "".into(),
        ord_svr_dvsn_cd: "0".into(),
        ctac_tlno: "".into(),
    };
    let r = sell::call(client, &req).await?;
    println!("채권 매도 주문 접수");
    println!("  종목:     {}", symbol);
    println!("  수량:     {}", qty);
    println!("  지정가:   {:.4}", price);
    println!("  주문번호: {}", r.odno);
    Ok(())
}
