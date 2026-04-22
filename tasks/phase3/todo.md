# Phase 3 — `kis daytrade run` (실주문)

## 배경
- Phase 2 (paper) 완성: EOD 강제청산 · 손절(SL)/익절(TP) · 세션 자동 리포트 · 히스토리 조회 완료.
- 2026-04-22 추가: **`--qty` + `--budget` 필수화 + 예산 한도 내 피라미딩** (`paper.rs:250-275`).
- Phase 3는 **paper의 모든 로직 + 실주문 어댑터 + 실주문 특화 안전장치**.

## 핵심 설계 — 공통 엔진 추출
- `paper.rs`의 tick 루프·청산 우선순위·피라미딩 분기는 **paper/run 공통**.
- `Executor` trait 도입해 체결 경로만 다형화 → 로직 중복 제거.

```rust
#[async_trait]
trait Executor {
    async fn buy(&self, code: &str, qty: u64, ref_price: f64) -> Result<Fill>;
    async fn sell(&self, code: &str, qty: u64, ref_price: f64) -> Result<Fill>;
}

struct Fill { qty: u64, price: f64, ts: DateTime<Tz> }

struct PaperExecutor { slippage_bps: f64 }  // 기존 로직
struct LiveExecutor  { client: Arc<KisClient>, order_type: OrderType, tick_offset: i32 }
```

---

## 작업 항목

### 3.1 공통 엔진 리팩토링 (Phase 3 선행) ✅
- [x] `src/commands/daytrade/engine.rs` 신설 — tick, 청산 우선순위, 피라미딩, EOD, SL/TP
- [x] `Executor` trait + `Fill` 구조체 + `sync_position` 훅
- [x] `PaperExecutor` 구현 (기존 slippage 가상 체결 이전)
- [x] `paper.rs` 를 engine + PaperExecutor 얇은 래퍼로 축소 (90줄)
- [x] **ATR 기반 SL/TP 추가** (`--stop-loss-atr N` / `--take-profit-atr N`)
  - [x] ATR(Wilder) 계산 함수 → `indicators.rs` + 단위테스트
  - [x] SL/TP 판정: `%` / `ATR×N` 둘 다 지정 시 더 타이트한 쪽 적용
  - [x] engine.rs에 넣으므로 paper까지 자동 수혜
- [x] `cargo check` 통과

### 3.2 `LiveExecutor` — 실주문 어댑터 ✅ (기본)
- [x] 국내 매수/매도: `order_cash::call` (지정가, `ord_dvsn=00`)
- [x] 해외 매수/매도: `order::call` (지정가 전용 — 해외 시장가 불가)
- [x] 주문 → 체결 확인 폴링 — 국내 `inquire_daily_ccld` / 해외 `inquire_ccnl`, ODNO 필터, 타임아웃
- [x] 호가 스냅 (`--tick-offset N` — KRX 가격대별 틱, USA 0.01 USD)
- [x] `CLI daytrade run` + `unpack_daytrade_run` + 대화형 `--yes` 확인
- [ ] `OrderType::Market` 스위치 — 국내만 가능 (해외는 EOD 강제청산 시 -N% limit로 대체 필요)
- [ ] partial fill 처리 — 현재는 부분체결 시 경고만, 미체결 수량 자동 취소/재시도 없음

