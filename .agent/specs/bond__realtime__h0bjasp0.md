<!-- endpoint: /tryitout/H0BJASP0 -->
<!-- category: [장내채권] 실시간시세 -->
<!-- korean_name: 일반채권 실시간호가 -->

# 일반채권 실시간호가 [실시간-053]

## Info
- **Method**: POST
- **URL**: /tryitout/H0BJASP0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 모의투자 미지원
- **실전TRID**: H0BJCNT0
- **모의TRID**: 모의투자 미지원

## 개요
일반채권 실시간호가 API입니다.
[참고자료]
실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
채권 종목코드 마스터파일은 "포럼 > FAQ > 종목정보 다운로드(국내) > 장내채권 - 채권코드" 참고 부탁드립니다.
[호출 데이터]
헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
[응답 데이터]
1. 정상 등록 여부 (JSON)
- JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
- JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
- JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
2. 실시간 결과 응답 ( | 로 구분되는 값)
ex) 0|H0STCNT0|004|005930^123929^73100^5^...
- 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
- TR_ID : 등록한 tr_id (ex. H0STCNT0)
- 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
- 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓 접속키 | String | Y | 36 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 / P : 개인 |
| custtype | 등록/해제 | String | Y | 1 | 1: 등록, 2:해제 |
| content-type | 컨텐츠타입 | String | Y | 20 | utf-8 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 2 | H0BJCNT0 |
| tr_key | 구분값 | String | Y | 12 | 채권 종목코드 (ex. KR103502GA34) |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| STND_ISCD | 표준종목코드 | String | Y | 12 |  |
| STCK_CNTG_HOUR | 주식체결시간 | String | Y | 6 |  |
| ASKP_ERT1 | 매도호가수익률 | String | Y | 10 |  |
| BIDP_ERT1 | 매수호가수익률1 | String | Y | 10 |  |
| ASKP1 | 매도호가1 | String | Y | 8 |  |
| BIDP1 | 매수호가1 | String | Y | 8 |  |
| ASKP_RSQN1 | 매도호가잔량1 | String | Y | 8 |  |
| BIDP_RSQN1 | 매수호가잔량1 | String | Y | 8 |  |
| ASKP_ERT2 | 매도호가수익률2 | String | Y | 10 |  |
| BIDP_ERT2 | 매수호가수익률2 | String | Y | 10 |  |
| ASKP2 | 매도호가2 | String | Y | 8 |  |
| BIDP2 | 매수호가2 | String | Y | 8 |  |
| ASKP_RSQN2 | 매도호가잔량2 | String | Y | 8 |  |
| BIDP_RSQN2 | 매수호가잔량2 | String | Y | 8 |  |
| ASKP_ERT3 | 매도호가수익률3 | String | Y | 10 |  |
| BIDP_ERT3 | 매수호가수익률3 | String | Y | 10 |  |
| ASKP3 | 매도호가3 | String | Y | 8 |  |
| BIDP3 | 매수호가3 | String | Y | 8 |  |
| ASKP_RSQN3 | 매도호가잔량3 | String | Y | 8 |  |
| BIDP_RSQN3 | 매수호가잔량3 | String | Y | 8 |  |
| ASKP_ERT4 | 매도호가수익률4 | String | Y | 10 |  |
| BIDP_ERT4 | 매수호가수익률4 | String | Y | 10 |  |
| ASKP4 | 매도호가4 | String | Y | 8 |  |
| BIDP4 | 매수호가4 | String | Y | 8 |  |
| ASKP_RSQN4 | 매도호가잔량4 | String | Y | 8 |  |
| BIDP_RSQN4 | 매수호가잔량4 | String | Y | 8 |  |
| ASKP_ERT5 | 매도호가수익률5 | String | Y | 10 |  |
| BIDP_ERT5 | 매수호가수익률5 | String | Y | 10 |  |
| ASKP5 | 매도호가5 | String | Y | 8 |  |
| BIDP5 | 매수호가5 | String | Y | 8 |  |
| ASKP_RSQN52 | 매도호가잔량5 | String | Y | 8 |  |
| BIDP_RSQN53 | 매수호가잔량5 | String | Y | 8 |  |
| TOTAL_ASKP_RSQN | 총매도호가잔량 | String | Y | 8 |  |
| TOTAL_BIDP_RSQN | 총매수호가잔량 | String | Y | 8 |  |
