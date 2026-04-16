use anyhow::{Context, Result};

use crate::client::KisClient;
use crate::models::{AccountHolding, AccountSummary};

const TR_ID_ACCOUNT_BALANCE: &str = "TTTC8434R";

pub async fn run(client: &KisClient) -> Result<()> {
    let cano = client.cano().to_string();
    let product_code = client.product_code().to_string();

    let params = [
        ("CANO", cano.as_str()),
        ("ACNT_PRDT_CD", product_code.as_str()),
        ("AFHR_FLPR_YN", "N"),
        ("OFL_YN", ""),
        ("INQR_DVSN", "02"),
        ("UNPR_DVSN", "01"),
        ("FUND_STTL_ICLD_YN", "N"),
        ("FNCG_AMT_AUTO_RDPT_YN", "N"),
        ("PRCS_DVSN", "01"),
        ("CTX_AREA_FK100", ""),
        ("CTX_AREA_NK100", ""),
    ];

    let resp = client
        .get(
            "/uapi/domestic-stock/v1/trading/inquire-balance",
            TR_ID_ACCOUNT_BALANCE,
            &params,
        )
        .await?;

    // 보유종목
    let holdings: Vec<AccountHolding> = if let Some(output1) = resp.output1 {
        serde_json::from_value(output1).unwrap_or_default()
    } else {
        vec![]
    };

    // 요약
    let summary: AccountSummary = if let Some(output2) = resp.output2 {
        let arr: Vec<AccountSummary> = serde_json::from_value(output2).unwrap_or_default();
        arr.into_iter().next().context("계좌 요약 데이터 없음")?
    } else {
        anyhow::bail!("계좌 요약 데이터가 없습니다");
    };

    // 출력
    println!("계좌잔고 ({}-{})", cano, product_code);
    println!("═══════════════════════════════════════════════════════════════════");

    if holdings.is_empty() {
        println!("  보유 종목 없음");
    } else {
        println!(
            "{:<8} {:<12} {:>8} {:>12} {:>12} {:>12} {:>8}",
            "종목코드", "종목명", "수량", "매입평균", "평가금액", "손익금액", "수익률"
        );
        println!("───────────────────────────────────────────────────────────────────");
        for h in &holdings {
            let pnl_rate = h.evlu_pfls_rt.parse::<f64>().unwrap_or(0.0);
            let sign = if pnl_rate >= 0.0 { "+" } else { "" };
            println!(
                "{:<8} {:<12} {:>8} {:>12} {:>12} {:>12} {:>7.2}%",
                h.pdno,
                h.prdt_name,
                h.hldg_qty,
                h.pchs_avg_pric,
                h.evlu_amt,
                format!("{sign}{}", h.evlu_pfls_amt),
                pnl_rate,
            );
        }
    }

    println!("═══════════════════════════════════════════════════════════════════");
    println!("예수금:      {:>15}원", summary.dnca_tot_amt);
    println!("매입합계:    {:>15}원", summary.pchs_amt_smtl_amt);
    println!("평가합계:    {:>15}원", summary.evlu_amt_smtl_amt);
    println!("손익합계:    {:>15}원", summary.evlu_pfls_smtl_amt);
    println!("총평가금액:  {:>15}원", summary.tot_evlu_amt);

    Ok(())
}
