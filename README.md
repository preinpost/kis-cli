# kis-cli

한국투자증권(KIS) Open API를 Rust로 래핑한 CLI. 시세 조회·주문·실시간 스트리밍·기술적 분석·백테스트·자동 손절까지 터미널 한 자리에서, TradingView Lightweight Charts 기반 네이티브 차트 뷰어도 함께 제공한다.

- **API 바인딩 338개** 구현 (OAuth · 국내주식 · 해외주식 · 장내채권 · 국내선물옵션 · 해외선물옵션)
- **CLI 서브커맨드 50개+** — 조회/주문/분석/백테스트/감시/자동매도까지 노출
- **종목 이름 자동 해석** — `삼성전자` → `005930`, `Tesla` → `TSLA`
- **WebSocket 실시간 스트리밍** — 국내주식·해외주식·KRX 야간선물
- **기술적 분석** — MA/RSI/MACD/볼린저/일목균형표/OBV 로컬 계산
- **백테스트 엔진** — 7개 전략, 파라미터 스윕, 수수료·슬리피지·레버리지·손익절 반영
- **시그널 감시** — cron 스케줄로 전략 신호 로깅 (주문 없음)
- **자동 손절 데몬** — 폴링 또는 WebSocket tick 기반 감시 + 매도
- **네이티브 차트 뷰어 (wry)** — 일/주/월 전환, 무한 스크롤, 심볼 검색, 구름 채색, 백테스트 GUI
- **파일락 기반 프로세스간 TPS 제한** — 여러 CLI 인스턴스가 동시에 돌아도 KIS 레이트리밋 준수

## 설치

### A. Rust/cargo 로 설치 (추천)

```bash
cargo install --git https://github.com/preinpost/kis-cli --locked
kis skill install                       # Claude Code 스킬 배포 (선택)
```

`~/.cargo/bin/kis` 에 바이너리 설치. `~/.cargo/bin` 이 `PATH` 에 있어야 한다.

### B. 바이너리만 내려받기 (Rust 불필요)

지원 플랫폼: **macOS arm64 (Apple Silicon)**, **Linux x86_64**.

macOS arm64:

```bash
TAG=$(curl -fsSL https://api.github.com/repos/preinpost/kis-cli/releases/latest | grep tag_name | cut -d'"' -f4)
curl -fsSL -o /tmp/kis.tar.gz "https://github.com/preinpost/kis-cli/releases/download/${TAG}/kis-${TAG}-aarch64-apple-darwin.tar.gz"
tar xzf /tmp/kis.tar.gz -C /tmp
mkdir -p ~/.local/bin
install -m 755 "/tmp/kis-${TAG}-aarch64-apple-darwin/kis" ~/.local/bin/kis
# ~/.local/bin 이 PATH 에 없다면: echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
kis skill install
```

Linux x86_64 는 위 스니펫에서 `aarch64-apple-darwin` 을 `x86_64-unknown-linux-gnu` 로 치환. Linux 는 차트 뷰어용 `libwebkit2gtk-4.1-dev` 가 별도로 필요하다.

### C. 개발 체크아웃 (소스 빌드)

```bash
git clone https://github.com/preinpost/kis-cli
cd kis-cli
cargo run --release -- install          # cargo install --path . + 스킬 배포 (`--force` 덮어쓰기)
```

### 업데이트

```bash
kis version                     # 현재 바이너리 + GitHub 최신 릴리스 비교
kis update                      # GitHub Release 에서 바이너리 다운로드 → atomic 교체 (A·B 경로)
kis update --from-source        # C 경로 — 로컬 리포에서 git pull + cargo install
kis update --from-source --no-pull   # 로컬 변경사항 그대로 재빌드만
```

`kis update` 는 `CARGO_PKG_VERSION` 과 `releases/latest` 태그를 비교해 같으면 스킵, 다르면 현재 플랫폼 triple 에 맞는 tar.gz 를 내려받아 실행 중 바이너리 자리에 rename 으로 갈아끼운다 (rust/cargo/git 불필요). 지원 플랫폼 밖 (macOS Intel 등) 은 `--from-source` 경로.

### 플랫폼 메모

macOS 기본 빌드 (WKWebView 내장). Linux 는 `libwebkit2gtk-4.1-dev`, Windows 는 WebView2 런타임 필요.

## 설정

