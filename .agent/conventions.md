# kis-cli Rust 코딩 규약

## 리프 모듈 템플릿 (GET, 단일 output)

```rust
//! <한글 API 이름> — <endpoint 경로>
//!
//! 스펙: .agent/specs/<slug>.md

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const TR_ID: &str = "FHKST01010100"; // 실전/모의 동일. 다르면 아래 참고.
pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-price";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// FID_COND_MRKT_DIV_CODE — 조건 시장 분류 코드 (예: J=KRX)
    pub market: String,
    /// FID_INPUT_ISCD — 종목코드
    pub symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub rprs_mrkt_kor_name: String,
    #[serde(default)]
    pub stck_prpr: String,
    // ... 스펙에 있는 모든 output 필드 (String + #[serde(default)])
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.market.as_str()),
        ("FID_INPUT_ISCD", req.symbol.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.context("응답에 output 없음")?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
```

## 리프 모듈 템플릿 (POST JSON)

```rust
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const TR_ID_REAL: &str = "TTTC0802U"; // 실전
pub const TR_ID_MOCK: &str = "VTTC0802U"; // 모의
pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/order-cash";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "PDNO")]
    pub pdno: String,
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default, rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: String,
    #[serde(default, rename = "ODNO")]
    pub odno: String,
    #[serde(default, rename = "ORD_TMD")]
    pub ord_tmd: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let tr_id = if client.is_mock() { TR_ID_MOCK } else { TR_ID_REAL };
    let resp: ApiResponse = client.post_json(ENDPOINT, tr_id, req, &[]).await?;
    let output = resp.output.ok_or_else(|| anyhow::anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
```

## 다중 output (output1/output2)

스펙에 output이 여러 개면 (예: 잔고조회 = 보유종목 목록 + 요약):

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Holding { /* output1의 row */ }

#[derive(Debug, Clone, Deserialize)]
pub struct Summary { /* output2의 단일 객체 */ }

#[derive(Debug, Clone)]
pub struct Response {
    pub holdings: Vec<Holding>,
    pub summary: Option<Summary>,
    pub ctx_area_fk100: String, // 연속조회용 토큰
    pub ctx_area_nk100: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let holdings = resp.output1
        .map(serde_json::from_value::<Vec<Holding>>)
        .transpose()?
        .unwrap_or_default();
    let summary = resp.output2
        .and_then(|v| serde_json::from_value::<Vec<Summary>>(v).ok())
        .and_then(|mut arr| arr.pop());
    Ok(Response { holdings, summary, ctx_area_fk100: String::new(), ctx_area_nk100: String::new() })
}
```

## 필드명 매핑

- KIS API는 영문 파라미터 키가 대문자(예: `FID_COND_MRKT_DIV_CODE`).
- GET은 `client.get`이 자동으로 대문자로 변환 → `Request`는 소문자 필드 이름으로 써도 됨.
- POST body는 JSON. 대문자 그대로 전송해야 하므로 `#[serde(rename = "CANO")]` 필수.
- 응답 필드도 스펙의 영문명 그대로 사용. 소문자면 `#[serde(rename)]` 불필요.

## `Request` 필드 타입 선택

- 정수/금액/수량도 **`String`**으로 두는 것이 안전 (KIS 명세 통일). 사용자가 `to_string()`해서 넘김.
- `bool`은 `"Y"/"N"` 문자열. 타입은 `String`.
- 날짜는 `YYYYMMDD` 문자열.

## `Response` 필드

- 모두 `String` + `#[serde(default)]`. 빈 값/결측 허용.
- 필드 이름은 스펙의 영문명 그대로 snake_case로 작성 (KIS는 대부분 소문자).

## 에러

- `anyhow::Result` 사용. `?` 연산자로 전파.
- 응답에 `output*`이 없는 경우: `.context("응답에 output 없음")` 또는 `anyhow::anyhow!(...)`.

## 주석

- 파일 최상단에 `//!` 로 한글 API 이름과 endpoint, 스펙 경로.
- 필드별 인라인 주석은 최소화. 한글명은 주석 없이 영문 필드만 둔다 (Response가 너무 길어짐).
- WHY가 필요한 곳에만 주석 (예: TR_ID가 실전/모의 다른 경우 → 이미 const로 명시).
