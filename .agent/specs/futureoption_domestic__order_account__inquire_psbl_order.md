<!-- endpoint: /uapi/domestic-futureoption/v1/trading/inquire-psbl-order -->
<!-- category: [국내선물옵션] 주문/계좌 -->
<!-- korean_name: 선물옵션 주문가능 -->

# 선물옵션 주문가능[v1_국내선물-005]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-futureoption/v1/trading/inquire-psbl-order
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: TTTO5105R
- **모의TRID**: VTTO5105R

## 개요
선물옵션 주문가능 API입니다. 주문가능 내역과 수량을 확인하실 수 있습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access Token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Credentials Grant 절차를 준용) 제휴사(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자] TTTO5105R : 선물 옵션 주문 가능[모의투자] VTTO5105R : 선물 옵션 주문 가능 |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객타입 | String | N | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사 APP을 사용하는 경우 사용자(회원) 핸드폰번호ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | 제휴사는 사용자(회원)의 IP Address 필수이며 일반고객은 제외 |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CANO | 종합계좌번호 | String | N | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | N | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| PDNO | 상품번호 | String | N | 12 | 선물옵션종목코드선물 6자리 (예: 101S03)옵션 9자리 (예: 201S03370) |
| SLL_BUY_DVSN_CD | 매도매수구분코드 | String | N | 2 | 01 : 매도02 : 매수 |
| UNIT_PRICE | 주문가격1 | String | N | 23 | 주문가격※ 주문가격 '0'일 경우 - 옵션매수 : 현재가 - 그 이외 : 기준가 |
| ORD_DVSN_CD | 주문구분코드 | String | N | 2 | 01 : 지정가02 : 시장가03 : 조건부04 : 최유리,10 : 지정가(IOC)11 : 지정가(FOK)12 : 시장가(IOC)13 : 시장가(FOK)14 : 최유리(IOC)15 : 최유리(FOK) |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | Y | 1 | tr_cont를 이용한 다음조회 불가 API |
| gt_uid | Global UID | String | Y | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 | 응답코드 |
| msg1 | 응답메세지 | String | Y | 80 | 응답메세지 |
| output | 응답상세 | Array | Y | - |  |
| tot_psbl_qty | 총가능수량 | String | Y | 10 | 총가능수량 |
| lqd_psbl_qty1 | 청산가능수량1 | String | Y | 10 | 청산가능수량 |
| ord_psbl_qty | 주문가능수량 | String | Y | 10 | 주문가능수량 |
| bass_idx | 기준지수 | String | Y | 32 | 기준지수 |
