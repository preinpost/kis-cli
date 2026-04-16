<!-- endpoint: /tryitout/H0ZOASP0 -->
<!-- category: [국내선물옵션] 실시간시세 -->
<!-- korean_name: 주식옵션 실시간호가 -->

# 주식옵션 실시간호가 [실시간-045]

## Info
- **Method**: POST
- **URL**: /tryitout/H0ZOASP0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 모의투자 미지원
- **실전TRID**: H0ZOASP0
- **모의TRID**: 모의투자 미지원

## 개요
요청

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓 접속키 | String | Y | 36 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 / P : 개인 |
| tr_type | 등록/해제 | String | Y | 1 | "1: 등록, 2:해제" |
| content-type | 컨텐츠타입 | String | Y | 20 | utf-8 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 7 | H0ZOASP0 |
| tr_key | 종목코드 | String | Y | 6 | 종목코드 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| OPTN_SHRN_ISCD | 옵션단축종목코드 | Object | Y | 9 | '각 항목사이에는 구분자로 ^ 사용,모든 데이터타입은 String으로 변환되어 push 처리됨' |
| BSOP_HOUR | 영업시간 | String | Y | 6 |  |
| OPTN_ASKP1 | 옵션매도호가1 | String | Y | 8 |  |
| OPTN_ASKP2 | 옵션매도호가2 | String | Y | 8 |  |
| OPTN_ASKP3 | 옵션매도호가3 | String | Y | 8 |  |
| OPTN_ASKP4 | 옵션매도호가4 | String | Y | 8 |  |
| OPTN_ASKP5 | 옵션매도호가5 | String | Y | 8 |  |
| OPTN_BIDP1 | 옵션매수호가1 | String | Y | 8 |  |
| OPTN_BIDP2 | 옵션매수호가2 | String | Y | 8 |  |
| OPTN_BIDP3 | 옵션매수호가3 | String | Y | 8 |  |
| OPTN_BIDP4 | 옵션매수호가4 | String | Y | 8 |  |
| OPTN_BIDP5 | 옵션매수호가5 | String | Y | 8 |  |
| ASKP_CSNU1 | 매도호가건수1 | String | Y | 4 |  |
| ASKP_CSNU2 | 매도호가건수2 | String | Y | 4 |  |
| ASKP_CSNU3 | 매도호가건수3 | String | Y | 4 |  |
| ASKP_CSNU4 | 매도호가건수4 | String | Y | 4 |  |
| ASKP_CSNU5 | 매도호가건수5 | String | Y | 4 |  |
| BIDP_CSNU1 | 매수호가건수1 | String | Y | 4 |  |
| BIDP_CSNU2 | 매수호가건수2 | String | Y | 4 |  |
| BIDP_CSNU3 | 매수호가건수3 | String | Y | 4 |  |
| BIDP_CSNU4 | 매수호가건수4 | String | Y | 4 |  |
| BIDP_CSNU5 | 매수호가건수5 | String | Y | 4 |  |
| ASKP_RSQN1 | 매도호가잔량1 | String | Y | 8 |  |
| ASKP_RSQN2 | 매도호가잔량2 | String | Y | 8 |  |
| ASKP_RSQN3 | 매도호가잔량3 | String | Y | 8 |  |
| ASKP_RSQN4 | 매도호가잔량4 | String | Y | 8 |  |
| ASKP_RSQN5 | 매도호가잔량5 | String | Y | 8 |  |
| BIDP_RSQN1 | 매수호가잔량1 | String | Y | 8 |  |
| BIDP_RSQN2 | 매수호가잔량2 | String | Y | 8 |  |
| BIDP_RSQN3 | 매수호가잔량3 | String | Y | 8 |  |
| BIDP_RSQN4 | 매수호가잔량4 | String | Y | 8 |  |
| BIDP_RSQN5 | 매수호가잔량5 | String | Y | 8 |  |
| TOTAL_ASKP_CSNU | 총매도호가건수 | String | Y | 4 |  |
| TOTAL_BIDP_CSNU | 총매수호가건수 | String | Y | 4 |  |
| TOTAL_ASKP_RSQN | 총매도호가잔량 | String | Y | 8 |  |
| TOTAL_BIDP_RSQN | 총매수호가잔량 | String | Y | 8 |  |
| TOTAL_ASKP_RSQN_ICDC | 총매도호가잔량증감 | String | Y | 4 |  |
| TOTAL_BIDP_RSQN_ICDC | 총매수호가잔량증감 | String | Y | 4 |  |
| OPTN_ASKP6 | 옵션매도호가6 | String | Y | 8 |  |
| OPTN_ASKP7 | 옵션매도호가7 | String | Y | 8 |  |
| OPTN_ASKP8 | 옵션매도호가8 | String | Y | 8 |  |
| OPTN_ASKP9 | 옵션매도호가9 | String | Y | 8 |  |
| OPTN_ASKP10 | 옵션매도호가10 | String | Y | 8 |  |
| OPTN_BIDP6 | 옵션매수호가6 | String | Y | 8 |  |
| OPTN_BIDP7 | 옵션매수호가7 | String | Y | 8 |  |
| OPTN_BIDP8 | 옵션매수호가8 | String | Y | 8 |  |
| OPTN_BIDP9 | 옵션매수호가9 | String | Y | 8 |  |
| OPTN_BIDP10 | 옵션매수호가10 | String | Y | 8 |  |
| ASKP_CSNU6 | 매도호가건수6 | String | Y | 4 |  |
| ASKP_CSNU7 | 매도호가건수7 | String | Y | 4 |  |
| ASKP_CSNU8 | 매도호가건수8 | String | Y | 4 |  |
| ASKP_CSNU9 | 매도호가건수9 | String | Y | 4 |  |
| ASKP_CSNU10 | 매도호가건수10 | String | Y | 4 |  |
| BIDP_CSNU6 | 매수호가건수6 | String | Y | 4 |  |
| BIDP_CSNU7 | 매수호가건수7 | String | Y | 4 |  |
| BIDP_CSNU8 | 매수호가건수8 | String | Y | 4 |  |
| BIDP_CSNU9 | 매수호가건수9 | String | Y | 4 |  |
| BIDP_CSNU10 | 매수호가건수10 | String | Y | 4 |  |
| ASKP_RSQN6 | 매도호가잔량6 | String | Y | 8 |  |
| ASKP_RSQN7 | 매도호가잔량7 | String | Y | 8 |  |
| ASKP_RSQN8 | 매도호가잔량8 | String | Y | 8 |  |
| ASKP_RSQN9 | 매도호가잔량9 | String | Y | 8 |  |
| ASKP_RSQN10 | 매도호가잔량10 | String | Y | 8 |  |
| BIDP_RSQN6 | 매수호가잔량6 | String | Y | 8 |  |
| BIDP_RSQN7 | 매수호가잔량7 | String | Y | 8 |  |
| BIDP_RSQN8 | 매수호가잔량8 | String | Y | 8 |  |
| BIDP_RSQN9 | 매수호가잔량9 | String | Y | 8 |  |
| BIDP_RSQN10 | 매수호가잔량10 | String | Y | 8 |  |
