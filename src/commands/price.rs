use anyhow::{Context, Result};

use crate::client::KisClient;
use crate::models::DomesticStockPrice;

const TR_ID_DOMESTIC_PRICE: &str = "FHKST01010100";

pub async fn run(client: &KisClient, symbol: &str) -> Result<()> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", "J"),
        ("FID_INPUT_ISCD", symbol),
    ];

    let resp = client
        .get(
            "/uapi/domestic-stock/v1/quotations/inquire-price",
            TR_ID_DOMESTIC_PRICE,
            &params,
        )
        .await?;

    let output = resp.output.context("응답에 output 없음")?;
    let price: DomesticStockPrice = serde_json::from_value(output)?;

    let sign = match price.prdy_vrss_sign.as_str() {
        "1" => "▲",  // 상한
        "2" => "▲",  // 상승
        "3" => "-",  // 보합
        "4" => "▼",  // 하한
        "5" => "▼",  // 하락
        _ => " ",
    };

    println!("종목: {} [{}]", symbol, price.rprs_mrkt_kor_name);
    println!("─────────────────────────────");
    println!("현재가:      {:>12}원", format_number(&price.stck_prpr));
    println!(
        "전일대비:    {:>12}원 ({}{:.2}%)",
        format_number(&price.prdy_vrss),
        sign,
        price.prdy_ctrt.parse::<f64>().unwrap_or(0.0).abs()
    );
    println!("시가:        {:>12}원", format_number(&price.stck_oprc));
    println!("고가:        {:>12}원", format_number(&price.stck_hgpr));
    println!("저가:        {:>12}원", format_number(&price.stck_lwpr));
    println!("거래량:      {:>12}주", format_number(&price.acml_vol));

    if !price.hts_avls.is_empty() {
        println!("시가총액:    {:>12}억", format_number(&price.hts_avls));
    }
    if !price.per.is_empty() {
        println!("PER:         {:>12}", price.per);
    }
    if !price.pbr.is_empty() {
        println!("PBR:         {:>12}", price.pbr);
    }
    if !price.w52_hgpr.is_empty() {
        println!("52주 최고:   {:>12}원", format_number(&price.w52_hgpr));
        println!("52주 최저:   {:>12}원", format_number(&price.w52_lwpr));
    }

    Ok(())
}

fn format_number(s: &str) -> String {
    let s = s.trim().trim_start_matches('-');
    let parts: Vec<&str> = s.split('.').collect();
    let integer = parts[0];

    let formatted: String = integer
        .chars()
        .rev()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i > 0 && i % 3 == 0 {
                acc.push(',');
            }
            acc.push(c);
            acc
        })
        .chars()
        .rev()
        .collect();

    if parts.len() > 1 {
        format!("{}.{}", formatted, parts[1])
    } else {
        formatted
    }
}
