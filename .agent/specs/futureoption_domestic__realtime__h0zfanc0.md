<!-- endpoint: /tryitout/H0ZFANC0 -->
<!-- category: [국내선물옵션] 실시간시세 -->
<!-- korean_name: 주식선물 실시간예상체결 -->

# 주식선물 실시간예상체결 [실시간-031]

## Info
- **Method**: POST
- **URL**: /tryitout/H0ZFANC0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 모의투자 미지원
- **실전TRID**: H0ZFANC0
- **모의TRID**: 모의투자 미지원

## 개요
[참고자료]
실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓 접속키 | String | Y | 36 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 / P : 개인 |
| tr_type | 등록/해제 | String | Y | 1 | 1: 등록, 2:해제 |
| content-type | 컨텐츠타입 | String | Y | 20 | utf-8 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 2 | H0ZFANC0 |
| tr_key | 구분값 | String | Y | 12 | 주식선물 종목코드 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| FUTS_SHRN_ISCD | 선물단축종목코드 | String | Y | 9 |  |
| BSOP_HOUR | 영업시간 | String | Y | 6 |  |
| ANTC_CNPR | 예상체결가 | String | Y | 8 |  |
| ANTC_CNTG_VRSS | 예상체결대비 | String | Y | 8 |  |
| ANTC_CNTG_VRSS_SIGN | 예상체결대비부호 | String | Y | 1 |  |
| ANTC_CNTG_PRDY_CTRT | 예상체결전일대비율 | String | Y | 8 |  |
| ANTC_MKOP_CLS_CODE | 예상장운영구분코드 | String | Y | 3 |  |
| ANTC_CNQN | 예상체결수량 | String | Y | 8 |  |
