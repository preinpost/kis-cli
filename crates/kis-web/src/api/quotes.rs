//! 현재가 조회 — 국내/해외 자동 판별 후 해당 시세 API 호출.

use poem::http::StatusCode;
use poem::web::Data;
use poem::{Error, Result};
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};

use kis_core::api::domestic_stock::quotations::inquire_daily_itemchartprice as dome_chart;
use kis_core::api::domestic_stock::quotations::inquire_price as dome_price;
use kis_core::api::overseas_stock::quotations::dailyprice as os_chart;
use kis_core::api::overseas_stock::quotations::price as os_price;
use kis_core::client::KisClient;

use crate::auth::SessionAuth;
use crate::state::AppState;
use crate::symbols::{self, SymbolInfo};

use super::ApiTag;

#[derive(Object)]
struct Quote {
    symbol: String,
    name: String,
    market: String,   // "domestic" | "overseas"
    currency: String, // "KRW" | "USD"
    price: String,
    change: String,      // 전일대비
    change_rate: String, // 전일대비율(%)
    sign: String,        // 1상한 2상승 3보합 4하한 5하락
    open: String,
    high: String,
    low: String,
    volume: String,
}

/// 미니 차트(스파크라인)용 종가 시계열 (오래된→최신).
#[derive(Object)]
struct Spark {
    symbol: String,
    points: Vec<f64>,
    /// 구간 등락 (마지막 >= 처음)
    up: bool,
}

const SPARK_DAYS: usize = 30;

pub struct QuotesApi;

#[OpenApi(prefix_path = "/quotes", tag = "ApiTag::Quotes")]
impl QuotesApi {
    /// 종목 현재가. symbol 은 국내 코드(005930) 또는 해외 티커(AAPL).
    #[oai(path = "/:symbol", method = "get")]
    async fn quote(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
        symbol: Path<String>,
    ) -> Result<Json<Quote>> {
        let st = state.0;
        let client = st
            .clients
            .get(&st.db, &st.config.master_key, &auth.0.id)
            .await
            .map_err(internal)?
            .ok_or_else(|| Error::from_string("KIS 자격증명을 먼저 등록하세요", StatusCode::CONFLICT))?;

        let info = symbols::resolve(st.config.symbols_db_path.clone(), symbol.0.clone()).await;

        let quote = match info.kind {
            "overseas" => fetch_overseas(&client, &info).await.map_err(upstream)?,
            "domestic" => fetch_domestic(&client, &info).await.map_err(upstream)?,
            _ => {
                return Err(Error::from_string(
                    "지원하지 않는 종목 유형입니다",
                    StatusCode::BAD_REQUEST,
                ));
            }
        };

        Ok(Json(quote))
    }

    /// 미니 차트용 일봉 종가 시계열 (최근 ~30거래일, 오래된→최신).
    #[oai(path = "/:symbol/spark", method = "get")]
    async fn spark(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
        symbol: Path<String>,
    ) -> Result<Json<Spark>> {
        let st = state.0;
        let client = st
            .clients
            .get(&st.db, &st.config.master_key, &auth.0.id)
            .await
            .map_err(internal)?
            .ok_or_else(|| Error::from_string("KIS 자격증명을 먼저 등록하세요", StatusCode::CONFLICT))?;

        let info = symbols::resolve(st.config.symbols_db_path.clone(), symbol.0.clone()).await;

        // 캐시 히트 시 KIS 호출 생략 (일봉은 장중 거의 안 변함)
        if let Some((points, up)) = st.spark_get(&info.code) {
            return Ok(Json(Spark { symbol: info.code, points, up }));
        }

        let points = match info.kind {
            "domestic" => fetch_spark_domestic(&client, &info).await.map_err(upstream)?,
            "overseas" => fetch_spark_overseas(&client, &info).await.map_err(upstream)?,
            _ => return Err(Error::from_string("지원하지 않는 종목 유형입니다", StatusCode::BAD_REQUEST)),
        };
        let up = points.first().zip(points.last()).map(|(a, b)| b >= a).unwrap_or(true);
        st.spark_put(&info.code, points.clone(), up);
        Ok(Json(Spark { symbol: info.code, points, up }))
    }
}

/// 국내 일봉 종가 (오래된→최신, 최대 SPARK_DAYS).
async fn fetch_spark_domestic(client: &KisClient, info: &SymbolInfo) -> anyhow::Result<Vec<f64>> {
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    let from = (chrono::Local::now() - chrono::Duration::days(60))
        .format("%Y%m%d")
        .to_string();
    let req = dome_chart::Request {
        fid_cond_mrkt_div_code: "J".into(),
        fid_input_iscd: info.code.clone(),
        fid_input_date_1: from,
        fid_input_date_2: today,
        fid_period_div_code: "D".into(),
        fid_org_adj_prc: "0".into(),
    };
    let mut r = dome_chart::call(client, &req).await?;
    r.candles.reverse(); // 최신→오래된 → 오래된→최신
    Ok(closes(r.candles.iter().map(|c| c.stck_clpr.as_str())))
}

