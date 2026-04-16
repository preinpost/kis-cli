<!-- endpoint: /uapi/domestic-futureoption/v1/trading/inquire-daily-amount-fee -->
<!-- category: [국내선물옵션] 주문/계좌 -->
<!-- korean_name: 선물옵션기간약정수수료일별 -->

# 선물옵션기간약정수수료일별[v1_국내선물-017]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-futureoption/v1/trading/inquire-daily-amount-fee
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTFO6119R
- **모의TRID**: 모의투자 미지원

## 개요
선물옵션기간약정수수료일별 API입니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTFO6119R |
| tr_cont | 연속 거래 여부 | String | N | 1 | 공백 : 초기 조회N : 다음 데이터 조회 (output header의 tr_cont가 M일 경우) |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| INQR_STRT_DAY | 조회시작일 | String | Y | 8 | 조회시작일(YYYYMMDD) |
| INQR_END_DAY | 조회종료일 | String | Y | 8 | 조회종료일(YYYYMMDD) |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | Y | 200 | 연속조회검색조건200 |
| CTX_AREA_NK200 | 연속조회키200 | String | Y | 200 | 연속조회키200 |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | N | 1 | F or M : 다음 데이터 있음D or E : 마지막 데이터 |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 |  |
| msg_cd | 응답코드 | String | Y | 8 |  |
| msg1 | 응답메세지 | String | Y | 80 |  |
| output1 | 응답상세 | Array | Y |  | array |
| ord_dt | 주문일자 | String | Y | 8 |  |
| pdno | 상품번호 | String | Y | 12 |  |
| item_name | 종목명 | String | Y | 60 |  |
| sll_agrm_amt | 매도약정금액 | String | Y | 19 |  |
| sll_fee | 매도수수료 | String | Y | 19 |  |
| buy_agrm_amt | 매수약정금액 | String | Y | 19 |  |
| buy_fee | 매수수수료 | String | Y | 19 |  |
| tot_fee_smtl | 총수수료합계 | String | Y | 19 |  |
| trad_pfls | 매매손익 | String | Y | 19 |  |
| output2 | 응답상세2 | Object | Y |  |  |
| futr_agrm | 선물약정 | String | Y | 19 |  |
| futr_agrm_amt | 선물약정금액 | String | Y | 19 |  |
| futr_agrm_amt_smtl | 선물약정금액합계 | String | Y | 19 |  |
| futr_sll_fee_smtl | 선물매도수수료합계 | String | Y | 19 |  |
| futr_buy_fee_smtl | 선물매수수수료합계 | String | Y | 19 |  |
| futr_fee_smtl | 선물수수료합계 | String | Y | 19 |  |
| opt_agrm | 옵션약정 | String | Y | 19 |  |
| opt_agrm_amt | 옵션약정금액 | String | Y | 19 |  |
| opt_agrm_amt_smtl | 옵션약정금액합계 | String | Y | 19 |  |
| opt_sll_fee_smtl | 옵션매도수수료합계 | String | Y | 19 |  |
| opt_buy_fee_smtl | 옵션매수수수료합계 | String | Y | 19 |  |
| opt_fee_smtl | 옵션수수료합계 | String | Y | 19 |  |
| prdt_futr_agrm | 상품선물약정 | String | Y | 19 |  |
| prdt_fuop | 상품선물옵션 | String | Y | 19 |  |
| prdt_futr_evlu_amt | 상품선물평가금액 | String | Y | 8 |  |
| futr_fee | 선물수수료 | String | Y | 19 |  |
| opt_fee | 옵션수수료 | String | Y | 19 |  |
| fee | 수수료 | String | Y | 19 |  |
| sll_agrm_amt | 매도약정금액 | String | Y | 19 |  |
| buy_agrm_amt | 매수약정금액 | String | Y | 19 |  |
| agrm_amt_smtl | 약정금액합계 | String | Y | 19 |  |
| sll_fee | 매도수수료 | String | Y | 19 |  |
| buy_fee | 매수수수료 | String | Y | 19 |  |
| fee_smtl | 수수료합계 | String | Y | 19 |  |
| trad_pfls_smtl | 매매손익합계 | String | Y | 19 |  |
