# Ralph Loop — 교훈 (lessons learned)

> 매 iteration 시작 시 반드시 읽을 것. 새 실수 발생 시 하단에 누적.

---

## 초기 규칙 (사전 설정됨)

### L01. registry는 single source of truth
`tasks/api-registry.json`의 `status` 필드만 믿는다. 파일 존재 여부로 판단하지 말 것 — 레이스/손상 가능.

### L02. cargo check 실패 3회 → 중단
무한 수정 루프 방지. 3회 실패 시 `status=blocked`로 표기하고 이 파일에 원인 기록.

### L03. 실시간(WebSocket) 카테고리는 `call()` 함수 없음
`.agent/rust-patterns.md`의 실시간 섹션 참조. `subscribe_payload` + `parse_frame` + `Response`만 제공.

### L04. POST body는 영문 필드 대문자 rename 필수
`#[serde(rename = "CANO")]` 등. 잊으면 KIS가 400 반환 (그러나 compile은 통과 — 이 문제는 실제 호출 시에만 드러남).

### L05. 응답 필드는 String + `#[serde(default)]`
정수/금액도 String. KIS가 빈 문자열 돌려주는 경우가 흔하며, `u64`로 선언하면 역직렬화 실패.

---

## iteration 누적 교훈

<!-- Ralph Loop가 실수할 때마다 아래에 추가. 형식:
### L0X. <제목>
**상황**: <어떤 API에서 어떤 실수>
**원인**: ...
**규칙**: 다음부터는 ...
-->

### L06. OAuth 토큰 발급/폐기 API는 KisClient 사용 금지
**상황**: 첫 iteration(`oauth__tokenp`)에서 발견. `KisClient::post_json`은 항상 `authorization: Bearer <token>` 헤더를 붙이는데, 이 API는 **토큰 발급/폐기**이므로 Bearer를 붙이면 안 된다.
**규칙**:
- `/oauth2/tokenP`, `/oauth2/revokeP`, `/oauth2/Approval`, `/uapi/hashkey` 는 `reqwest::Client::new()`로 직접 POST.
- 시그니처는 `pub async fn call(is_mock: bool, req: &Request) -> Result<Response>` (KisClient 대신 `is_mock: bool`만 받음).
- `use crate::client::{BASE_URL_MOCK, BASE_URL_PROD};` 로 도메인 상수만 빌림.

### L07. 응답 필드에 `Number` 타입이 있으면 `i64` 허용
**상황**: `oauth__tokenp`의 `expires_in`이 `Number, Length 10` (예: 7776000).
**규칙**:
- 스펙 Type이 `Number`이고 분명히 정수인 경우 `#[serde(default)] pub field: i64` 사용 가능.
- 애매하면 여전히 `String`이 안전 (KIS는 Number 표기해놓고 문자열 돌려주는 케이스도 있음).
- 확신 없으면 `String` 고수.

### L10. 모의투자 미지원 API는 is_mock일 때 거부
**상황**: 신용주문, 예약주문, 해외주식 상당수 등 "모의Domain: 모의투자 미지원"이라고 명시된 API.
**규칙**:
- 파일 상단 주석에 "모의투자 미지원"을 명기.
- `call()` 본문 첫 줄: `if client.is_mock() { bail!("... 모의투자 미지원 API입니다"); }`.
- TR_ID 상수는 실전용 2개만 (`TR_ID_BUY`, `TR_ID_SELL`, 혹은 단일 `TR_ID`).

### L11. Response 필드 대소문자는 스펙의 표기를 따른다
**상황**: `order_cash`는 Response가 `KRX_FWDG_ORD_ORGNO`(대문자, rename 필요), `order_credit`은 `krx_fwdg_ord_orgno`(소문자). 스펙 표기를 무시하면 역직렬화 실패.
**규칙**:
- 스펙 Response Body 테이블의 Element 이름을 **그대로** 필드명으로 사용 (대문자면 snake_case 필드 + `#[serde(rename = "...")]`, 소문자면 rename 없이).
- 확신 없으면 `#[serde(default)]`는 항상 붙임 (빈 응답 방어).

### L09. 주문 API의 TR_ID는 매수/매도 × 실전/모의 4종 분기
**상황**: `domestic_stock__order_account__order_cash` (주식주문 현금)에서 TR_ID가 4개 (TTTC0011U/TTTC0012U/VTTC0011U/VTTC0012U).
**규칙**:
- `pub enum Side { Buy, Sell }` 선언.
- `call(client, side: Side, req)` 시그니처로 매수/매도 구분 받음.
- `match (client.is_mock(), side)`로 4가지 TR_ID 중 선택.
- 상수 이름 관례: `TR_ID_REAL_BUY`, `TR_ID_REAL_SELL`, `TR_ID_MOCK_BUY`, `TR_ID_MOCK_SELL`.

### L08. tokenp는 Response에 `Option<>` 감싸지 않음
**상황**: 토큰 발급 API는 성공 응답이 `{access_token, token_type, expires_in, access_token_token_expired}` 네 필드가 반드시 채워짐. `ApiResponse`(rt_cd/output 래퍼) 형태가 아니라 **평면 JSON**으로 옴.
**규칙**:
- `resp.output`에서 꺼내지 말고 `serde_json::from_str(&body)`로 직접 파싱.
- 즉, 토큰 발급류는 `ApiResponse` 래퍼를 통하지 **않는다**.
- 이 패턴은 OAuth 카테고리 4개(tokenP, revokeP, Approval, hashkey)에만 적용. 다른 API는 전부 `ApiResponse`를 통과함.