```bash
kis config init           # APP_KEY / APP_SECRET / 계좌번호 입력
kis config path           # 설정 파일 경로 확인
kis auth                  # 토큰 발급 + WebSocket approval key 확인
```

설정 파일: `~/Library/Application Support/kis-cli/config.toml` (macOS 기준).

```toml
is_mock = false           # true면 모의투자 서버 사용 (최상위 필드)

[credentials]
app_key = "..."
app_secret = "..."
account_number = "12345678-01"   # 뒤 2자리는 상품코드 (ACNT_PRDT_CD)
```

## 종목 마스터 동기화

이름 검색을 쓰려면 먼저 한 번 동기화.

```bash
kis symbols sync                    # KOSPI/KOSDAQ/NASDAQ/NYSE/AMEX + 선물옵션 약 2만개
kis symbols sync --if-stale         # 24시간 지났을 때만
kis symbols find 테슬라              # 한글/영문/코드 혼합 검색
kis symbols find KOSPI200 --limit 5
```

데이터 소스: `new.real.download.dws.co.kr/common/master/*.{mst,cod}.zip` (한국투자증권 공개 마스터 파일).
로컬 DB: `~/Library/Application Support/kis-cli/symbols.db` (SQLite + FTS5).

## 주식 (국내)

```bash
kis stock dome price 삼성전자 --pick 1    # 현재가
kis stock dome chart 삼성전자 --pick 1    # 일봉 (ASCII 표)
kis stock dome asking 005930              # 호가 10호가
kis stock dome balance                    # 잔고
kis stock dome psbl                       # 매수가능금액
kis stock dome history                    # 최근 30일 주문/체결 내역
kis stock dome order buy 005930 10 --price 75000     # 지정가 매수
kis stock dome order sell 005930 10                  # 시장가 매도
kis stock dome order cancel 0000123456               # 주문 취소 (전량)
kis stock dome watch 삼성전자 --pick 1    # 실시간 체결 (WebSocket, H0STCNT0)
```

`<symbol>`은 코드(6자리) 또는 이름. 이름으로 넘기면 마스터 DB에서 검색.
복수 매칭 시 프롬프트 또는 `--pick N`으로 지정.

## 주식 (해외 / 미국)

```bash
kis stock usa price Tesla --pick 1             # TSLA 현재가
kis stock usa chart TSLA                        # 일봉
kis stock usa balance --exchange NASD           # 잔고
kis stock usa order buy TSLA 1 --price 400
kis stock usa history --exchange NASD
kis stock usa watch TSLA --pick 1               # HDFSCNT0 실시간 (한국시각 기준 UTC 표시)
```

거래소 구분: `NASD`(NASDAQ), `NYSE`, `AMEX`.

## 장내채권

모의투자 미지원. 실전 계좌 전용.

```bash
kis bond price KR...                           # 채권 코드 직접
kis bond chart KR...
kis bond balance
kis bond order buy KR... 100 --price 99.50
```

## 선물옵션

### 국내

```bash
kis fo dome price 1A01606                                 # KOSPI200 6월물 시세
kis fo dome balance
kis fo dome order buy 1A01606 1 --price 350.5
kis fo dome watch-night "KOSPI200 선물" --pick 1         # H0MFCNT0 야간선물 실시간
```

### 해외 (모의 미지원)

```bash
kis fo usa price 6EU24                        # 유로 선물
kis fo usa balance --currency USD
kis fo usa order buy 6EU24 1 --price 1.08
```

## 기술적 분석

```bash
kis analyze 삼성전자 --pick 1                  # ASCII 차트 + MA/RSI/MACD/볼린저/일목
kis analyze Tesla --usa --pick 1               # 해외
kis analyze 삼성전자 --pick 1 --json            # 구조화 JSON (LLM 해석 파이프용)
kis analyze 삼성전자 --pick 1 --chart           # 네이티브 차트 창
kis analyze 삼성전자 --pick 1 --chart --json    # 창 + JSON 동시 (Claude 협업용)
kis analyze 삼성전자 --pick 1 --save out.html   # HTML 파일로만 저장
```

### 차트 뷰어 기능 (`--chart`)

