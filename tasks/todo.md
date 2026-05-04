# `kis daytrade` 커맨드 트리 도입 (국장/미장)

## 배경
- 기존 `signal-watch`/`backtest`는 일/주/월봉만 지원 → 데이트레이드 불가.
- `kis daytrade` 최상위 커맨드 트리를 만들어 분봉 기반 감시/페이퍼/실주문/백테스트를 통합.
- **확정 요구:** 데이트레이드 특성상 **장 마감 10분 전 모든 포지션 강제 청산** (paper/run 공통).

## 설계 요약

### 데이터 레이어
- 기존 `Series` (dates: `Vec<String>` YYYYMMDD)를 분봉에도 쓰려면 **일자+시각** 결합 타임스탬프로 확장 필요.
  - 방안: `Series.dates`를 그대로 두되 분봉에서는 `"YYYYMMDD HHMM"` 포맷으로 사용.
- 분봉 fetch 경로 신설 (`src/commands/daytrade/fetch.rs`):
  - 국내 당일: `inquire_time_itemchartprice` (30건/페이지, 기준시각 페이징)
  - 국내 과거: `inquire_time_dailychartprice` (날짜+시각 페이징, 최대 1년, 모의 X)
  - 해외: `overseas_stock/quotations/inquire_time_itemchartprice` (`nmin` 파라미터, 모의 X)

### 세션 엔진 (`src/commands/daytrade/session.rs`)
- `Session` enum: `Krx` / `UsaDst` / `UsaStd` (일광절약 자동 판정)
- 시장 시간 테이블:
  - KRX: 09:00–15:30 KST
  - 미장 서머: 22:30–05:00 KST (DST)
  - 미장 윈터: 23:30–06:00 KST
- 공개 함수:
  - `fn market_close_kst(market, date) -> DateTime`
  - `fn is_in_session(market, now) -> bool`
  - `fn minutes_to_close(market, now) -> i64`
  - `fn should_force_exit(market, now) -> bool` — 마감 10분 전 true

### 전략 재사용
- 기존 `backtest::compute_signals` / `Strategy` / `Params` 그대로 사용.
- 분봉에서도 MA/RSI/MACD/볼린저/OBV 전부 동작 (인덱스 기반이라 봉 주기 독립).

### CLI 구조 (안)
```
kis daytrade signal-watch <strategy> <symbol> [--usa] [--period 1m|5m|...] [--cron ...]
kis daytrade paper        <strategy> <symbol> [--usa] [--period ...] [--capital N]
kis daytrade run          <strategy> <symbol> [--usa] [--period ...] [--qty N] [--stop-loss-pct X]
kis daytrade backtest     <strategy> <symbol> [--usa] [--period ...] [--from] [--to]
```

---

## Phase 1 — MVP: `daytrade signal-watch` (분봉 감시) ✅

- [x] `src/commands/daytrade/mod.rs` 신설, `fetch`/`session`/`period`/`signal_watch` 서브모듈
- [x] 분봉 fetch 구현
  - [x] 국내 (`inquire_time_dailychartprice` 1분봉 페이징 + 클라이언트 집계)
  - [x] 해외 (`overseas_stock/quotations/inquire_time_itemchartprice`, `nmin`=period)
  - [ ] 국내 당일(`inquire_time_itemchartprice`) 모의투자 폴백 — 유보 (실전 전용)
- [x] `Period` enum: `Min(u8)` — `"1m"/"5m"/..."` 파싱
- [x] 세션 엔진 (KRX/미장, DST 자동, `is_in_session`/`time_until_open`/`next_bar_boundary`/`should_force_exit`)
- [x] `kis daytrade signal-watch <strategy>` 서브커맨드 (ma-cross/rsi/macd/bollinger/ichimoku/obv 6종)
- [x] cron 대신 tokio 기반 동적 스케줄러 (세션 엔진이 봉 경계 + 10초 슬랙)
- [x] 감시 전용 — 청산 로직은 Phase 2
- [x] `cargo check` 통과, help 출력 정상
- [ ] 수동 동작 확인 (국장/미장 각 1회) — 사용자 실행 대기

## Phase 2 — `daytrade paper` (실전 서버 기반 모의테스트 + EOD + 손절) ✅

**정의:** `paper` = 실전 KIS API 분봉 데이터로 가상 매매. 실주문 없음. 분봉 API가 모의투자 미지원이라 실전 계정 필수.

**청산 우선순위:** EOD 강제 청산 > 손절(stop-loss) > 전략 신호

- [x] 인메모리 포지션 추적 (`Position { qty, avg_price, entry_time }`)
- [x] 가상 체결 엔진 (최신 봉 종가 × (1 ± `slippage_bps`/10000))
- [x] 수수료 — 매매 한쪽당 `fee_bps`, 청산 시 양쪽 합산 차감
- [x] **장 마감 10분 전 강제 청산** — `session::should_force_exit(10)`; 포지션 즉시 해소 + 신규 진입 차단
- [x] **손절(stop-loss)** — `--stop-loss-pct N` 옵션, 진입가 대비 -N% 이하 시 즉시 청산 (신호보다 우선)
- [x] 매매 기록 SQLite (`~/.config/kis-cli/daytrade.db`)
  - `trades(id, session_id, symbol, market, side, qty, price, ts, strategy, mode, pnl, pnl_pct, reason)`
