//! 장내채권현재가(호가) — GET /uapi/domestic-bond/v1/quotations/inquire-asking-price
//!
//! 스펙: .agent/specs/bond__quotations__inquire_asking_price.md
//! 모의투자 미지원.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-bond/v1/quotations/inquire-asking-price";
pub const TR_ID: &str = "FHKBJ773401C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub market: String,
    pub symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub aspr_acpt_hour: String,
    #[serde(default)]
    pub bond_askp1: String,
    #[serde(default)]
    pub bond_askp2: String,
    #[serde(default)]
    pub bond_askp3: String,
    #[serde(default)]
    pub bond_askp4: String,
    #[serde(default)]
    pub bond_askp5: String,
    #[serde(default)]
    pub bond_bidp1: String,
    #[serde(default)]
    pub bond_bidp2: String,
    #[serde(default)]
    pub bond_bidp3: String,
    #[serde(default)]
    pub bond_bidp4: String,
    #[serde(default)]
    pub bond_bidp5: String,
    #[serde(default)]
    pub askp_rsqn1: String,
    #[serde(default)]
    pub askp_rsqn2: String,
    #[serde(default)]
    pub askp_rsqn3: String,
    #[serde(default)]
    pub askp_rsqn4: String,
    #[serde(default)]
    pub askp_rsqn5: String,
    #[serde(default)]
    pub bidp_rsqn1: String,
    #[serde(default)]
    pub bidp_rsqn2: String,
    #[serde(default)]
    pub bidp_rsqn3: String,
    #[serde(default)]
    pub bidp_rsqn4: String,
    #[serde(default)]
    pub bidp_rsqn5: String,
    #[serde(default)]
    pub total_askp_rsqn: String,
    #[serde(default)]
    pub total_bidp_rsqn: String,
    #[serde(default)]
    pub ntby_aspr_rsqn: String,
    #[serde(default)]
    pub seln_ernn_rate1: String,
    #[serde(default)]
    pub seln_ernn_rate2: String,
    #[serde(default)]
    pub seln_ernn_rate3: String,
    #[serde(default)]
    pub seln_ernn_rate4: String,
    #[serde(default)]
    pub seln_ernn_rate5: String,
    #[serde(default)]
    pub shnu_ernn_rate1: String,
    #[serde(default)]
    pub shnu_ernn_rate2: String,
    #[serde(default)]
    pub shnu_ernn_rate3: String,
    #[serde(default)]
    pub shnu_ernn_rate4: String,
    #[serde(default)]
    pub shnu_ernn_rate5: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("장내채권현재가(호가)는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.market.as_str()),
        ("FID_INPUT_ISCD", req.symbol.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.context("응답에 output 없음")?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
