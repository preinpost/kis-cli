<!-- endpoint: /uapi/domestic-stock/v1/finance/profit-ratio -->
<!-- category: [국내주식] 종목정보 -->
<!-- korean_name: 국내주식 수익성비율 -->

# 국내주식 수익성비율[v1_국내주식-081]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/finance/profit-ratio
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: FHKST66430400
- **모의TRID**: 모의투자 미지원

## 개요
국내주식 수익성비율 API입니다.
한국투자 HTS(eFriend Plus) > [0635] 재무분석종합 화면의 하단 '4. 수익성비율' 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHKST66430400 |
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
| fid_input_iscd | 입력 종목코드 | String | Y | 12 | 000660 : 종목코드 |
| FID_DIV_CLS_CODE | 분류 구분 코드 | String | Y | 2 | 0: 년, 1: 분기 |
| fid_cond_mrkt_div_code | 조건 시장 분류 코드 | String | Y | 2 | J |

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
| stac_yymm | 결산 년월 | String | Y | 6 |  |
| cptl_ntin_rate | 총자본 순이익율 | String | Y | 92 |  |
| self_cptl_ntin_inrt | 자기자본 순이익율 | String | Y | 92 |  |
| sale_ntin_rate | 매출액 순이익율 | String | Y | 92 |  |
| sale_totl_rate | 매출액 총이익율 | String | Y | 92 |  |
