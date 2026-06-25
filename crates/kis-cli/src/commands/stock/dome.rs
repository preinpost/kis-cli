//! 국내주식 커맨드.

use anyhow::{anyhow, Result};

use crate::api::domestic_stock::order_account::{
    inquire_balance, inquire_daily_ccld, inquire_psbl_order, order_cash, order_rvsecncl,
};
use crate::api::domestic_stock::quotations::{
    inquire_asking_price_exp_ccn, inquire_daily_itemchartprice, inquire_price,
};
use crate::client::KisClient;
use crate::commands::helpers::{format_number, resolve_symbol};
use crate::symbols::ResolveMode;
use crate::ws;

pub async fn run_price(client: &KisClient, symbol: &str, pick: Option<usize>) -> Result<()> {
    let sym = resolve_symbol(symbol, ResolveMode::Domestic, pick)?;
    let req = inquire_price::Request {
        fid_cond_mrkt_div_code: "J".into(),
        fid_input_iscd: sym.code.clone(),
    };
    let r = inquire_price::call(client, &req).await?;
    let sign = sign_arrow(&r.prdy_vrss_sign);
    println!("종목: {} [{}] {}", sym.code, sym.name_kr, r.rprs_mrkt_kor_name);
    println!("{}", "─".repeat(40));
    println!("현재가:      {:>14}원", format_number(&r.stck_prpr));
    println!(
        "전일대비:    {:>14}원 ({}{}%)",
        format_number(&r.prdy_vrss),
        sign,
        r.prdy_ctrt
    );
    println!("시가:        {:>14}원", format_number(&r.stck_oprc));
    println!("고가:        {:>14}원", format_number(&r.stck_hgpr));
    println!("저가:        {:>14}원", format_number(&r.stck_lwpr));
    println!("거래량:      {:>14}주", format_number(&r.acml_vol));
    if !r.hts_avls.is_empty() {
        println!("시가총액:    {:>14}억", format_number(&r.hts_avls));
    }
    if !r.per.is_empty() { println!("PER:         {:>14}", r.per); }
    if !r.pbr.is_empty() { println!("PBR:         {:>14}", r.pbr); }
    if !r.w52_hgpr.is_empty() {
        println!("52주최고:    {:>14}원", format_number(&r.w52_hgpr));
        println!("52주최저:    {:>14}원", format_number(&r.w52_lwpr));
    }
    Ok(())
}

pub async fn run_chart(
    client: &KisClient,
    symbol: &str,
    period: char,
    pick: Option<usize>,
) -> Result<()> {
    let sym = resolve_symbol(symbol, ResolveMode::Domestic, pick)?;
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    // 약 100일 전
    let from = (chrono::Local::now() - chrono::Duration::days(150))
        .format("%Y%m%d")
        .to_string();
    let req = inquire_daily_itemchartprice::Request {
        fid_cond_mrkt_div_code: "J".into(),
        fid_input_iscd: sym.code.clone(),
        fid_input_date_1: from,
        fid_input_date_2: today,
        fid_period_div_code: period.to_string(),
        fid_org_adj_prc: "0".into(),
    };
    let r = inquire_daily_itemchartprice::call(client, &req).await?;
    println!("[{}] {} — {}봉", sym.code, sym.name_kr, match period {
        'D' => "일", 'W' => "주", 'M' => "월", _ => "?",
    });
    println!("{:<10} {:>12} {:>12} {:>12} {:>12} {:>14}",
        "일자", "시가", "고가", "저가", "종가", "거래량");
    println!("{}", "─".repeat(80));
    for c in r.candles.iter().take(30) {
        println!(
            "{:<10} {:>12} {:>12} {:>12} {:>12} {:>14}",
            c.stck_bsop_date,
            format_number(&c.stck_oprc),
            format_number(&c.stck_hgpr),
            format_number(&c.stck_lwpr),
            format_number(&c.stck_clpr),
            format_number(&c.acml_vol),
        );
    }
    Ok(())
}

