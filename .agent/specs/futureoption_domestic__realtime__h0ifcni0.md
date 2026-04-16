<!-- endpoint: /tryitout/H0IFCNI0 -->
<!-- category: [국내선물옵션] 실시간시세 -->
<!-- korean_name: 선물옵션 실시간체결통보 -->

# 선물옵션 실시간체결통보[실시간-012]

## Info
- **Method**: POST
- **URL**: /tryitout/H0IFCNI0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: ws://ops.koreainvestment.com:31000
- **실전TRID**: H0IFCNI0
- **모의TRID**: H0IFCNI9

## 개요
[참고자료]
실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓접속키 | String | Y | 36 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| tr_type | 등록/해제 | String | Y | 1 | 1: 등록, 2:해제 |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 / P : 개인 |
| content-type | 컨텐츠타입 | String | Y | 20 | utf-8 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 7 | [실전투자]H0IFCNI0 : 실시간 선물옵션 체결통보[모의투자]H0IFCNI9 : 실시간 선물옵션 체결통보 |
| tr_key | 코드 | String | Y | 6 | 예:101S12 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CUST_ID | 고객 ID | Array | Y | 16 | '각 항목사이에는 구분자로 ^ 사용,모든 데이터타입은 String으로 변환되어 push 처리됨' |
| ACNT_NO | 계좌번호 | String | Y | 16 |  |
| ODER_NO | 주문번호 | String | Y | 1 |  |
| OODER_NO | 원주문번호 | String | Y | 8 |  |
| SELN_BYOV_CLS | 매도매수구분 | String | Y | 8 | 01:매도, 02매수 |
| RCTF_CLS | 정정구분 | String | Y | 8 |  |
| ODER_KIND2 | 주문종류2 | String | Y | 8 | L: 주문접수통보, 0: 체결통보 |
| STCK_SHRN_ISCD | 주식 단축 종목코드 | String | Y | 8 |  |
| CNTG_QTY | 체결 수량 | String | Y | 8 |  |
| CNTG_UNPR | 체결단가 | String | Y | 8 |  |
| STCK_CNTG_HOUR | 주식 체결 시간 | String | Y | 8 |  |
| RFUS_YN | 거부여부 | String | Y | 8 |  |
| CNTG_YN | 체결여부 | String | Y | 8 | 1: 주문,정정,취소,거부 통보, 2 체결 |
| ACPT_YN | 접수여부 | String | Y | 8 | 1:주문접수, 2:확인, 3, 취소 |
| BRNC_NO | 지점번호 | String | Y | 8 |  |
| ODER_QTY | 주문수량 | String | Y | 8 |  |
| ACNT_NAME | 계좌명 | String | Y | 8 |  |
| CNTG_ISNM | 체결종목명 | String | Y | 8 |  |
| ODER_COND | 주문조건 | String | Y | 8 |  |
| ORD_GRP | 주문그룹ID | String | Y | 8 |  |
| ORD_GRPSEQ | 주문그룹SEQ | String | Y | 8 |  |
| ORDER_PRC | 주문가격 | String | Y | 8 |  |