/// 해외 일봉 종가 (오래된→최신).
async fn fetch_spark_overseas(client: &KisClient, info: &SymbolInfo) -> anyhow::Result<Vec<f64>> {
    let excd = if info.excd.is_empty() { "NAS" } else { info.excd };
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    let req = os_chart::Request {
        auth: "".into(),
        excd: excd.into(),
        symb: info.code.clone(),
        gubn: "0".into(), // 0 일봉
        bymd: today,      // 기준일(최신)
        modp: "1".into(), // 수정주가
    };
    let mut r = os_chart::call(client, &req).await?;
    r.bars.reverse();
    Ok(closes(r.bars.iter().map(|b| b.clos.as_str())))
}

/// 종가 문자열 시퀀스 → f64 (0/공백 제외), 마지막 SPARK_DAYS개만.
fn closes<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<f64> {
    let all: Vec<f64> = iter
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .filter(|v| *v > 0.0)
        .collect();
    let start = all.len().saturating_sub(SPARK_DAYS);
    all[start..].to_vec()
}

async fn fetch_domestic(client: &KisClient, info: &SymbolInfo) -> anyhow::Result<Quote> {
    let req = dome_price::Request {
        fid_cond_mrkt_div_code: "J".into(),
        fid_input_iscd: info.code.clone(),
    };
    let r = dome_price::call(client, &req).await?;
    Ok(Quote {
        symbol: info.code.clone(),
        name: if info.name.is_empty() { info.code.clone() } else { info.name.clone() },
        market: "domestic".into(),
        currency: "KRW".into(),
        price: r.stck_prpr,
        change: apply_sign(&r.prdy_vrss, &r.prdy_vrss_sign),
        change_rate: apply_sign(&r.prdy_ctrt, &r.prdy_vrss_sign),
        sign: r.prdy_vrss_sign,
        open: r.stck_oprc,
        high: r.stck_hgpr,
        low: r.stck_lwpr,
        volume: r.acml_vol,
    })
}

async fn fetch_overseas(client: &KisClient, info: &SymbolInfo) -> anyhow::Result<Quote> {
    let excd = if info.excd.is_empty() { "NAS" } else { info.excd };
    let req = os_price::Request {
        auth: "".into(),
        excd: excd.into(),
        symb: info.code.clone(),
    };
    let r = os_price::call(client, &req).await?;
    Ok(Quote {
        symbol: info.code.clone(),
        name: if info.name.is_empty() { info.code.clone() } else { info.name.clone() },
        market: "overseas".into(),
        currency: "USD".into(),
        price: r.last,
        change: apply_sign(&r.diff, &r.sign),
        change_rate: apply_sign(&r.rate, &r.sign),
        sign: r.sign,
        open: r.base,
        high: String::new(), // 해외 현재가 API 응답엔 고/저가 없음
        low: String::new(),
        volume: r.tvol,
    })
}

/// KIS 전일대비부호(1상한·2상승·3보합·4하한·5하락)를 절대값 크기에 적용해 부호 있는 문자열로.
/// KIS 가 부호를 어떻게 주든(절대값/이미부호) 일관되게 정규화한다.
fn apply_sign(magnitude: &str, sign_code: &str) -> String {
    let m = magnitude.trim().trim_start_matches(['+', '-']);
    if m.is_empty() || m == "0" {
        return m.to_string();
    }
    match sign_code.trim() {
        "4" | "5" => format!("-{m}"), // 하한·하락
        _ => m.to_string(),           // 1상한·2상승·3보합(또는 미상)
    }
}

/// 포트폴리오용 — 종목의 현재가 + 통화만. resolve + 시세 호출, 실패/미해석 시 None.
pub(crate) async fn current_price(
    client: &KisClient,
    symbols_db_path: String,
    symbol: &str,
) -> Option<(f64, String)> {
    let info = symbols::resolve(symbols_db_path, symbol.to_string()).await;
    match info.kind {
        "domestic" => {
            let q = fetch_domestic(client, &info).await.ok()?;
            Some((q.price.trim().parse().ok()?, "KRW".to_string()))
        }
        "overseas" => {
            let q = fetch_overseas(client, &info).await.ok()?;
            Some((q.price.trim().parse().ok()?, "USD".to_string()))
        }
        _ => None,
    }
}

fn internal<E: std::fmt::Display>(e: E) -> Error {
    tracing::error!("quotes internal error: {e}");
    Error::from_string("서버 오류", StatusCode::INTERNAL_SERVER_ERROR)
}

fn upstream(e: impl std::fmt::Display) -> Error {
    Error::from_string(format!("KIS 시세 조회 실패: {e}"), StatusCode::BAD_GATEWAY)
}
