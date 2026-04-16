<!-- endpoint: /uapi/overseas-stock/v1/trading/algo-ordno -->
<!-- category: [해외주식] 주문/계좌 -->
<!-- korean_name: 해외주식 지정가주문번호조회 -->

# 해외주식 지정가주문번호조회 [해외주식-071]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-stock/v1/trading/algo-ordno
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: TTTS6058R
- **모의TRID**: 모의투자 미지원
- **Format**: JSON
- **Content-Type**: application/json; charset=utf-8

## 개요
TWAP, VWAP 주문에 대한 주문번호를 조회하는 API

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용)법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | Y | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | TTTS6058R |
| tr_cont | 연속거래여부 | String | N | 1 | 공백 : 초기 조회N : 다음 데이터 조회 (output header의 tr_cont가 M일 경우) |
| custtype | 고객타입 | String | Y | 1 | B : 법인P : 개인 |
| seq_no | 일련번호 | String | N | 3 | [법인 필수] 001 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | IP주소 | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| TRAD_DT | 거래일자 | String | Y | 8 | YYYYMMDD |
| CANO | 계좌번호 | String | Y | 8 | 종합계좌번호 (8자리) |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌상품코드 (2자리) : 주식계좌는 01 |
| CTX_AREA_NK200 | 연속조회키200 | String | N | 200 |  |
| CTX_AREA_FK200 | 연속조회조건200 | String | N | 200 |  |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 |  |
| tr_cont | 연속거래여부 | String | N | 1 | F or M : 다음 데이터 있음D or E : 마지막 데이터 |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| output | 응답상세 | Object Array | Y | - |  |
| odno | 주문번호 | String | Y | 10 |  |
| trad_dvsn_name | 매매구분명 | String | Y | 60 |  |
| pdno | 상품번호 | String | Y | 12 |  |
| item_name | 종목명 | String | Y | 60 |  |
| ft_ord_qty | FT주문수량 | String | Y | 4 |  |
| ft_ord_unpr3 | FT주문단가 | String | Y | 8 |  |
| splt_buy_attr_name | 분할매수속성명 | String | Y | 60 |  |
| ft_ccld_qty | FT체결수량 | String | Y | 4 |  |
| ord_gno_brno | 주문채번지점번호 | String | N | 5 |  |
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 |  |
| msg1 | 응답메세지 | String | Y | 80 |  |
| ctx_area_fk200 | 연속조회검색조건200 | String | Y | 200 |  |
| ctx_area_nk200 | 연속조회키200 | String | Y | 200 |  |
