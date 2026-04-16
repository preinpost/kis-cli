# Rust 패턴 — Ralph 참고 (간결)

## `KisClient` API 치트시트

```rust
// GET: 쿼리 파라미터 키는 자동 대문자화됨
client.get(endpoint, tr_id, &[("key", "value"), ...]).await?

// POST JSON
client.post_json(endpoint, tr_id, &request_body, &[("extra-header", "value")]).await?

// 범용 HTTP method
client.send_json(reqwest::Method::DELETE, endpoint, tr_id, &body, &[]).await?

// 실전/모의 판별
if client.is_mock() { TR_ID_MOCK } else { TR_ID_REAL }

// 계좌번호 파츠
client.cano()         // 앞 8자리
client.product_code() // 뒤 2자리 (ACNT_PRDT_CD)
```

## `ApiResponse` 구조

```rust
pub struct ApiResponse {
    pub rt_cd: String,   // "0" = 성공 (client.get이 이미 검사함)
    pub msg_cd: String,
    pub msg1: String,
    pub output: Option<serde_json::Value>,
    pub output1: Option<serde_json::Value>,
    pub output2: Option<serde_json::Value>,
}
```

- 스펙에 `output`만 있으면 → `resp.output`에서 꺼내기.
- `output1` / `output2`로 나뉘면 → 각각 꺼내기. 하나는 배열이고 다른 하나는 단일 객체인 경우 많음.
- rt_cd는 `client.get`/`post_json`이 자동 검사하므로 Response에서 다시 볼 필요 없음.

## serde 관례

- **Request**: `#[derive(Serialize)]`. POST body는 `#[serde(rename = "UPPER_CASE")]` 필수.
- **Response**: `#[derive(Deserialize)]` + 모든 필드 `String` + `#[serde(default)]`.
- 숫자/금액 파싱은 사용자 책임 (`.parse::<f64>()`).
- 응답에 연속조회(FK/NK)가 포함되면 별도 필드로 빼서 Response에 포함.

## 실시간(WebSocket) API 특이사항

실시간 API (`/tryitout/H0xxxxxx`)는 REST가 아니라 **WebSocket 메시지**다:

- 도메인: `ws://ops.koreainvestment.com:21000` (실전), `ws://ops.koreainvestment.com:31000` (모의).
- 요청: JSON `{header: {...}, body: {input: {tr_id, tr_key}}}` 형태로 구독/해지.
- 응답: `0|H0STCNT0|001|005930^123929^...` 같은 `|`/`^` 구분 문자열.
- 현재 `ws.rs`에 H0STCNT0 하나만 구현되어 있음.

**Ralph Loop 권장 접근 (실시간 카테고리)**:
- 모듈을 만들되 `call()` 대신 다음 3개를 제공:
  ```rust
  pub const TR_ID: &str = "H0STCNT0";
  pub fn subscribe_payload(tr_key: &str) -> serde_json::Value { ... }   // 구독 메시지 생성
  pub fn parse_frame(frame: &str) -> Result<Response> { ... }           // ^ 구분 파싱
  pub struct Response { pub mksc_shrn_iscd: String, /* ... */ }
  ```
- 실제 WebSocket 연결은 Ralph가 다루지 않음. `call()` 없이 OK.

## 자주 하는 실수 방지

- `serde_json::from_value(output)?` 에서 `output`은 `serde_json::Value`여야 함. `Option<Value>`인 경우 먼저 `.context(...)?`로 unwrap.
- POST body 필드는 **반드시** `#[serde(rename = "UPPER_CASE")]`. 소문자로 보내면 KIS가 400 반환.
- GET 쿼리 파라미터 키는 `&str`로 배열에 넣으면 됨. 값도 `&str`. `String`은 `.as_str()`.
- `mod.rs`에 `pub mod` 추가 잊지 말기.
