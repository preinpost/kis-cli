---
name: kis
description: "한국투자증권(KIS) CLI 도구 `kis-cli`로 주식/채권/선물옵션 조회·주문·실시간 스트리밍·기술적 분석·백테스트·자동 손절을 수행한다. 다음 상황에서 사용: (1) '삼성전자 현재가' '테슬라 시세' 등 주식 시세 질의 (2) '내 잔고' '보유 종목' '최근 주문 내역' 계좌 조회 (3) '매수/매도' '주문' 등 매매 요청 (필수: 사용자 재확인) (4) '차트 보여줘' '분석해줘' 'RSI' 'MACD' '일목' '볼린저' 등 기술적 분석 (5) '백테스트' '전략 돌려봐' 'MA 교차' '파라미터 스윕' 등 과거 시뮬레이션 (6) '손절매 걸어줘' '자동 매도' '-5% 되면 팔아' 등 자동 손절 데몬 (7) '신호 감시' '전략 신호 알려줘' 등 cron 기반 감시 (8) '실시간' 'watch' '호가' 실시간 스트리밍 (9) '종목 검색' 'TSLA 코드' 종목 마스터 조회. 종목은 이름 또는 코드 모두 가능 — 자동 해석됨."
allowed-tools: [Bash, Read]
---

# kis-cli 사용 가이드

한국투자증권 Open API를 래핑한 Rust CLI. 실제 계좌에 접근·주문·자동 매도할 수 있으므로 **주문·손절 계열은 사용자 재확인** 후 실행한다.

바이너리 경로 확인: `which kis` 또는 `which kis-cli` — 없으면 `~/dev/rust/kis-cli/target/release/kis`.

## 자연어 → 커맨드 매핑

| 사용자 발화 | 실행 |
|---|---|
| "삼성전자 시세" | `kis stock dome price 삼성전자 --pick 1` |
| "테슬라 얼마야" | `kis stock usa price Tesla --pick 1` |
| "내 잔고" | `kis stock dome balance` |
| "해외 잔고" | `kis stock usa balance --exchange NASD` (NYSE/AMEX도 필요 시) |
| "최근 주문 내역" | `kis stock dome history` |
| "삼성전자 분석해줘" | `kis analyze 삼성전자 --pick 1 --chart --json` (창 띄우고 JSON 받아 해석) |
| "테슬라 차트" | `kis analyze Tesla --usa --pick 1 --chart` |
| "삼성전자 실시간" | `kis stock dome watch 삼성전자 --pick 1` (사용자가 Ctrl+C로 종료) |
| "테슬라 실시간" | `kis stock usa watch Tesla --pick 1` |
| "삼성전자 호가" | `kis stock dome asking 삼성전자 --pick 1` |
| "삼성전자 코드" | `kis symbols find 삼성전자 --limit 5` |
| "TSLA 뭐야" | `kis symbols find TSLA --limit 5` |
| "매수가능 얼마" | `kis stock dome psbl` |
| "KOSPI200 선물 시세" | `kis fo dome price 1A01606` (사용자 확인 후 월물 결정) |
| "삼성전자 MA 교차 백테스트" | `kis backtest ma-cross 삼성전자 --pick 1` |
| "테슬라 RSI 백테스트" | `kis backtest rsi Tesla --usa --pick 1` |
| "전략 바꿔가며 테스트" / "백테스트 차트" | `kis backtest chart 삼성전자 --pick 1` (GUI, 사용자 실행) |
| "파라미터 최적화" / "스윕" | `kis backtest ma-cross 삼성전자 --pick 1 --sweep --json` |
| "신호 감시 돌려줘" | `kis signal-watch 삼성전자 --pick 1 --strategy ma-cross` (장기 실행, 사용자) |
| "자동 손절 걸어줘" / "-5% 되면 팔아" | **먼저** `kis stop-loss run --threshold -5.0` (dry-run). 재확인 후 `--execute` 추가 |
| "손절 데몬 상태" | `kis stop-loss status` |

## 핵심 규칙

