<!-- endpoint: /uapi/domestic-stock/v1/quotations/market-time -->
<!-- category: [국내주식] 업종/기타 -->
<!-- korean_name: 국내선물 영업일조회 -->

# 국내선물 영업일조회 [국내주식-160]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/market-time
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: HHMCM000002C0
- **모의TRID**: 모의투자 미지원

## 개요
국내선물 영업일조회 API입니다.
한국투자 HTS(eFriend Plus) > [1938] 시가총액순위 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
API호출 시 body 혹은 params로 입력하는 사항이 없습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHMCM000002C0 |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 |  |
| msg_cd | 응답코드 | String | Y | 8 |  |
| msg1 | 응답메세지 | String | Y | 80 |  |
| output1 | 응답상세 | Object Array | Y |  | array |
| date1 | 영업일1 | String | Y | 8 |  |
| date2 | 영업일2 | String | Y | 8 |  |
| date3 | 영업일3 | String | Y | 8 | 영업일 당일 |
| date4 | 영업일4 | String | Y | 8 |  |
| date5 | 영업일5 | String | Y | 8 |  |
| today | 오늘일자 | String | Y | 8 |  |
| time | 현재시간 | String | Y | 6 |  |
| s_time | 장시작시간 | String | Y | 6 |  |
| e_time | 장마감시간 | String | Y | 6 |  |
