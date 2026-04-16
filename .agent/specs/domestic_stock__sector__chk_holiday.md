<!-- endpoint: /uapi/domestic-stock/v1/quotations/chk-holiday -->
<!-- category: [국내주식] 업종/기타 -->
<!-- korean_name: 국내휴장일조회 -->

# 국내휴장일조회[국내주식-040]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/chk-holiday
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTCA0903R
- **모의TRID**: 모의투자 미지원

## 개요
(★중요) 국내휴장일조회(TCA0903R) 서비스는 당사 원장서비스와 연관되어 있어
단시간 내 다수 호출시 서비스에 영향을 줄 수 있어 가급적 1일 1회 호출 부탁드립니다.
국내휴장일조회 API입니다.
영업일, 거래일, 개장일, 결제일 여부를 조회할 수 있습니다.
주문을 넣을 수 있는지 확인하고자 하실 경우 개장일여부(opnd_yn)을 사용하시면 됩니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTCA0903R |
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
| BASS_DT | 기준일자 | String | Y | 8 | 기준일자(YYYYMMDD) |
| CTX_AREA_NK | 연속조회키 | String | Y | 20 | 공백으로 입력 |
| CTX_AREA_FK | 연속조회검색조건 | String | Y | 20 | 공백으로 입력 |

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
| output | 응답상세1 | Object | Y |  |  |
| bass_dt | 기준일자 | String | Y | 8 | 기준일자(YYYYMMDD) |
| wday_dvsn_cd | 요일구분코드 | String | Y | 2 | 01:일요일, 02:월요일, 03:화요일, 04:수요일, 05:목요일, 06:금요일, 07:토요일 |
| bzdy_yn | 영업일여부 | String | Y | 1 | Y/N금융기관이 업무를 하는 날 |
| tr_day_yn | 거래일여부 | String | Y | 1 | Y/N증권 업무가 가능한 날(입출금, 이체 등의 업무 포함) |
| opnd_yn | 개장일여부 | String | Y | 1 | Y/N주식시장이 개장되는 날* 주문을 넣고자 할 경우 개장일여부(opnd_yn)를 사용 |
| sttl_day_yn | 결제일여부 | String | Y | 1 | Y/N주식 거래에서 실제로 주식을 인수하고 돈을 지불하는 날 |
