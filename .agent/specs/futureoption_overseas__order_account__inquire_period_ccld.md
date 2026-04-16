<!-- endpoint: /uapi/overseas-futureoption/v1/trading/inquire-period-ccld -->
<!-- category: [해외선물옵션] 주문/계좌 -->
<!-- korean_name: 해외선물옵션 기간계좌손익 일별 -->

# 해외선물옵션 기간계좌손익 일별[해외선물-010]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-futureoption/v1/trading/inquire-period-ccld
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: OTFM3118R
- **모의TRID**: 모의투자 미지원
- **Format**: JSON
- **Content-Type**: application/json; charset=utf-8

## 개요
해외선물옵션 기간계좌손익 일별 API입니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용)※ 토큰 지정시 토큰 타입("Bearer") 지정 필요. 즉, 발급받은 접근토큰 앞에 앞에 "Bearer" 붙여서 호출EX) "Bearer eyJ..........8GA" |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | OTFM3118R |
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
| INQR_TERM_FROM_DT | 조회기간FROM일자 | String | Y | 8 |  |
| INQR_TERM_TO_DT | 조회기간TO일자 | String | Y | 8 |  |
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| CRCY_CD | 통화코드 | String | Y | 3 | '%%% : 전체TUS: TOT_USD / TKR: TOT_KRWKRW: 한국 / USD: 미국EUR: EUR / HKD: 홍콩CNY: 중국 / JPY: 일본' |
| WHOL_TRSL_YN | 전체환산여부 | String | Y | 1 | N |
| FUOP_DVSN | 선물옵션구분 | String | Y | 2 | 00:전체 / 01:선물 / 02:옵션 |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | Y | 200 |  |
| CTX_AREA_NK200 | 연속조회키200 | String | Y | 200 |  |

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
| output1 | 응답상세1 | Object Array | Y |  | Array |
| cano | 종합계좌번호 | String | Y | 8 |  |
| acnt_prdt_cd | 계좌상품코드 | String | N | 2 |  |
| crcy_cd | 통화코드 | String | Y | 3 |  |
| fm_buy_qty | FM매수수량 | String | Y | 10 |  |
| fm_sll_qty | FM매도수량 | String | Y | 10 |  |
| fm_lqd_pfls_amt | FM청산손익금액 | String | Y | 20 |  |
| fm_fee | FM수수료 | String | Y | 20 |  |
| fm_net_pfls_amt | FM순손익금액 | String | Y | 20 |  |
| fm_ustl_buy_qty | FM미결제매수수량 | String | Y | 10 |  |
| fm_ustl_sll_qty | FM미결제매도수량 | String | Y | 10 |  |
| fm_ustl_evlu_pfls_amt | FM미결제평가손익금액 | String | Y | 20 |  |
| fm_ustl_evlu_pfls_amt2 | FM미결제평가손익금액2 | String | Y | 20 |  |
| fm_ustl_evlu_pfls_icdc_amt | FM미결제평가손익증감금액 | String | Y | 20 |  |
| fm_ustl_agrm_amt | FM미결제약정금액 | String | Y | 20 |  |
| fm_opt_lqd_amt | FM옵션청산금액 | String | Y | 20 |  |
| output2 | 응답상세2 | Object Array | Y |  | Array |
| cano | 종합계좌번호 | String | Y | 8 |  |
| acnt_prdt_cd | 계좌상품코드 | String | Y | 2 |  |
| ovrs_futr_fx_pdno | 해외선물FX상품번호 | String | Y | 32 |  |
| crcy_cd | 통화코드 | String | Y | 3 |  |
| fm_buy_qty | FM매수수량 | String | Y | 10 |  |
| fm_sll_qty | FM매도수량 | String | Y | 10 |  |
| fm_lqd_pfls_amt | FM청산손익금액 | String | Y | 20 |  |
| fm_fee | FM수수료 | String | Y | 20 |  |
| fm_net_pfls_amt | FM순손익금액 | String | Y | 20 |  |
| fm_ustl_buy_qty | FM미결제매수수량 | String | Y | 10 |  |
| fm_ustl_sll_qty | FM미결제매도수량 | String | Y | 10 |  |
| fm_ustl_evlu_pfls_amt | FM미결제평가손익금액 | String | Y | 20 |  |
| fm_ustl_evlu_pfls_amt2 | FM미결제평가손익금액2 | String | Y | 20 |  |
| fm_ustl_evlu_pfls_icdc_amt | FM미결제평가손익증감금액 | String | Y | 20 |  |
| fm_ccld_avg_pric | FM체결평균가격 | String | Y | 20 |  |
| fm_ustl_agrm_amt | FM미결제약정금액 | String | Y | 20 |  |
| fm_opt_lqd_amt | FM옵션청산금액 | String | Y | 20 |  |
