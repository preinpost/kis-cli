<!-- endpoint: /uapi/domestic-stock/v1/ksdinfo/list-info -->
<!-- category: [국내주식] 종목정보 -->
<!-- korean_name: 예탁원정보(상장정보일정) -->

# 예탁원정보(상장정보일정)[국내주식-150]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/ksdinfo/list-info
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: HHKDB669107C0
- **모의TRID**: 모의투자 미지원

## 개요
예탁원정보(상장정보일정) API입니다.
한국투자 HTS(eFriend Plus) > [0666] 상장정보 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHKDB669107C0 |
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
| SHT_CD | 종목코드 | String | Y | 9 | 공백: 전체, 특정종목 조회시 : 종목코드 |
| T_DT | 조회일자To | String | Y | 8 | ~ 일자 |
| F_DT | 조회일자From | String | Y | 8 | 일자 ~ |
| CTS | CTS | String | Y | 17 | 공백 |

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
| output1 | 응답상세 | Object Array | Y |  | array |
| list_dt | 상장/등록일 | String | Y | 10 |  |
| sht_cd | 종목코드 | String | Y | 9 |  |
| isin_name | 종목명 | String | Y | 40 |  |
| stk_kind | 주식종류 | String | Y | 10 |  |
| issue_type | 사유 | String | Y | 21 |  |
| issue_stk_qty | 상장주식수 | String | Y | 12 |  |
| tot_issue_stk_qty | 총발행주식수 | String | Y | 12 |  |
| issue_price | 발행가 | String | Y | 9 |  |
