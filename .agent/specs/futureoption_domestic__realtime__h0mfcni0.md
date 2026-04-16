<!-- endpoint: /tryitout/H0MFCNI0 -->
<!-- category: [국내선물옵션] 실시간시세 -->
<!-- korean_name: KRX야간선물 실시간체결통보 -->

# KRX야간선물 실시간체결통보 [실시간-066]

## Info
- **Method**: POST
- **URL**: /tryitout/H0MFCNI0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 모의투자 미지원
- **실전TRID**: H0MFCNI0
- **모의TRID**: 모의투자 미지원

## 개요
[참고자료]
실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info

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
| tr_id | 거래ID | String | Y | 2 | H0MFCNI0 |
| tr_key | 구분값 | String | Y | 12 | HTS ID |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CUST_ID | 고객 ID | String | Y | 8 |  |
| ACNT_NO | 계좌번호 | String | Y | 10 |  |
| ODER_NO | 주문번호 | String | Y | 10 |  |
| OODER_NO | 원주문번호 | String | Y | 10 |  |
| SELN_BYOV_CLS | 매도매수구분 | String | Y | 2 |  |
| RCTF_CLS | 정정구분 | String | Y | 1 |  |
| ODER_KIND2 | 주문종류2 | String | Y | 1 |  |
| STCK_SHRN_ISCD | 주식 단축 종목코드 | String | Y | 9 |  |
| CNTG_QTY | 체결 수량 | String | Y | 10 |  |
| CNTG_UNPR | 체결단가 | String | Y | 9 |  |
| STCK_CNTG_HOUR | 주식 체결 시간 | String | Y | 6 |  |
| RFUS_YN | 거부여부 | String | Y | 1 |  |
| CNTG_YN | 체결여부 | String | Y | 1 |  |
| ACPT_YN | 접수여부 | String | Y | 1 |  |
| BRNC_NO | 지점번호 | String | Y | 5 |  |
| ODER_QTY | 주문수량 | String | Y | 9 |  |
| ACNT_NAME | 계좌명 | String | Y | 12 |  |
| CNTG_ISNM | 체결종목명 | String | Y | 14 |  |
| ODER_COND | 주문조건 | String | Y | 1 |  |
