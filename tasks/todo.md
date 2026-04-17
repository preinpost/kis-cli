# CLI 완성 계획 (Option B — 큐레이션 + 종목 검색)

## 목표
338개 API 바인딩 중 실사용 가치 높은 ~40개를 CLI로 노출. 구조는 `api/` 디렉토리 트리와 1:1로 맞춤. 종목명→코드 자동 해석을 전 커맨드 기본 ON.

## 최종 CLI 표면

```
kis auth                        # 기존 유지
kis config init|path            # 기존 유지

kis symbols sync [--if-stale]   # 신규: 마스터 파일 다운로드·파싱·DB 갱신
kis symbols find <query>        # 신규: 이름/코드 검색 (FTS5)

kis stock dome
  price <symbol>                # 현재가 (기존 `kis price` 이관)
  chart <symbol> [--period D|W|M]  # 일/주/월봉
  asking <symbol>               # 호가
  ccnl <symbol>                 # 체결 내역
  balance                       # 잔고 (기존 `kis balance` 이관)
  deposit                       # 예수금
  order buy|sell <symbol> <qty> [--price N]  # 현금주문
  order cancel <order-no>       # 주문 취소
  ccld-history                  # 체결 내역 조회
  rank volume|change            # 순위조회
  watch <symbol>                # 실시간 체결 (기존 `kis watch` 이관)
  watch-asking <symbol>         # 실시간 호가

kis stock usa
  price <symbol>                # 해외 현재가
  chart <symbol> [--period D|W|M]
  balance
  order buy|sell <symbol> <qty> [--price N]
  ccld-history
  watch <symbol>

kis bond
  price <symbol>
  chart <symbol>
  balance
  order buy|sell <symbol> <qty> --price N

kis fo dome                     # 국내선물옵션
  price <symbol>
  chart <symbol>
  balance
  order buy|sell <symbol> <qty> [--price N]

kis fo usa                      # 해외선물옵션
  price <symbol>
  balance
  order buy|sell <symbol> <qty> [--price N]
```

**symbol 인자 자동 해석**: 숫자 6자리=국내코드, 영문=해외 티커, 그 외=이름 검색 (SQLite FTS5 매칭). 결과 0개→에러, 1개→진행, 복수→번호 선택 프롬프트.

## 작업 항목

### 1. 종목 마스터 인프라 (독립 작업, 먼저)
- [ ] `rusqlite` + `zip` + `encoding_rs` 의존성 추가
- [ ] `src/symbols/mod.rs` — DB 스키마 (symbol, name_kr, name_en, exchange, type), FTS5 virtual table
- [ ] `src/symbols/master.rs` — KOSPI/KOSDAQ/NASDAQ/NYSE/AMEX 마스터 파서 (고정폭 EUC-KR)
- [ ] `src/symbols/sync.rs` — zip 다운로드→압축해제→파싱→upsert
- [ ] `src/symbols/lookup.rs` — 코드/이름 자동 해석, 복수 결과 프롬프트
- [ ] `src/commands/symbols.rs` — `sync`/`find` 서브커맨드
- [ ] `~/.config/kis/symbols.db` 경로 관리 (`dirs` 재사용)
- [ ] README에 cron 예시 기재

### 2. CLI 구조 재편
- [ ] `main.rs` clap 구조 3단 중첩 (`Stock { dome|usa }`, `Fo { dome|usa }`, `Bond`)
- [ ] `commands/` 디렉토리 재구성:
  - `commands/stock/dome.rs`, `commands/stock/usa.rs`
  - `commands/bond.rs`
  - `commands/fo/dome.rs`, `commands/fo/usa.rs`
  - `commands/symbols.rs`
- [ ] 기존 `commands/price.rs`, `balance.rs`, `watch.rs` 내용을 `stock/dome.rs`로 이관 후 삭제 (alias 없이 깔끔하게)

### 3. 국내주식 커맨드 구현 (`commands/stock/dome.rs`)
각 함수는 기존 구현된 `api::domestic_stock::*::call()` 얇게 래핑 + 포맷팅만.
- [ ] `price` — `quotations::inquire_price`
- [ ] `chart` — `quotations::inquire_daily_itemchartprice`
- [ ] `asking` — `quotations::inquire_asking_price_exp_ccn`
- [ ] `ccnl` — `quotations::inquire_ccnl`
- [ ] `balance` — `order_account::inquire_balance`
- [ ] `deposit` — `order_account::inquire_deposit`
- [ ] `order buy|sell` — `order_account::order_cash` (Side enum 활용)
- [ ] `order cancel` — `order_account::order_rvsecncl`
- [ ] `ccld-history` — `order_account::inquire_daily_ccld`
- [ ] `rank volume` — `ranking::` 중 거래량순위
- [ ] `rank change` — `ranking::` 중 등락률순위
- [ ] `watch` — `realtime::h0stcnt0` (기존 watch.rs 재활용)
- [ ] `watch-asking` — `realtime::h0stasp0`

