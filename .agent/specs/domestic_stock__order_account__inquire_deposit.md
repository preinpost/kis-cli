<!-- endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-deposit -->
<!-- category: [국내주식] 주문/계좌 -->
<!-- korean_name: 퇴직연금 예수금조회 -->

# 퇴직연금 예수금조회[v1_국내주식-035]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/trading/pension/inquire-deposit
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: TTTC0506R
- **모의TRID**: 모의투자 미지원

## 개요
​※ 55번 계좌(DC가입자계좌)의 경우 해당 API 이용이 불가합니다.
KIS Developers API의 경우 HTS ID에 반드시 연결되어있어야만 API 신청 및 앱정보 발급이 가능한 서비스로 개발되어서 실물계좌가 아닌 55번 계좌는 API 이용이 불가능한 점 양해 부탁드립니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 |  | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 |  | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 |  | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 |  | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 |  | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID |  | Y | 13 | TTTC0506R |
| tr_cont | 연속 거래 여부 |  | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객 타입 |  | Y | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 |  | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 |  | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 |  | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP |  | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID |  | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CANO | 종합계좌번호 |  | Y | 8 |  |
| ACNT_PRDT_CD | 계좌상품코드 |  | Y | 2 | 29 |
| ACCA_DVSN_CD | 적립금구분코드 |  | Y | 2 | 00 |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 |  | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID |  | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 |  | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| gt_uid | Global UID |  | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 |  | Y | 1 |  |
| msg_cd | 응답코드 |  | Y | 8 |  |
| msg1 | 응답메세지 |  | Y | 80 |  |
| output | 응답상세1 |  | Y |  |  |
| dnca_tota | 예수금총액 |  | Y | 19 |  |
| nxdy_excc_amt | 익일정산액 |  | Y | 19 |  |
| nxdy_sttl_amt | 익일결제금액 |  | Y | 19 |  |
| nx2_day_sttl_amt | 2익일결제금액 |  | Y | 19 |  |
