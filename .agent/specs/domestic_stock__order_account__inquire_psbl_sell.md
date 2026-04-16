<!-- endpoint: /uapi/domestic-stock/v1/trading/inquire-psbl-sell -->
<!-- category: [국내주식] 주문/계좌 -->
<!-- korean_name: 매도가능수량조회 -->

# 매도가능수량조회 [국내주식-165]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/trading/inquire-psbl-sell
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: TTTC8408R
- **모의TRID**: 모의투자 미지원

## 개요
매도가능수량조회 API입니다.
한국투자 HTS(eFriend Plus) > [0971] 주식 매도 화면에서 종목코드 입력 후 "가능" 클릭 시 매도가능수량이 확인되는 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
특정종목 매도가능수량 확인 시, 매도주문 내시려는 주문종목(PDNO)으로 API 호출 후
output > ord_psbl_qty(주문가능수량) 확인하실 수 있습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | TTTC8408R |
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
| CANO | 종합계좌번호 | String | Y | 8 | 종합계좌번호 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌상품코드 |
| PDNO | 종목번호 | String | Y | 12 | 보유종목 코드 ex)000660 |

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
| output1 | 응답상세 | Object | Y |  |  |
| pdno | 상품번호 | String | Y | 12 |  |
| prdt_name | 상품명 | String | Y | 60 |  |
| buy_qty | 매수수량 | String | Y | 10 |  |
| sll_qty | 매도수량 | String | Y | 10 |  |
| cblc_qty | 잔고수량 | String | Y | 19 |  |
| nsvg_qty | 비저축수량 | String | Y | 19 |  |
| ord_psbl_qty | 주문가능수량 | String | Y | 10 |  |
| pchs_avg_pric | 매입평균가격 | String | Y | 184 |  |
| pchs_amt | 매입금액 | String | Y | 19 |  |
| now_pric | 현재가 | String | Y | 8 |  |
| evlu_amt | 평가금액 | String | Y | 19 |  |
| evlu_pfls_amt | 평가손익금액 | String | Y | 19 |  |
| evlu_pfls_rt | 평가손익율 | String | Y | 72 |  |