- 캔들스틱 + **MA5/20/60** (굵은 선)
- **볼린저 밴드** 상/중/하
- **일목균형표**: 전환선, 기준선, 선행 A/B — 미래 26봉 투영, 두 라인 사이를 양운(초록)/음운(빨강) Canvas 오버레이로 채색
- 하단 **거래량 히스토그램** (상승일 초록 / 하락일 빨강)
- 하단 **MACD** 라인 + 시그널 + 히스토그램
- 상단 **일/주/월 토글**
- 왼쪽 끝 근접 시 **무한 스크롤** (과거 100봉씩 KIS 재호출 → prepend)
- 헤더 **심볼 검색창** — 타이핑으로 FTS5 자동완성, ↑↓ Enter 선택 → 같은 창에서 전환
- 크로스헤어 / 줌 / 팬 (Lightweight Charts 기본)

## 백테스트

전략별 서브커맨드. 공통 옵션(수수료·슬리피지·레버리지·손절/익절·스윕·JSON)은 모든 전략이 공유.

```bash
kis backtest ma-cross 삼성전자 --pick 1                      # 20/60 MA 교차 (기본)
kis backtest ma-cross 삼성전자 --pick 1 --fast 5 --slow 20   # 파라미터 변경
kis backtest rsi 삼성전자 --pick 1 --rsi-period 14 --rsi-oversold 30 --rsi-overbought 70
kis backtest macd 삼성전자 --pick 1                           # 12/26/9 고정, 히스토그램 부호
kis backtest bollinger 삼성전자 --pick 1 --bb-period 20 --bb-sigma 2.0
kis backtest ichimoku 삼성전자 --pick 1                       # 9/26/52 고정, 구름대+전환/기준선
kis backtest obv 삼성전자 --pick 1 --obv-period 20            # OBV vs SMA(OBV,N) 교차
kis backtest manual 삼성전자 --pick 1 \
    --entry-date 20250301 --exit-date 20250420 --direction long
```

### 공통 옵션

```
--period D|W|M          봉 주기 (기본 D)
--from YYYYMMDD         시작일
--to   YYYYMMDD         종료일
--fee-bps 5             수수료 bps (진입·청산 각각, 기본 0.05%)
--slippage-bps 0        슬리피지 bps
--allow-short           숏 포지션 허용 (양방향)
--leverage 1.0          레버리지 배수
--stop-loss-pct 5       포지션 대비 손실 % 도달 시 강제 청산
--take-profit-pct 10    포지션 대비 수익 % 도달 시 강제 청산
--sweep                 내장 그리드 파라미터 스윕 → Sharpe 내림차순 상위 15개
--json                  구조화 JSON 덤프
--usa                   해외 종목
```

### 파라미터 스윕

```bash
kis backtest ma-cross 삼성전자 --pick 1 --sweep --from 20200101
# → 조합별 (Return / Sharpe / MDD / WinRate / Trades) 표 상위 15개
```

### 인터랙티브 백테스트 차트

전략·파라미터를 GUI에서 바꿔가며 곡선/체결 마커 확인.

```bash
kis backtest chart 삼성전자 --pick 1 --from 20200101
```

## 시그널 감시 (주문 없음)

cron 스케줄로 전략 신호를 로그에 기록. 주문은 집행하지 않는 감시 전용.

```bash
# 평일 15:20 (장 마감 10분 전, 당일 매매 여유)에 MA 크로스 감시
kis signal-watch ma-cross 삼성전자 --pick 1

# RSI 과매도/과매수 감시 (매시 30분)
kis signal-watch rsi 삼성전자 --pick 1 --cron "0 30 * * * *"

# 볼린저 돌파 감시
kis signal-watch bollinger 005930 --bb-period 20 --bb-sigma 2.0
```

cron 표현식은 6필드(초·분·시·일·월·요일). 전략 서브커맨드: `ma-cross`, `rsi`, `macd`, `bollinger`, `ichimoku`, `obv`.

## 자동 손절 (stop-loss 데몬)

잔고를 주기적으로 조회(또는 WebSocket tick 수신)하고, 평가손익이 임계치를 벗어나면 매도 주문을 낸다.

```bash
# dry-run — 실제 매도 없이 로그만
kis stop-loss run --threshold -5 --interval 30

# 실제 매도 집행, 특정 종목만
kis stop-loss run --threshold -5 --symbols 005930,TSLA --execute

# WebSocket 실시간 tick 감시 (폴링 대신)
kis stop-loss run --threshold -3 --ws --execute

# 해외주식 지정가 스프레드 조정 (기본 1.0%)
kis stop-loss run --usa-spread 2.0 --execute

# 상태·경로 조회
kis stop-loss status
kis stop-loss path
```

