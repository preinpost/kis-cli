<!-- endpoint: /uapi/overseas-price/v1/quotations/rights-by-ice -->
<!-- category: [해외주식] 시세분석 -->
<!-- korean_name: 해외주식 권리종합 -->

# 해외주식 권리종합 [해외주식-050]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-price/v1/quotations/rights-by-ice
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: HHDFS78330900
- **모의TRID**: 모의투자 미지원

## 개요
해외주식 권리종합 API입니다.
한국투자 HTS(eFriend Plus) > [7833] 해외주식 권리(ICE제공) 화면의 "전체" 탭 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
※ 조회기간 기준일 입력시 참고 - 상환: 상환일자, 조기상환: 조기상환일자, 티커변경: 적용일, 그 외: 발표일

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHDFS78330900 |
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
| NCOD | 국가코드 | String | Y | 2 | CN:중국 HK:홍콩 US:미국 JP:일본 VN:베트남 |
| SYMB | 심볼 | String | Y | 20 | 종목코드 |
| ST_YMD | 일자 시작일 | String | Y | 8 | 미입력 시, 오늘-3개월기간지정 시, 종료일 입력(ex. 20240514)※ 조회기간 기준일 입력시 참고- 상환: 상환일자, 조기상환: 조기상환일자, 티커변경: 적용일, 그 외: 발표일 |
| ED_YMD | 일자 종료일 | String | Y | 8 | 미입력 시, 오늘+3개월기간지정 시, 종료일 입력(ex. 20240514)※ 조회기간 기준일 입력시 참고- 상환: 상환일자, 조기상환: 조기상환일자, 티커변경: 적용일, 그 외: 발표일 |

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
| anno_dt | ICE공시일 | String | Y | 8 |  |
| ca_title | 권리유형 | String | Y | 12 |  |
| div_lock_dt | 배당락일 | String | Y | 8 |  |
| pay_dt | 지급일 | String | Y | 8 |  |
| record_dt | 기준일 | String | Y | 8 |  |
| validity_dt | 효력일자 | String | Y | 8 |  |
| local_end_dt | 현지지시마감일 | String | Y | 8 |  |
| lock_dt | 권리락일 | String | Y | 8 |  |
| delist_dt | 상장폐지일 | String | Y | 8 |  |
| redempt_dt | 상환일자 | String | Y | 8 |  |
| early_redempt_dt | 조기상환일자 | String | Y | 8 |  |
| effective_dt | 적용일 | String | Y | 8 |  |
