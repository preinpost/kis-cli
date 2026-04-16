<!-- endpoint: /uapi/domestic-bond/v1/trading/inquire-psbl-order -->
<!-- category: [장내채권] 주문/계좌 -->
<!-- korean_name: 장내채권 매수가능조회 -->

# 장내채권 매수가능조회 [국내주식-199]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-bond/v1/trading/inquire-psbl-order
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: TTTC8910R
- **모의TRID**: 모의투자 미지원

## 개요
장내채권 매수가능조회 API입니다.
한국투자 HTS(eFriend Plus) > [0978] 장내채권주문 화면의 "왼쪽 하단 증거금 사용가능 내역 / 주문가능금액 및 수량" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
※ (중요) 채권의 경우 주식과 달리, 매수가능수량(buy_psbl_qty) = 매수가능금액(buy_psbl_amt) / 채권주문단가2(bond_ord_unpr2) * 10 인 점 유의하시기 바랍니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | TTTC8910R |
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
| CANO | 종합계좌번호 | String | Y | 8 |  |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 |  |
| PDNO | 상품번호 | String | Y | 12 |  |
| BOND_ORD_UNPR | 채권주문단가 | String | Y | 182 |  |
| SAMT_MKET_PTCI_YN | 소액시장참여여부 | String | Y | 1 | Y(소액시장) N (일반시장) |

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
| output | 응답상세 | Object Array | Y |  | array |
| ord_psbl_cash | 주문가능현금 | String | Y | 19 |  |
| ord_psbl_sbst | 주문가능대용 | String | Y | 19 |  |
| ruse_psbl_amt | 재사용가능금액 | String | Y | 19 |  |
| bond_ord_unpr2 | 채권주문단가2 | String | Y | 182 |  |
| buy_psbl_amt | 매수가능금액 | String | Y | 19 |  |
| buy_psbl_qty | 매수가능수량 | String | Y | 10 | 매수가능수량(buy_psbl_qty) = 매수가능금액(buy_psbl_amt) / 채권주문단가2(bond_ord_unpr2) * 10 |
| cma_evlu_amt | CMA평가금액 | String | Y | 19 |  |
