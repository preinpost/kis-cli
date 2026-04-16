<!-- endpoint: /uapi/domestic-stock/v1/trading/inquire-credit-psamount -->
<!-- category: [국내주식] 주문/계좌 -->
<!-- korean_name: 신용매수가능조회 -->

# 신용매수가능조회[v1_국내주식-042]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/trading/inquire-credit-psamount
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: TTTC8909R
- **모의TRID**: 모의투자 미지원
- **Content-Type**: application/json; charset=utf-8

## 개요
신용매수가능조회 API입니다.
신용매수주문 시 주문가능수량과 금액을 확인하실 수 있습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | TTTC8909R |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
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
| PDNO | 상품번호 | String | Y | 12 | 종목코드(6자리) |
| ORD_UNPR | 주문단가 | String | Y | 19 | 1주당 가격 * 장전 시간외, 장후 시간외, 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력 권고 |
| ORD_DVSN | 주문구분 | String | Y | 2 | 00 : 지정가 01 : 시장가 02 : 조건부지정가 03 : 최유리지정가 04 : 최우선지정가 05 : 장전 시간외 06 : 장후 시간외 07 : 시간외 단일가 등 |
| CRDT_TYPE | 신용유형 | String | Y | 2 | 21 : 자기융자신규 23 : 유통융자신규 26 : 유통대주상환 28 : 자기대주상환 25 : 자기융자상환 27 : 유통융자상환 22 : 유통대주신규 24 : 자기대주신규 |
| CMA_EVLU_AMT_ICLD_YN | CMA평가금액포함여부 | String | Y | 1 | Y/N |
| OVRS_ICLD_YN | 해외포함여부 | String | Y | 1 | Y/N |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 | 응답코드 |
| msg1 | 응답메세지 | String | Y | 80 | 응답메시지 |
| output | 응답상세 | Object | Y |  |  |
| ord_psbl_cash | 주문가능현금 | String | Y | 19 |  |
| ord_psbl_sbst | 주문가능대용 | String | Y | 19 |  |
| ruse_psbl_amt | 재사용가능금액 | String | Y | 19 |  |
| fund_rpch_chgs | 펀드환매대금 | String | Y | 19 |  |
| psbl_qty_calc_unpr | 가능수량계산단가 | String | Y | 19 |  |
| nrcvb_buy_amt | 미수없는매수금액 | String | Y | 19 |  |
| nrcvb_buy_qty | 미수없는매수수량 | String | Y | 10 |  |
| max_buy_amt | 최대매수금액 | String | Y | 19 |  |
| max_buy_qty | 최대매수수량 | String | Y | 10 |  |
| cma_evlu_amt | CMA평가금액 | String | Y | 19 |  |
| ovrs_re_use_amt_wcrc | 해외재사용금액원화 | String | Y | 19 |  |
| ord_psbl_frcr_amt_wcrc | 주문가능외화금액원화 | String | Y | 19 |  |