pub async fn run_asking(client: &KisClient, symbol: &str, pick: Option<usize>) -> Result<()> {
    let sym = resolve_symbol(symbol, ResolveMode::Domestic, pick)?;
    let req = inquire_asking_price_exp_ccn::Request {
        fid_cond_mrkt_div_code: "J".into(),
        fid_input_iscd: sym.code.clone(),
    };
    let r = inquire_asking_price_exp_ccn::call(client, &req).await?;
    let a = r.asking.ok_or_else(|| anyhow!("호가 데이터 없음"))?;
    println!("[{}] {} — 호가", sym.code, sym.name_kr);
    println!("{:<10}  {:<12}  │  {:<12}  {:<10}", "매도잔량", "매도호가", "매수호가", "매수잔량");
    println!("{}", "─".repeat(60));
    let asks = [
        (&a.askp1, &a.askp_rsqn1), (&a.askp2, &a.askp_rsqn2),
        (&a.askp3, &a.askp_rsqn3), (&a.askp4, &a.askp_rsqn4),
        (&a.askp5, &a.askp_rsqn5),
    ];
    let bids = [
        (&a.bidp1, &a.bidp_rsqn1), (&a.bidp2, &a.bidp_rsqn2),
        (&a.bidp3, &a.bidp_rsqn3), (&a.bidp4, &a.bidp_rsqn4),
        (&a.bidp5, &a.bidp_rsqn5),
    ];
    for i in 0..5 {
        let (ap, aq) = asks[4 - i];
        let (bp, bq) = bids[i];
        println!(
            "{:>10}  {:>12}  │  {:>12}  {:>10}",
            format_number(aq), format_number(ap),
            format_number(bp), format_number(bq),
        );
    }
    Ok(())
}

pub async fn run_balance(client: &KisClient) -> Result<()> {
    let req = inquire_balance::Request {
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
    let r = inquire_balance::call(client, &req).await?;

    println!("국내주식 잔고 ({}-{})", client.cano(), client.product_code());
    println!("{}", "═".repeat(90));
    if r.holdings.is_empty() {
        println!("  보유 종목 없음");
    } else {
        println!(
            "{:<8} {:<14} {:>6} {:>12} {:>14} {:>14} {:>8}",
            "코드", "종목명", "수량", "평균단가", "평가금액", "손익", "수익률"
        );
        println!("{}", "─".repeat(90));
        for h in &r.holdings {
            let rate = h.evlu_pfls_rt.parse::<f64>().unwrap_or(0.0);
            let sign = if rate >= 0.0 { "+" } else { "" };
            println!(
                "{:<8} {:<14} {:>6} {:>12} {:>14} {:>14} {:>7.2}%",
                h.pdno, h.prdt_name, h.hldg_qty,
                format_number(&h.pchs_avg_pric),
                format_number(&h.evlu_amt),
                format!("{sign}{}", format_number(&h.evlu_pfls_amt)),
                rate,
            );
        }
    }
    if let Some(s) = r.summary {
        println!("{}", "═".repeat(90));
        println!("예수금:      {:>15}원", format_number(&s.dnca_tot_amt));
        println!("매입합계:    {:>15}원", format_number(&s.pchs_amt_smtl_amt));
        println!("평가합계:    {:>15}원", format_number(&s.evlu_amt_smtl_amt));
        println!("손익합계:    {:>15}원", format_number(&s.evlu_pfls_smtl_amt));
        println!("총평가금액:  {:>15}원", format_number(&s.tot_evlu_amt));
    }
    Ok(())
}

pub async fn run_psbl(client: &KisClient) -> Result<()> {
    let req = inquire_psbl_order::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        pdno: "".into(),
        ord_unpr: "".into(),
        ord_dvsn: "01".into(),
        cma_evlu_amt_icld_yn: "N".into(),
        ovrs_icld_yn: "N".into(),
    };
    let r = inquire_psbl_order::call(client, &req).await?;
    println!("매수가능");
    println!("{}", "─".repeat(40));
    println!("주문가능현금:  {:>15}원", format_number(&r.ord_psbl_cash));
    println!("주문가능대용:  {:>15}원", format_number(&r.ord_psbl_sbst));
    println!("재사용가능액:  {:>15}원", format_number(&r.ruse_psbl_amt));
    Ok(())
}

