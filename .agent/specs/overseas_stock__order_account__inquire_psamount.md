<!-- endpoint: /uapi/overseas-stock/v1/trading/inquire-psamount -->
<!-- category: [해외주식] 주문/계좌 -->
<!-- korean_name: 해외주식 매수가능금액조회 -->

# 해외주식 매수가능금액조회[v1_해외주식-014]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-stock/v1/trading/inquire-psamount
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: TTTS3007R
- **모의TRID**: VTTS3007R
- **Format**: JSON
- **Content-Type**: application/json; charset=utf-8

## 개요
해외주식 매수가능금액조회 API입니다.
* 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자]TTTS3007R[모의투자]VTTS3007R |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객 타입 | String | N | 1 | B : 법인 / P : 개인 |
| seq_no | 일련번호 | String | N | 2 | 법인 : "001" / 개인: ""(Default) |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| OVRS_EXCG_CD | 해외거래소코드 | String | Y | 4 | NASD : 나스닥 / NYSE : 뉴욕 / AMEX : 아멕스SEHK : 홍콩 / SHAA : 중국상해 / SZAA : 중국심천TKSE : 일본 / HASE : 하노이거래소 / VNSE : 호치민거래소 |
| OVRS_ORD_UNPR | 해외주문단가 | String | Y | 27 | 해외주문단가 (23.8) 정수부분 23자리, 소수부분 8자리 |
| ITEM_CD | 종목코드 | String | Y | 12 | 종목코드 |

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
| rt_cd | 성공 실패 여부 | String | Y | 1 |  |
| msg_cd | 응답코드 | String | Y | 8 |  |
| msg1 | 응답메세지 | String | Y | 80 |  |
| output | 응답상세1 | Object | N | - |  |
| tr_crcy_cd | 거래통화코드 | String | N | 3 | 18.2 |
| ord_psbl_frcr_amt | 주문가능외화금액 | String | N | 21 | 18.2 |
| sll_ruse_psbl_amt | 매도재사용가능금액 | String | N | 21 | 가능금액 산정 시 사용 |
| ovrs_ord_psbl_amt | 해외주문가능금액 | String | N | 21 | - 한국투자 앱 해외주식 주문화면내 "외화" 인경우 주문가능금액 |
| max_ord_psbl_qty | 최대주문가능수량 | String | N | 19 | - 한국투자 앱 해외주식 주문화면내 "외화" 인경우 주문가능수량- 매수 시 수량단위 절사해서 사용 예 : (100주단위) 545 주 -> 500 주 / (10주단위) 545 주 -> 540 주 |
| echm_af_ord_psbl_amt | 환전이후주문가능금액 | String | N | 21 | 사용되지 않는 사항(0으로 출력) |
| echm_af_ord_psbl_qty | 환전이후주문가능수량 | String | N | 19 | 사용되지 않는 사항(0으로 출력) |
| ord_psbl_qty | 주문가능수량 | String | N | 10 | 22(20.1) |
| exrt | 환율 | String | N | 22 | 25(18.6) |
| frcr_ord_psbl_amt1 | 외화주문가능금액1 | String | N | 25 | - 한국투자 앱 해외주식 주문화면내 "통합" 인경우 주문가능금액 |
| ovrs_max_ord_psbl_qty | 해외최대주문가능수량 | String | N | 19 | - 한국투자 앱 해외주식 주문화면내 "통합" 인경우 주문가능수량- 매수 시 수량단위 절사해서 사용 예 : (100주단위) 545 주 -> 500 주 / (10주단위) 545 주 -> 540 주 |