- [x] 세션 종료(Ctrl+C) 시 일일 리포트 (체결수, 승률, 총 PnL, 평균 PnL %)
- [x] 헬프/시작 로그 문구 — "실전 서버 기반 모의테스트" 명시
- [ ] 수동 동작 확인 (국장/미장 각 1회) — 사용자 실행 대기 (내일 IONQ bollinger 테스트 예정)

## Phase 2.5 — UX·운영 편의 (history / take-profit / 자동 리포트) ✅

**범위 확정 (2026-04-21):** 1~3만. 4~6은 Phase 2.6로 분리.

### 2.5.1 `kis daytrade history` 서브커맨드 (SQLite 조회) ✅
- [x] `storage.rs` 쿼리 추가 (`recent_sessions`, `trades_for_session`, `trades_filtered`)
- [x] `SessionRow` / `TradeRow` (`#[derive(Serialize)]`) 추가
- [x] `src/commands/daytrade/history.rs` 신설 — 포맷팅/출력
- [x] main.rs `DaytradeAction::History { session, symbol, today, days, limit, json }`
- [x] CLI: 기본 세션 요약 / `--session` / `--symbol` / `--today` / `--days` / `--limit` / `--json`
- [x] smoke: `daytrade history`, `--json`, `--symbol`, `--today` 모두 정상 응답

### 2.5.2 익절 `--take-profit-pct` (손절 대칭) ✅
- [x] `paper::Config`에 `take_profit_pct: Option<f64>` 추가
- [x] `tick()` SL/TP 통합 블록 (우선순위: EOD > SL > TP > 신호)
- [x] reason 라벨: `"익절 (+X.XX% ≥ +TP%)"`
- [x] `DaytradePaperCommonArgs`에 `--take-profit-pct` 옵션
- [x] 시작 로그에 `SL=... TP=...` 표시

### 2.5.3 세션 자동 종료 리포트 + session_id 롤오버 ✅
- [x] `paper::run` 루프에 `session_reported: bool` 상태
- [x] EOD 청산 블록 끝에서 `!session_reported`면 자동 `print_report` + 플래그 세팅
- [x] `tick()` 초입에서 `session_reported && is_in_session`면 session_id 재발급 + 플래그 리셋 + position 안전 리셋
- [x] 새 세션 시작 로그 출력
- [x] Ctrl+C 경로는 플래그 존중 (중복 리포트 방지)

---

### Phase 2.6 (보류)
- [ ] 다종목 병렬 감시 — 한 프로세스당 한 종목 제약 해소
- [ ] 국내 분봉 API 모의투자 폴백 (`inquire_time_itemchartprice` 당일용)
- [ ] 휴장일 체크 — 현재는 요일만 봄 (`countries_holiday` API 활용)

## Phase 3 — `daytrade run` (실주문)

→ 세부 계획: [`tasks/phase3/todo.md`](phase3/todo.md)

요약: 공통 엔진 추출(`engine.rs` + `Executor` trait) → `LiveExecutor` 실주문 어댑터 → 실주문 안전장치(쿨다운·일일 횟수·킬스위치·포지션 동기화) → EOD 시장가 강제청산 → ATR 기반 SL/TP(공통) → 텔레그램 알림.
포지션 사이징은 `--qty + --budget` 필수 (paper와 동일, `--qty-pct` 는 사용 안 함).

## Phase 4 — `daytrade backtest` + 특화 전략

- [ ] 분봉 백테스트 경로 (기존 backtest 엔진 재사용, Series 분봉 버전)
- [ ] 세션 경계 고려 (overnight gap 제외, EOD flat)
- [ ] VWAP 돌파 / 갭 / ORB(Opening Range Breakout) 전략 추가

---

## 우선 확정 사항 (구현 전 합의 필요)

1. **MVP = Phase 1만**으로 갈지, Phase 2(paper + 강제청산) 까지 한 번에 묶을지.
2. **분봉 파싱 형식** — `"1m"`, `"5m"`, `"5min"`, 아니면 정수 `--minutes 5`? (`"1m"` 제안)
3. **미장 cron** — 정적 cron은 DST 때문에 깨짐. `tokio` 루프로 동적 스케줄링 제안.
4. **Period enum 도입 위치** — 기존 `backtest`/`signal-watch`의 `period: char`도 같이 바꿀지, 아니면 `daytrade`만 신규 enum 쓰고 기존은 유지할지.

---

# Phase 5 — `daytrade daemon` (단일 프로세스 + toml 기반)

