<!-- endpoint: /uapi/domestic-futureoption/v1/trading/inquire-psbl-ngt-order -->
<!-- category: [국내선물옵션] 주문/계좌 -->
<!-- korean_name: (야간)선물옵션 주문가능 조회 -->

# (야간)선물옵션 주문가능 조회 [국내선물-011]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-futureoption/v1/trading/inquire-psbl-ngt-order
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: (구) JTCE1004R (신) STTN5105R
- **모의TRID**: 모의투자 미지원

## 개요
(야간)선물옵션 주문가능 조회 API입니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | (구) JTCE1004R (신) STTN5105R |
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
| CANO | 종합계좌번호 | String | Y | 8 |  |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 |  |
| PDNO | 상품번호 | String | Y | 12 |  |
| PRDT_TYPE_CD | 상품유형코드 | String | Y | 3 | 301 : 선물옵션 |
| SLL_BUY_DVSN_CD | 매도매수구분코드 | String | Y | 2 | 01 : 매도 , 02 : 매수 |
| UNIT_PRICE | 주문가격1 | String | Y | 23 |  |
| ORD_DVSN_CD | 주문구분코드 | String | Y | 2 | '01 : 지정가 02 : 시장가 03 : 조건부 04 : 최유리, 10 : 지정가(IOC) 11 : 지정가(FOK) 12 : 시장가(IOC) 13 : 시장가(FOK) 14 : 최유리(IOC) 15 : 최유리(FOK)' |

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
| output | 응답상세1 | Object | Y |  |  |
| max_ord_psbl_qty | 최대주문가능수량 | String | Y | 19 | 최대주문가능수량 (신규 TR 미사용 필드) |
| tot_psbl_qty | 최대주문가능수량 | String | Y | 19 |  |
| lqd_psbl_qty | 청산가능수량 | String | Y | 19 | 청산가능수량 |
| lqd_psbl_qty_1 | 청산가능수량 | String | Y | 19 | 신규 TR 사용 필드 |
| ord_psbl_qty | 주문가능수량 | String | Y | 19 |  |
| bass_idx | 기준지수 | String | Y | 23 | 신규 TR 사용 필드 |