pub async fn run_order(
    client: &KisClient,
    side: order_cash::Side,
    symbol: &str,
    qty: u64,
    price: Option<u64>,
    pick: Option<usize>,
) -> Result<()> {
    let sym = resolve_symbol(symbol, ResolveMode::Domestic, pick)?;
    let (ord_dvsn, ord_unpr) = match price {
        Some(p) => ("00".to_string(), p.to_string()),    // 지정가
        None => ("01".to_string(), "0".to_string()),     // 시장가
    };
    let req = order_cash::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        pdno: sym.code.clone(),
        sll_type: match side {
            order_cash::Side::Sell => Some("01".into()),
            order_cash::Side::Buy => None,
        },
        ord_dvsn,
        ord_qty: qty.to_string(),
        ord_unpr,
        cndt_pric: None,
        excg_id_dvsn_cd: None,
    };
    let r = order_cash::call(client, side, &req).await?;
    let side_label = match side { order_cash::Side::Buy => "매수", order_cash::Side::Sell => "매도" };
    println!("{} 주문 접수 완료", side_label);
    println!("  종목:     {} {}", sym.code, sym.name_kr);
    println!("  수량:     {}", qty);
    match price {
        Some(p) => println!("  지정가:   {}원", format_number(&p.to_string())),
        None => println!("  시장가"),
    }
    println!("  주문번호: {}", r.odno);
    println!("  주문시각: {}", r.ord_tmd);
    Ok(())
}

pub async fn run_order_cancel(client: &KisClient, order_no: &str, qty: u64) -> Result<()> {
    let req = order_rvsecncl::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        krx_fwdg_ord_orgno: "".into(),
        orgn_odno: order_no.into(),
        ord_dvsn: "00".into(),
        rvse_cncl_dvsn_cd: "02".into(), // 02: 취소
        ord_qty: qty.to_string(),
        ord_unpr: "0".into(),
        qty_all_ord_yn: if qty == 0 { "Y".into() } else { "N".into() },
        cndt_pric: None,
        excg_id_dvsn_cd: None,
    };
    let r = order_rvsecncl::call(client, &req).await?;
    println!("취소 주문 접수 완료");
    println!("  원주문번호: {}", order_no);
    println!("  주문번호:   {}", r.odno);
    println!("  주문시각:   {}", r.ord_tmd);
    Ok(())
}

pub async fn run_history(client: &KisClient) -> Result<()> {
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    let from = (chrono::Local::now() - chrono::Duration::days(30))
        .format("%Y%m%d")
        .to_string();
    let req = inquire_daily_ccld::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        inqr_strt_dt: from,
        inqr_end_dt: today,
        sll_buy_dvsn_cd: "00".into(),
        pdno: "".into(),
        ord_gno_brno: "".into(),
        odno: "".into(),
        ccld_dvsn: "00".into(),
        inqr_dvsn: "00".into(),
        inqr_dvsn_1: "".into(),
        inqr_dvsn_3: "00".into(),
        excg_id_dvsn_cd: "".into(),
        ctx_area_fk100: "".into(),
        ctx_area_nk100: "".into(),
    };
    let r = inquire_daily_ccld::call(client, inquire_daily_ccld::Period::Within3Months, &req).await?;
    println!("최근 주문/체결 내역 (최대 30건)");
    println!(
        "{:<10} {:<12} {:<10} {:<4} {:>8} {:>12} {:<12}",
        "일자", "주문번호", "종목", "구분", "수량", "단가", "상태"
    );
    println!("{}", "─".repeat(80));
    for o in r.rows.iter().take(30) {
        println!(
            "{:<10} {:<12} {:<10} {:<4} {:>8} {:>12} {:<12}",
            o.ord_dt, o.odno, o.pdno, o.sll_buy_dvsn_cd_name,
            o.ord_qty, format_number(&o.ord_unpr), o.ord_dvsn_name,
        );
    }
    Ok(())
}

pub async fn run_watch(client: &KisClient, symbol: &str, pick: Option<usize>) -> Result<()> {
    let sym = resolve_symbol(symbol, ResolveMode::Domestic, pick)?;
    ws::run_domestic(client.token_manager.clone(), &sym.code).await
}

fn sign_arrow(code: &str) -> &'static str {
    match code {
        "1" | "2" => "▲",
        "4" | "5" => "▼",
        _ => " ",
    }
}
