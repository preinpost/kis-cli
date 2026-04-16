<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-time-itemconclusion -->
<!-- category: [국내주식] 기본시세 -->
<!-- korean_name: 주식현재가 당일시간대별체결 -->

# 주식현재가 당일시간대별체결[v1_국내주식-023]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-time-itemconclusion
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: FHPST01060000
- **모의TRID**: FHPST01060000

## 개요
주식현재가 당일시간대별체결 API입니다.
* FID_INPUT_HOUR_1 를 이용하여 과거시간대 체결데이터 확인 가능

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHPST01060000 |
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
| FID_COND_MRKT_DIV_CODE | 조건 시장 분류 코드 | String | Y | 2 | J:KRX, NX:NXT, UN:통합 |
| FID_INPUT_ISCD | 입력 종목코드 | String | Y | 12 | 종목코드 (ex 005930 삼성전자) |
| FID_INPUT_HOUR_1 | 입력 시간1 | String | Y | 10 | 입력시간 |

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
| output1 | 응답상세 | Object | Y |  | single |
| stck_prpr | 주식 현재가 | String | Y | 10 |  |
| prdy_vrss | 전일 대비 | String | Y | 10 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| prdy_ctrt | 전일 대비율 | String | Y | 11 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| prdy_vol | 전일 거래량 | String | Y | 18 |  |
| rprs_mrkt_kor_name | 대표 시장 한글 명 | String | Y | 40 |  |
| output2 | 응답상세 | Object | Y |  | single |
| stck_cntg_hour | 주식 체결 시간 | String | Y | 6 |  |
| stck_pbpr | 주식 현재가 | String | Y | 10 |  |
| prdy_vrss | 전일 대비 | String | Y | 10 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| prdy_ctrt | 전일 대비율 | String | Y | 11 |  |
| askp | 매도호가 | String | Y | 10 |  |
| bidp | 매수호가 | String | Y | 10 |  |
| tday_rltv | 당일 체결강도 | String | Y | 14 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| cnqn | 체결량 | String | Y | 18 |  |