1. **종목 인자는 이름 또는 코드 둘 다 OK** — 내부에서 `symbols` DB(FTS5)로 자동 해석. 복수 매칭 시 에러 → `--pick N` 추가.
2. **DB 없으면 sync 필요**: `'일치하는 종목 없음'` 에러 시 `kis symbols sync` 먼저 실행 (약 3-5초). 반복 호출은 `--if-stale`로 24시간 이내 스킵.
3. **`--json`**: `analyze`·`backtest` 서브커맨드 구조화 출력. Claude가 지표 해석하려면 `analyze --chart --json` — 사용자는 창을 보고, 나는 JSON stdout 받아 해석.
4. **모의투자 미지원 API**: 해외주식·채권·해외선물·야간선물·예약주문 — `is_mock = true`면 에러. 사용자에게 실전 계좌 여부 확인.
5. **주문 계열** (`stock … order buy/sell/cancel`, `bond order`, `fo order`): 즉시 체결된다. **실행 전 반드시 사용자 확인 메시지로 수량·가격·종목 재확인**. 자율 판단으로 넣지 않는다.
6. **자동 손절** (`stop-loss run --execute`): 데몬이 임계치 도달 시 **실제 매도 집행** (국내 시장가, 해외 공격적 지정가). 기본은 dry-run. `--execute` 붙이려면 임계치·대상 종목·실거래 여부를 사용자에게 반드시 재확인하고, 상태 점검은 `kis stop-loss status`로.
7. **`signal-watch`는 주문 없음** — cron 스케줄로 전략 신호를 **로그만** 남긴다. 자동 매매 아님.
8. **장기 실행 프로세스** (`watch`, `watch-night`, `signal-watch`, `stop-loss run`, `analyze --chart`, `backtest chart`): Bash `run_in_background=true`로 띄우지 말고 **사용자가 직접 터미널에서 실행**하도록 안내. `--chart` 계열은 macOS AppKit 때문에 GUI 창이 사용자 세션에서 열려야 한다.

## 전체 커맨드 트리

```
kis auth                                        # 토큰 발급/확인
kis config {init, path}                         # 설정 파일
kis symbols {sync [--if-stale], find}           # 종목 마스터

kis stock dome {price, chart, asking, balance, psbl, order {buy/sell/cancel}, history, watch}
kis stock usa  {price, chart, balance, order {buy/sell}, history, watch}
kis bond       {price, chart, balance, order {buy/sell}}        # 실전 전용
kis fo dome    {price, balance, order {buy/sell}, watch-night}
kis fo usa     {price, balance, order {buy/sell}}               # 실전 전용

kis analyze <symbol> [--usa] [--pick N] [--json] [--chart] [--save PATH]

kis backtest <strategy> <symbol>
   strategy: ma-cross | rsi | macd | bollinger | ichimoku | obv | manual | chart
   공통: [--usa] [--period D|W|M] [--from YYYYMMDD] [--to YYYYMMDD]
         [--fee-bps 5.0] [--slippage-bps 0.0] [--allow-short] [--leverage 1.0]
         [--stop-loss-pct N] [--take-profit-pct N] [--sweep] [--json] [--pick N]
   전략별:
     ma-cross  --fast 20 --slow 60
     rsi       --rsi-period 14 --rsi-oversold 30 --rsi-overbought 70
     bollinger --bb-period 20 --bb-sigma 2.0
     obv       --obv-period 20
     manual    --entry-date YYYYMMDD [--exit-date YYYYMMDD] [--direction long|short]
     chart     GUI (인터랙티브 — 전략·파라미터를 창에서 변경)

kis signal-watch <symbol> [--strategy ma-cross|rsi|macd|bollinger|ichimoku|obv]
                          [--cron "0 20 15 * * Mon-Fri"] [--period D|W|M]
                          [--fast --slow / --rsi-period --rsi-oversold --rsi-overbought /
                           --bb-period --bb-sigma / --obv-period] [--pick N]

kis stop-loss run   [--threshold -5.0] [--interval 30] [--symbols A,B,C]
                    [--execute] [--usa-spread 1.0] [--ws]
kis stop-loss {status, path}

kis skill {install [--force], uninstall, path}
kis install [--force]                           # cargo install (바이너리) + Claude 스킬
kis update  [--no-pull]                         # git pull + cargo install --force + 스킬 갱신
```

## 분석 워크플로 (Claude 협업)

사용자가 "XXX 분석해줘"라고 하면:

1. `kis analyze <name> --pick 1 --chart --json` 실행 (해외면 `--usa` 추가)
2. stdout JSON 전체 받아 파싱
3. 주요 지표 요약:
   - **추세**: MA 배열 + 일목 구름 위치 + 후행스팬
   - **모멘텀**: RSI 구간 + MACD 히스토그램 방향/크로스
   - **변동성**: 볼린저 %B + 밴드 폭
   - **시그널**: `report.signals` 배열 그대로 인용 + 맥락 해설
4. 결론 — 지지/저항 가격대 + 단기 리스크 언급
5. 사용자가 차트창을 동시에 보고 있으므로, 숫자 나열보다 "왜 그 가격대인지" 맥락 설명에 집중

## 백테스트 워크플로

사용자가 "백테스트 해줘"라고 할 때:

1. 전략이 명시됐으면 해당 서브커맨드 (`ma-cross` / `rsi` / `macd` / `bollinger` / `ichimoku` / `obv`). 애매하면 무엇으로 돌릴지 먼저 확인.
2. `--json`으로 받아 주요 지표 요약: **CAGR / Sharpe / Sortino / Calmar / MDD / 승률 / 트레이드 수**
3. "파라미터 최적화"·"스윕"이면 `--sweep` — Sharpe 내림차순 상위 15개 결과 요약
4. 인터랙티브 비교는 `kis backtest chart <symbol>` → GUI (사용자가 직접 실행)

