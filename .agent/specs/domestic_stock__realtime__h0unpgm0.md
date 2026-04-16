<!-- endpoint: /tryitout/H0UNPGM0 -->
<!-- category: [국내주식] 실시간시세 -->
<!-- korean_name: 국내주식 실시간프로그램매매 (통합) -->

# 국내주식 실시간프로그램매매 (통합)

## Info
- **Method**: POST
- **URL**: /tryitout/H0UNPGM0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 모의투자 미지원
- **실전TRID**: H0UNPGM0
- **모의TRID**: 모의투자 미지원

## 개요
요청

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓 접속키 | String | N | 286 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| custtype | 고객타입 | String | N | 1 | 'B : 법인P : 개인' |
| tr_type | 거래타입 | String | N | 1 | '1 : 등록2 : 해제' |
| content-type | 컨텐츠타입 | String | N | 1 | ' utf-8' |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 2 | H0UNPGM0 : 실시간 주식종목프로그램매매 통합 |
| tr_key | 구분값 | String | Y | 12 | 종목코드 (ex 005930 삼성전자) |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| MKSC_SHRN_ISCD | 유가증권 단축 종목코드 | String | Y | 9 |  |
| STCK_CNTG_HOUR | 주식 체결 시간 | String | Y | 6 |  |
| SELN_CNQN | 매도 체결량 | String | Y | 8 |  |
| SELN_TR_PBMN | 매도 거래 대금 | String | Y | 8 |  |
| SHNU_CNQN | 매수2 체결량 | String | Y | 8 |  |
| SHNU_TR_PBMN | 매수2 거래 대금 | String | Y | 8 |  |
| NTBY_CNQN | 순매수 체결량 | String | Y | 8 |  |
| NTBY_TR_PBMN | 순매수 거래 대금 | String | Y | 8 |  |
| SELN_RSQN | 매도호가잔량 | String | Y | 8 |  |
| SHNU_RSQN | 매수호가잔량 | String | Y | 8 |  |
| WHOL_NTBY_QTY | 전체순매수호가잔량 | String | Y | 8 |  |