## 배경
- 현재 `paper|run --background` 는 종목·전략별로 systemd unit을 따로 등록 → N개의 프로세스 = N개의 KIS API 클라이언트 = TPS 압박 (EGW00201) + 토큰 중복 + 분봉 fetch 중복.
- systemd가 unit을 재실행할 때 매번 symbol resolve 를 다시 함 → "복수 매칭(N) TTY 아님" 에러로 30초마다 크래시 루프 (관측됨: SK하이닉스 000660).
- 단일 데몬 + toml 설정 + 파일 watch hot-reload 로 통합.

## 설계 (확정)
- **설정**: `~/.config/kis-cli/daytrade.toml` (`[[strategy]]` 배열, `id` = ULID, `code`+`market` = 사전 resolve)
- **CLI 진입 관리** (toml만 수정, 데몬 무관):
  - `add <mode> <kind> <symbol> [opts]` — 1회 resolve, ULID 부여, append
  - `rm <id>` — 제거
  - `list` — toml 항목 표시 (+ 데몬 살아있으면 라이브 상태 병합)
- **CLI 라이프사이클**:
  - `start` — `kis-daytrade.service` (단일 unit) 설치 + enable + start
  - `stop` — stop + disable + unit 파일 삭제
  - `status` — systemctl status + 라이브 strategy 상태
- **내부**: `daemon` — 포그라운드 실행, systemd ExecStart 가 호출
- **`paper|run --background` 제거** (CLI 단계에서 깔끔히 제거)

## Daemon 동작
- 시작 시 toml 로드 → strategy별 `tokio::spawn`, 각 task `CancellationToken` 보유
- `notify` 로 toml watch (debounce 500ms): 신규 spawn / 삭제 cancel / 변경은 다음 진입부터 적용 (즉시 교체는 rm+add)
- `KisClient` Arc 공유 (TPS·토큰 중앙)
- 같은 `(code, period)` 분봉 fetch dedup (한 번 받아 fan-out)
- panic 격리: JoinHandle 감시 → panic 시 30초 후 재spawn (해당 strategy만)
- SIGTERM: 모든 task cancel, `run` 모드 보유 포지션 평탄화 후 종료

## 구현 단계

### 5.1 의존성·기반
- [ ] `Cargo.toml`: `notify = "8"`, `ulid = "1"`, `tokio-util` (CancellationToken; 이미 있으면 skip)
- [ ] `src/commands/daytrade/dconfig.rs` — toml 스키마 (`StrategyEntry`, `DaytradeConfig`), load/save/diff

### 5.2 engine 어댑터
- [ ] `EngineConfig` 에 `pre_resolved: Option<ResolvedSymbol>` 추가 (있으면 lookup skip)
- [ ] `engine::run` 시그니처 유지하되 cfg.symbol 비어있고 pre_resolved 있으면 그걸 사용

### 5.3 CLI 진입 관리
- [ ] `kis daytrade add` — symbol resolve → `StrategyEntry` 생성 → toml append
- [ ] `kis daytrade rm <id>` — toml에서 id 일치 제거
- [ ] `kis daytrade list` — 표 출력 (id 단축 8자, kind, code, market, mode, qty, budget)

### 5.4 Daemon
- [ ] `src/commands/daytrade/daemon.rs` — main loop:
  - `Arc<KisClient>` 1개
  - `HashMap<Ulid, RunningTask>` (CancellationToken + JoinHandle)
  - 파일 watcher 스레드 → mpsc::UnboundedSender 로 reload 이벤트
  - reload 시 diff → spawn / cancel
  - 각 task: `engine::run(client.clone(), cfg, executor)` 무한 루프 (단, EOD 후 다음날 재시작)
- [ ] `kis daytrade daemon` 서브커맨드 추가
- [ ] panic 핸들링: `tokio::spawn` 결과를 별도 task에서 `await` → panic이면 toml에서 제거하지 않고 30초 후 재spawn
- [ ] SIGTERM 핸들러 — 모든 token cancel, run 모드 평탄화 (engine 안에 이미 EOD 평탄화 로직 있음, 재사용 가능한지 확인)

### 5.5 라이프사이클 CLI
- [ ] `kis daytrade start` — `/etc/systemd/system/kis-daytrade.service` 작성, daemon-reload, enable --now
- [ ] `kis daytrade stop` — disable --now, unit 파일 삭제
- [ ] `kis daytrade status` — systemctl status + (가능하면) IPC 또는 `~/.local/state/kis-cli/daytrade-state.json` 읽어서 라이브 상태 표시

### 5.6 정리
- [ ] `paper|run --background` CLI 플래그 제거
- [ ] 기존 per-strategy `kis-daytrade-*-*` unit 검출 시 `start` 에서 경고 + `daytrade remove --all` 안내
- [ ] README/SKILL.md 업데이트

### 5.7 검증
- [ ] `cargo check` 통과
- [ ] `add` → `list` → 로컬에서 `daemon` 포그라운드 실행 → toml 편집 시 hot-reload 확인
- [ ] paper 한 건 + run --yes 1건으로 실제 분봉 받아 동작 확인