### 백테스트 기본 가정
- 체결가는 해당 봉의 **종가** (인트라바 경로 무시)
- 수수료·슬리피지는 bps 단위 (기본 fee 5bps = 0.05%)
- `--stop-loss-pct` / `--take-profit-pct`도 종가 터치 시에만 트리거
- 레버리지는 수익률·수수료에 승수 (증거금·청산 모델 없음)
- 최소 30봉 필요 — 부족하면 "데이터 부족" 에러

## 자동 손절 (`stop-loss`)

데몬이 주기적으로 잔고를 조회해서 평가손익률이 임계치 이하인 종목을 매도한다. 같은 세션에서 한 번 매도된 종목은 재트리거 안 함.

```bash
# 1) dry-run으로 동작 확인 (주문 안 나감, 로그만)
kis stop-loss run --threshold -5.0 --interval 30
# 2) 다른 터미널에서 상태 확인
kis stop-loss status
# 3) 검증 끝나면 실거래 (사용자 재확인 후)
kis stop-loss run --threshold -5.0 --ws --execute
```

- `--ws`: 폴링 대신 WebSocket tick 단위 감시 (반응 빠름).
- `--symbols A,B,C`: 쉼표 구분 코드/이름 일부 필터 (미지정 시 전체 잔고).
- 국내 → 시장가, 해외 → 현재가에 `--usa-spread`(%) 적용한 공격적 지정가.
- 상태 파일: `kis stop-loss path`.

## 신호 감시 (`signal-watch`)

cron 스케줄에 맞춰 전략 신호를 로그로 남기는 **감시 전용** 도구 (주문 없음). 장기 실행이므로 사용자가 직접 실행.

```bash
# 평일 15:20 (장 마감 10분 전) MA 교차 체크 (기본 cron)
kis signal-watch 삼성전자 --pick 1

# 6필드 cron (초 분 시 일 월 요일) + RSI 전략
kis signal-watch 삼성전자 --pick 1 --cron "0 0 9 * * Mon-Fri" --strategy rsi
```

## JSON 리포트 스키마 (analyze)

```jsonc
{
  "symbol": "005930", "name": "삼성전자",
  "date": "20260417", "current_price": 216000, "bars": 100,
  "ma": { "ma5": 210400, "ma20": 194230, "ma60": 183593,
          "ma120": null, "alignment": "정배열" },
  "rsi": { "period": 14, "value": 61.64, "state": "상승 우세" },
  "macd": { "macd": 7740.7, "signal": 5424.34, "histogram": 2316.36,
            "cross": "상승 우세" },
  "bollinger": { "upper": 222188, "middle": 194230, "lower": 166272,
                 "percent_b": 0.89, "bandwidth_pct": 28.79 },
  "ichimoku": { "tenkan": 205200, "kijun": 192500,
                "senkou_a_now": 190925, "senkou_b_now": 166150,
                "cloud_color": "양운", "price_vs_cloud": "구름 위",
                "chikou_signal": "양전환" },
  "signals": ["5일선 > 20일선 (단기 상승)", "일목 양운 위 — 강세"]
}
```

## 에러 처리

- `설정 파일을 읽을 수 없습니다` → `kis config init` 안내
- `일치하는 종목 없음` → `kis symbols sync` 먼저 안내
- `복수 매칭(N)이나 TTY 아님` → `--pick 1` (또는 특정 번호) 추가
- `모의투자 미지원 API입니다` → 실전 계좌 필요 설명
- `데이터 부족 (N봉) — 백테스트에 최소 30봉` → `--from`/`--to` 조정 또는 주기(`--period`) 변경
- KIS API 오류 `[코드]: 메시지` → 메시지를 그대로 사용자에게 전달. rate limit이면 잠시 대기.

## 자주 쓰는 조합

```bash
# 잔고 + 최근 활동 한 번에
kis stock dome balance && echo "---" && kis stock dome history

# 분석 + 차트 같이 (Claude 해석용)
kis analyze 삼성전자 --pick 1 --chart --json

# 해외 잔고 3거래소 통합
for ex in NASD NYSE AMEX; do kis stock usa balance --exchange $ex; done

# 심볼 먼저 확인 후 정확한 코드로 주문
kis symbols find 삼성전자 --limit 3
# 확인 후 → kis stock dome order buy 005930 1 --price 75000

# 파라미터 스윕으로 최적 조합 탐색 → 상위 5개만
kis backtest ma-cross 삼성전자 --pick 1 --sweep --json | jq '.sweep[0:5]'

# 분석 → 매수 후보면 손절 dry-run으로 임계치 탐색
kis analyze 005930 --json | jq '.rsi, .bollinger'
kis stop-loss run --threshold -3.0 --symbols 005930 --ws   # 먼저 dry-run
```
