<!-- endpoint: /uapi/domestic-stock/v1/trading/order-resv-rvsecncl -->
<!-- category: [국내주식] 주문/계좌 -->
<!-- korean_name: 주식예약주문정정취소 -->

# 주식예약주문정정취소[v1_국내주식-018,019]

## Info
- **Method**: POST
- **URL**: /uapi/domestic-stock/v1/trading/order-resv-rvsecncl
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: (예약취소) CTSC0009U (예약정정) CTSC0013U
- **모의TRID**: 모의투자 미지원
- **Format**: JSON
- **Content-Type**: application/json; charset=utf-8

## 개요
국내주식 예약주문 정정/취소 API 입니다.
* 정정주문은 취소주문에 비해 필수 입력값이 추가 됩니다.
하단의 입력값을 참조하시기 바랍니다.
※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
(EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자]CTSC0009U : 국내주식예약취소주문CTSC0013U : 국내주식예약정정주문* 모의투자 사용 불가 |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객타입 | String | N | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CANO | 종합계좌번호 | String | Y | 8 | [정정/취소] 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | [정정/취소] 계좌번호 체계(8-2)의 뒤 2자리 |
| PDNO | 종목코드(6자리) | String | Y | 12 | [정정] |
| ORD_QTY | 주문수량 | String | Y | 10 | [정정] 주문주식수 |
| ORD_UNPR | 주문단가 | String | Y | 19 | [정정] 1주당 가격 * 장전 시간외, 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력 권고 |
| SLL_BUY_DVSN_CD | 매도매수구분코드 | String | Y | 2 | [정정]01 : 매도02 : 매수 |
| ORD_DVSN_CD | 주문구분코드 | String | Y | 2 | [정정]00 : 지정가01 : 시장가02 : 조건부지정가05 : 장전 시간외 |
| ORD_OBJT_CBLC_DVSN_CD | 주문대상잔고구분코드 | String | Y | 2 | [정정]10 : 현금12 : 주식담보대출14 : 대여상환21 : 자기융자신규22 : 유통대주신규23 : 유통융자신규24 : 자기대주신규25 : 자기융자상환26 : 유통대주상환27 : 유통융자상환28 : 자기대주상환 |
| LOAN_DT | 대출일자 | String | N | 8 | [정정] |
| RSVN_ORD_END_DT | 예약주문종료일자 | String | N | 8 | [정정] |
| CTAL_TLNO | 연락전화번호 | String | N | 20 | [정정] |
| RSVN_ORD_SEQ | 예약주문순번 | String | Y | 10 | [정정/취소] |
| RSVN_ORD_ORGNO | 예약주문조직번호 | String | N | 5 | [정정/취소] |
| RSVN_ORD_ORD_DT | 예약주문주문일자 | String | N | 8 | [정정/취소] |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공 0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 |  |
| msg | 응답메세지 | String | Y | 80 |  |
| output | 응답상세 | Array | Y | - |  |
| nrml_prcs_yn | 정상처리여부 | String | Y | 1 |  |
