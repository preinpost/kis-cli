# kis-cli

한국투자증권(KIS) Open API를 Rust로 래핑한 CLI. 시세 조회·주문·실시간 스트리밍·기술적 분석에 쓸 수 있는 터미널 도구와, TradingView Lightweight Charts 기반 네이티브 차트 뷰어를 제공한다.

- **API 바인딩 338개** 구현 (OAuth · 국내주식 · 해외주식 · 장내채권 · 국내선물옵션 · 해외선물옵션)
- **CLI 서브커맨드 35개+** — 자주 쓰는 조회/주문에 노출
- **종목 이름 자동 해석** — `삼성전자` → `005930`, `Tesla` → `TSLA`
- **WebSocket 실시간 스트리밍** — 국내주식·해외주식·KRX 야간선물
- **기술적 분석** — MA/RSI/MACD/볼린저/일목균형표 로컬 계산
- **네이티브 차트 뷰어 (wry)** — 일/주/월 전환, 무한 스크롤, 심볼 검색, 구름 채색

## 설치

```bash
git clone <repo> kis-cli
cd kis-cli
cargo build --release
# 바이너리: ./target/release/kis   (PATH에 추가하거나 ~/bin 등에 복사)
```

macOS 기본 빌드 (WKWebView 내장). Linux는 `libwebkit2gtk-4.1-dev`, Windows는 WebView2 런타임 필요.

## 설정

```bash
kis config init           # APP_KEY / APP_SECRET / 계좌번호 입력
kis config path           # 설정 파일 경로 확인
kis auth                  # 토큰 발급 + WebSocket approval key 확인
```

설정 파일: `~/Library/Application Support/kis-cli/config.toml` (macOS 기준).

```toml
[credentials]
app_key = "..."
app_secret = "..."
account_number = "12345678-01"
is_mock = false   # true면 모의투자 서버 사용
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

그리고 `kis skill install` 한 번만 실행하면 Claude Code가 kis 사용법을 바로 알고 응답한다 (아래).

## Claude 스킬 설치

```bash
kis skill install          # ~/.claude/skills/kis/SKILL.md 배포
kis skill install --force  # 덮어쓰기
kis skill path             # 설치 경로 확인
kis skill uninstall        # 삭제
```

설치 후 Claude Code에서 "삼성전자 분석해줘" "내 잔고 확인해줘" 같은 자연어 요청을 kis 명령으로 매핑해 실행한다.

## 아키텍처 요약

```
src/
├── api/                    # 338개 API 바인딩 (OAuth/국내주식/해외주식/채권/선물옵션)
│   └── {domain}/{category}/{endpoint}.rs
├── client.rs               # KisClient — OAuth·HTTP·hashkey
├── token.rs                # 토큰 발급·캐시
├── ws.rs                   # WebSocket 스트리밍 (domestic/overseas/night-futures)
├── symbols/                # 마스터 파일 다운로드·파싱·SQLite FTS5
├── analysis/               # 기술적 지표 순수함수 (MA/EMA/RSI/MACD/BB/Ichimoku)
├── commands/               # CLI 서브커맨드 핸들러
│   ├── stock/{dome,usa}.rs
│   ├── fo/{dome,usa}.rs
│   ├── bond.rs · auth.rs · symbols.rs · analyze.rs
├── viewer/                 # wry 기반 네이티브 차트 창 (IPC로 KIS 재호출)
└── main.rs                 # clap 서브커맨드 디스패치
```

## 주의

- 이 CLI는 **실제 주문을 집행**한다. `order buy/sell`은 확인 프롬프트 없이 즉시 전송됨. 공부용이면 `is_mock = true`로.
- API 키는 `config.toml`에 평문 저장. 파일 권한 관리 필수.
- **모의투자 미지원 API** (해외주식·채권·해외선물·야간·예약주문 등)는 실전 전용 — 호출 시 에러 반환.
- 당일 주식주문 TR_ID는 매수/매도 × 실전/모의 4종 분기 (자동 처리).

## 라이선스

MIT 추정 (명시 필요 시 추가).