### 4. 해외주식 커맨드 (`commands/stock/usa.rs`)
- [ ] `price` / `chart` / `balance` / `order buy|sell` / `ccld-history` / `watch`
- KIS 해외주식은 거래소 구분(`EXCD`) 필수 → 자동해석으로 채움 (NASD/NYSE/AMEX)

### 5. 채권 (`commands/bond.rs`)
- [ ] `price` / `chart` / `balance` / `order buy|sell`

### 6. 선물옵션 국내/해외 (`commands/fo/*.rs`)
- [ ] 국내: `price` / `chart` / `balance` / `order buy|sell`
- [ ] 해외: `price` / `balance` / `order buy|sell`

### 7. 검증
- [ ] `cargo check` 통과
- [ ] `cargo build --release` 통과
- [ ] `kis symbols sync` 실제 실행 — 국내+나스닥 최소 확인
- [ ] `kis symbols find 테슬라` → TSLA 반환 확인
- [ ] `kis stock dome price 삼성전자` → 005930 자동 해석 확인
- [ ] 모의투자 미지원 API는 `is_mock` 시 명확 에러 (기존 L10 패턴)

## 주의사항
- 마스터 파일 EUC-KR 인코딩 (CP949) — `encoding_rs` 필수
- KOSPI/KOSDAQ `.mst`와 해외 `.cod` 포맷 다름 — 각각 파서 필요
- 복수 매칭 시 non-TTY 환경(파이프) 고려: `--pick N` 플래그로 강제 선택 허용
- 기존 Flat 커맨드(`kis price`, `kis balance`, `kis watch`) 제거 — 사용자 확인됨

## 리뷰 (구현 완료 2026-04-17)

### 최종 상태
- `cargo build --release` 통과
- `kis symbols sync` 실제 실행 성공:
  - KOSPI 2508, KOSDAQ 1823, NASDAQ 5100, NYSE 2419, AMEX 4170 = 총 16,020개
- `kis symbols find 테슬라` → 한글명 "테슬라" 매칭, NASD TSLA 포함
- `kis symbols find Tesla` → TSLA(NASD) 최상위

### 최종 스코프
- `kis auth / config / symbols {sync,find}` — 4
- `kis stock dome {price,chart,asking,balance,psbl,order {buy,sell,cancel},history,watch}` — 10
- `kis stock usa {price,chart,balance,order {buy,sell},history}` — 6 (watch는 v2로 보류)
- `kis bond {price,chart,balance,order {buy,sell}}` — 5
- `kis fo dome {price,balance,order {buy,sell}}` — 4
- `kis fo usa {price,balance,order {buy,sell}}` — 4
- **총 33개 커맨드**

### 중요 결정 / 주의
- 해외 `.cod` 파서는 탭 구분 (필드 [4]=code, [6]=name_kr, [7]=name_en). 최초 휴리스틱은 중복 key 생성으로 실패 → 실제 포맷 확인 후 재작성.
- KOSPI/KOSDAQ `.mst`는 "라인 끝 228바이트 트레일러 + 앞 9/12바이트 고정 + 나머지 한글명" 관례 파싱.
- symbol 자동해석: `stock` 만 지원 (bond/fo는 마스터 파일 없음 — 코드 직접 입력).
- `ws.rs`의 `run` → `run_domestic`으로 개명. 해외 실시간 (HDFSCNT0)은 v2 이관.

### 다음 단계 (사용자 검증)
1. `kis config init` — APP_KEY/SECRET/계좌 설정
2. `kis auth` — 토큰 발급 확인
3. `kis stock dome price 삼성전자` — 자동 해석 + 시세 조회 확인
4. `kis stock usa price Tesla` — 해외 자동 해석 확인
5. `kis stock dome balance` — 잔고 조회 (실 계좌)

### 남은 개선 TODO (v2)
- `stock usa watch` (HDFSCNT0) 연결
- `stock dome watch-asking` (H0STASP0) — 호가 실시간
- `stock dome rank {volume, change}` — 순위조회
- FTS5 rank 튜닝 (정확 일치 우선)
- 해외 주식 거래소 자동 감지 (현재는 `--exchange` 기본 NASD)
- 테스트: 파서 유닛 테스트, 특히 가변 한글명 엣지케이스
