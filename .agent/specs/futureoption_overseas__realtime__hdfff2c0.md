<!-- endpoint: /tryitout/HDFFF2C0 -->
<!-- category: [해외선물옵션]실시간시세 -->
<!-- korean_name: 해외선물옵션 실시간체결내역통보 -->

# 해외선물옵션 실시간체결내역통보[실시간-020]

## Info
- **Method**: POST
- **URL**: /tryitout/HDFFF2C0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 모의투자 미지원
- **실전TRID**: HDFFF2C0
- **모의TRID**: 모의투자 미지원
- **Content-Type**: application/json; charset=utf-8

## 개요
요청

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓 접속키 | String | Y | 36 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| tr_type | 등록/해제 | String | Y | 1 | "1: 등록, 2:해제" |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 / P : 개인 |
| content-type | 컨텐츠타입 | String | Y | 20 | utf-8 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 7 | HDFFF2C0 |
| tr_key | HTSID | String | Y | 8 | HTSID |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| USER_ID | 유저ID | Object | Y | 8 | '각 항목사이에는 구분자로 ^ 사용,모든 데이터타입은 String으로 변환되어 push 처리됨' |
| ACCT_NO | 계좌번호 | String | Y | 10 |  |
| ORD_DT | 주문일자 | String | Y | 8 |  |
| ODNO | 주문번호 | String | Y | 10 |  |
| ORGN_ORD_DT | 원주문일자 | String | Y | 8 |  |
| ORGN_ODNO | 원주문번호 | String | Y | 10 |  |
| SERIES | 종목명 | String | Y | 32 |  |
| RVSE_CNCL_DVSN_CD | 정정취소구분코드 | String | Y | 2 | 해당없음 : 00 , 정정 : 01 , 취소 : 02 |
| SLL_BUY_DVSN_CD | 매도매수구분코드 | String | Y | 2 | 01 : 매도, 02 : 매수 |
| CPLX_ORD_DVSN_CD | 복합주문구분코드 | String | Y | 1 | 0 (hedge청산만 이용) |
| PRCE_TP | 가격구분코드 | String | Y | 1 |  |
| FM_EXCG_RCIT_DVSN_CD | FM거래소접수구분코드 | String | Y | 2 |  |
| ORD_QTY | 주문수량 | String | Y | 18 |  |
| FM_LMT_PRIC | FMLIMIT가격 | String | Y | 21 |  |
| FM_STOP_ORD_PRIC | FMSTOP주문가격 | String | Y | 21 |  |
| TOT_CCLD_QTY | 총체결수량 | String | Y | 18 | 동일한 주문건에 대한 누적된 체결수량 (하나의 주문건에 여러건의 체결내역 발생) |
| TOT_CCLD_UV | 총체결단가 | String | Y | 21 |  |
| ORD_REMQ | 잔량 | String | Y | 21 |  |
| FM_ORD_GRP_DT | FM주문그룹일자 | String | Y | 8 |  |
| ORD_GRP_STNO | 주문그룹번호 | String | Y | 12 |  |
| ORD_DTL_DTIME | 주문상세일시 | String | Y | 17 |  |
| OPRT_DTL_DTIME | 조작상세일시 | String | Y | 17 |  |
| WORK_EMPL | 주문자 | String | Y | 8 |  |
| CCLD_DT | 체결일자 | String | Y | 8 |  |
| CCNO | 체결번호 | String | Y | 11 |  |
| API_CCNO | API 체결번호 | String | Y | 20 |  |
| CCLD_QTY | 체결수량 | String | Y | 18 | 매 체결 단위 체결수량임 (여러건 체결내역 누적 체결수량인 총체결수량과 다름) |
| FM_CCLD_PRIC | FM체결가격 | String | Y | 21 |  |
| CRCY_CD | 통화코드 | String | Y | 3 |  |
| TRST_FEE | 위탁수수료 | String | Y | 21 |  |
| ORD_MDIA_ONLINE_YN | 주문매체온라인여부 | String | Y | 1 |  |
| FM_CCLD_AMT | FM체결금액 | String | Y | 21 |  |
| FUOP_ITEM_DVSN_CD | 선물옵션종목구분코드 | String | Y | 2 |  |