- `--threshold` 음수 권장 (-5 = -5% 이하일 때 매도). 양수면 익절 트리거.
- 미지정 시 전체 잔고 감시, 코드/종목명 일부로 필터링 가능.
- `--execute` 없으면 드라이런 — 실제 주문 없이 로그만 남는다.
- 상태 파일: `~/Library/Application Support/kis-cli/stoploss.status.json` (진행 상황·최근 가격 기록). `kis stop-loss path`로 경로 확인 가능.

## Claude와 같이 쓰기

`--chart --json`으로 차트창 띄우면서 stdout에 JSON 리포트를 출력.
Claude에게 JSON만 붙여넣으면 지표 해석 / 매매 시사점 분석을 받을 수 있다.

```bash
kis analyze 삼성전자 --pick 1 --chart --json
# → 창 열리고 stdout으로 구조화 리포트:
# {"symbol":"005930","name":"삼성전자","current_price":216000,
#  "ma":{"ma5":210400,...,"alignment":"정배열"},
#  "rsi":{"value":61.64,"state":"상승 우세"},
#  "macd":{...}, "bollinger":{...}, "ichimoku":{...},
#  "signals":["5일선 > 20일선 (단기 상승)","일목 양운 위 — 강세"]}
```

`kis install` 한 번만 실행하면 Claude Code가 kis 사용법을 바로 알고 응답한다 (아래).

## Claude 스킬 설치

```bash
kis skill install          # ~/.claude/skills/kis/SKILL.md 배포
kis skill install --force  # 덮어쓰기
kis skill path             # 설치 경로 확인
kis skill uninstall        # 삭제
```

설치 후 Claude Code에서 "삼성전자 분석해줘" "내 잔고 확인해줘" "삼성전자 MA 크로스 백테스트 돌려줘" "테슬라 -5% 되면 팔아" 같은 자연어 요청을 kis 명령으로 매핑해 실행한다.

## 아키텍처 요약

```
src/
├── api/                    # 338개 API 바인딩 (OAuth/국내주식/해외주식/채권/선물옵션)
│   └── {domain}/{category}/{endpoint}.rs
├── client.rs               # KisClient — OAuth·HTTP·hashkey
├── token.rs                # 토큰 발급·캐시
├── rate_limit.rs           # 파일락 기반 프로세스간 TPS 제한
├── ws.rs                   # WebSocket 스트리밍 (domestic/overseas/night-futures)
├── symbols/                # 마스터 파일 다운로드·파싱·SQLite FTS5
├── analysis/               # 기술적 지표 순수함수 (MA/EMA/RSI/MACD/BB/Ichimoku/OBV)
├── commands/               # CLI 서브커맨드 핸들러
│   ├── stock/{dome,usa}.rs
│   ├── fo/{dome,usa}.rs
│   ├── bond.rs · auth.rs · symbols.rs · analyze.rs · skill.rs
│   ├── installer.rs        # kis install / kis update
│   ├── backtest.rs         # 7개 전략 + 스윕 + 차트 뷰어
│   ├── signal_watch.rs     # cron 기반 시그널 로거
│   └── stop_loss.rs        # 자동 손절 데몬 (폴링/WebSocket)
├── viewer/                 # wry 기반 네이티브 차트 창 (IPC로 KIS 재호출)
└── main.rs                 # clap 서브커맨드 디스패치
```

## 주의

- 이 CLI는 **실제 주문을 집행**한다. `order buy/sell`은 확인 프롬프트 없이 즉시 전송됨. 공부용이면 `is_mock = true`로.
- `stop-loss run --execute` 도 실주문이다. 반드시 `--execute` 빼고 dry-run부터 돌려볼 것.
- API 키는 `config.toml`에 평문 저장. 파일 권한 관리 필수.
- **모의투자 미지원 API** (해외주식·채권·해외선물·야간·예약주문 등)는 실전 전용 — 호출 시 에러 반환.
- 당일 주식주문 TR_ID는 매수/매도 × 실전/모의 4종 분기 (자동 처리).
- `signal-watch`는 주문을 내지 않는다 — 로그만 남긴다. 자동매매를 원하면 직접 `stop-loss`나 `order`로 연결.

## 라이선스

MIT 추정 (명시 필요 시 추가).
