---
name: kis
description: "한국투자증권(KIS) CLI 도구 `kis-cli`로 주식/채권/선물옵션 조회·주문·실시간 스트리밍·기술적 분석을 수행한다. 다음 상황에서 사용: (1) '삼성전자 현재가' '테슬라 시세' 등 주식 시세 질의 (2) '내 잔고' '보유 종목' '최근 주문 내역' 계좌 조회 (3) '매수/매도' '주문' 등 매매 요청 (필수: 사용자 재확인) (4) '차트 보여줘' '분석해줘' 'RSI' 'MACD' '일목' '볼린저' 등 기술적 분석 (5) '실시간' 'watch' '호가' 실시간 스트리밍 (6) '종목 검색' 'TSLA 코드' 종목 마스터 조회. 종목은 이름 또는 코드 모두 가능 — 자동 해석됨."
allowed-tools: [Bash, Read]
---

# kis-cli 사용 가이드

한국투자증권 Open API를 래핑한 Rust CLI. 실제 계좌에 접근·주문할 수 있으므로 **주문 계열은 사용자 재확인** 후 실행한다.

바이너리 경로 확인: `which kis-cli` 또는 `which kis` — 없으면 `~/dev/rust/kis-cli/target/release/kis-cli`.

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

## 핵심 규칙

1. **종목 인자는 이름 또는 코드 둘 다 OK** — 내부에서 `symbols` DB(FTS5)로 자동 해석. 복수 매칭 시 에러 → `--pick N` 추가.
2. **DB 없으면 sync 필요**: `'일치하는 종목 없음'` 에러 시 `kis symbols sync` 먼저 실행 (약 3-5초).
3. **`--json`**: `analyze` 서브커맨드의 구조화 출력. 저(Claude)가 지표 해석하려면 `--chart --json`을 권장 — 사용자는 차트창 보고, 나는 JSON stdout 받아 해석.
4. **모의투자 미지원 API**: 해외주식·채권·해외선물·야간선물·예약주문 — `is_mock = true`면 에러. 사용자에게 실전 계좌 여부 확인.
5. **주문 계열 (`order buy/sell/cancel`, `bond order`, `fo order`)**: 즉시 체결된다. **실행 전 반드시 사용자 확인 메시지로 수량·가격·종목 재확인**. 자율 판단으로 넣지 않는다.
6. **실시간 (`watch`, `watch-night`)**: 프로세스가 장기 실행된다. Bash run_in_background=true로 띄우지 말고, 사용자가 직접 실행하도록 안내.

## 전체 커맨드 트리

```
kis auth                              # 토큰 발급/확인
kis config {init, path}               # 설정 파일
kis symbols {sync, find}              # 종목 마스터
kis stock dome {price, chart, asking, balance, psbl, order {buy/sell/cancel}, history, watch}
kis stock usa  {price, chart, balance, order {buy/sell}, history, watch}
kis bond       {price, chart, balance, order {buy/sell}}        # 실전 전용
kis fo dome    {price, balance, order {buy/sell}, watch-night}
kis fo usa     {price, balance, order {buy/sell}}               # 실전 전용
kis analyze    [--usa] [--pick N] [--json] [--chart] [--save PATH]
kis skill      {install, uninstall, path}
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

## JSON 리포트 스키마 (참고)

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
- `모의투자 미지원 API입니다` → 사용자에게 실전 계좌 필요 설명
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
```