### 3.3 실주문 전용 안전장치
- [ ] `--daily-max-trades N` — 일일 체결 횟수 제한 (매수+매도 합산, 초과 시 신규 진입 차단)
- [ ] `--daily-loss-limit <amount>` — 누적 실현 PnL ≤ -amount면 **킬스위치** (전량 청산 + 세션 종료)
- [ ] `--cooldown N` — 직전 체결 후 N봉 대기 (스팸 주문 방지, paper는 쿨다운 없음)
- [ ] 첫 매수 전 `--yes` 없으면 대화형 확인 (`Proceed with real orders? [y/N]`)
- [ ] **`inquire_balance` 기반 포지션 동기화** (평단가 출처 — 고려사항 #4 확정)
  - `Executor` trait에 `async fn sync_position(code) -> Result<Option<Position>>` 추가
    - `PaperExecutor`: `None` 반환 (엔진 자체 계산 유지)
    - `LiveExecutor`: `inquire_balance` 호출 → 해당 종목 row를 `Position { qty, avg_price, entry_time }` 으로 매핑
  - 호출 시점:
    - ① 세션 시작 시 — 기존 보유 로드 (이전 세션 이어받기)
    - ② 매수/매도 체결 직후 — 평단가/수량 재동기화 (부분체결·수수료 반영)
  - 불일치 감지: 매수 후 sync 결과가 엔진 예상과 크게 다르면 경고 로그 (예: 기대 3주, 실제 2주)

### 3.4 EOD 강제 청산 (실주문)
- [ ] 장 마감 10분 전 전 포지션 **시장가 매도** 발주
- [ ] 체결 확인 실패 시 최대 3회 재시도 (1초 간격)
- [ ] 3회 실패 시 텔레그램 긴급 알림 + 세션 로그에 미체결 경고
- [ ] 이미 수동으로 매도된 경우 대비 — 잔고 0이면 skip

### 3.5 에러 핸들링
- [ ] 주문 거절 코드 분류: 잔고부족 / 거래정지 / 가격범위초과 / 기타
- [ ] 잔고부족·거래정지 → 세션 종료 + 리포트
- [ ] 가격범위초과 → 현재가 재조회 후 재시도 (최대 2회)
- [ ] 네트워크 단절 → 재연결 시 `inquire_balance`로 포지션 재동기화

### 3.6 CLI
```
kis daytrade run <strategy> <symbol>
  --qty N --budget X                       # 필수 (paper와 동일)
  --usa --period 5m --pick N
  --stop-loss-pct N --take-profit-pct N
  --stop-loss-atr N --take-profit-atr N    # ATR 배수 기반 (% 와 택일)
  --daily-max-trades N
  --daily-loss-limit AMOUNT
  --cooldown N
  --order-type limit|market                # 기본: limit
  --tick-offset N                          # limit일 때 호가 스냅 (기본: 0)
  --yes                                    # 첫 진입 확인 생략
```

### 3.7 텔레그램 알림
- [ ] 진입/청산 체결 시 메시지 (수량, 가격, PnL)
- [ ] 킬스위치 발동 시 **긴급 알림** (에러 아이콘 + 사유)
- [ ] EOD 청산 실패 시 긴급 알림

### 3.8 수동 검증
- [ ] 모의투자 계정 불가 (분봉 API 실전 전용) → 실전 계좌 1주 소량으로 라이브 테스트
- [ ] 시나리오: 정상 진입 · SL 발동 · EOD 강제청산 · 킬스위치 발동 · 주문 거절

---

## 고려사항 (구현 전 확인)

1. **paper.rs 리팩토링 범위**: 공통 엔진으로 뽑으면 기존 paper 세션이 돌고 있을 때 배포 주의. 3.1 먼저 완성 후 한 번에 릴리스.
2. **OrderType 기본값**: 지정가가 안전하지만 미체결 시 EOD 놓칠 위험 → 기본 limit, EOD는 **무조건 market** 강제.
3. **피라미딩 쿨다운**: paper는 쿨다운 없이 매 봉 피라미딩 가능. 실주문은 `--cooldown` 필수로 할지, 기본값을 1봉이라도 줄지?
4. **평단가 출처** ✅ **확정**: 실주문은 **`inquire_balance` 우선**. 체결 후 계좌 잔고를 재조회해 평단가/보유수량을 실제 기준으로 동기화. paper는 기존대로 체결가 기반 자체 계산.
   - 이유: KIS 내부 수수료/환율/부분체결 반영이 자체 계산보다 정확. 피라미딩 시 오차 누적 방지.
   - 구현: `Executor` trait에 `fn sync_position(code) -> Option<Position>` 추가. paper는 `None` 반환(엔진이 자체 계산 유지), live는 `inquire_balance` 결과를 매핑해 반환.
5. **세션 재시작**: 프로세스 재시작 시 기존 포지션을 어떻게 이어받을지 (session_id 재발급 vs 기존 세션 복구).

---

## 완료 후 이어질 작업 (Phase 4)
- 분봉 백테스트 + VWAP/갭/ORB 전략 (기존 todo.md Phase 4)
- 다종목 병렬 감시 (Phase 2.6 보류분)
