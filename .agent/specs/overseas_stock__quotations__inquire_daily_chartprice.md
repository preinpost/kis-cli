<!-- endpoint: /uapi/overseas-price/v1/quotations/inquire-daily-chartprice -->
<!-- category: [해외주식] 기본시세 -->
<!-- korean_name: 해외주식 종목/지수/환율기간별시세(일/주/월/년) -->

# 해외주식 종목/지수/환율기간별시세(일/주/월/년)[v1_해외주식-012]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-price/v1/quotations/inquire-daily-chartprice
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: FHKST03030100
- **모의TRID**: FHKST03030100

## 개요
해외주식 종목/지수/환율기간별시세(일/주/월/년) API입니다.
해외지수 당일 시세의 경우 지연시세 or 종가시세가 제공됩니다.
※ 해당 API로 미국주식 조회 시, 다우30, 나스닥100, S&P500 종목만 조회 가능합니다.
더 많은 미국주식 종목 시세를 이용할 시에는, 해외주식기간별시세 API 사용 부탁드립니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자/모의투자]FHKST03030100 |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객 타입 | String | N | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| FID_COND_MRKT_DIV_CODE | FID 조건 시장 분류 코드 | String | Y | 2 | N: 해외지수, X 환율, I: 국채, S:금선물 |
| FID_INPUT_ISCD | FID 입력 종목코드 | String | Y | 12 | 종목코드※ 해외주식 마스터 코드 참조 (포럼 > FAQ > 종목정보 다운로드(해외) > 해외지수)※ 해당 API로 미국주식 조회 시, 다우30, 나스닥100, S&P500 종목만 조회 가능합니다. 더 많은 미국주식 종목 시세를 이용할 시에는, 해외주식기간별시세 API 사용 부탁드립니다. |
| FID_INPUT_DATE_1 | FID 입력 날짜1 | String | Y | 10 | 시작일자(YYYYMMDD) |
| FID_INPUT_DATE_2 | FID 입력 날짜2 | String | Y | 10 | 종료일자(YYYYMMDD) |
| FID_PERIOD_DIV_CODE | FID 기간 분류 코드 | String | Y | 32 | D:일, W:주, M:월, Y:년 |

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
| output1 | 응답상세1 | Object | N |  | 기본정보 |
| ovrs_nmix_prdy_vrss | 전일 대비 | String | N | 16 | 16(11.4) 정수부분 11자리, 소수부분 4자리 |
| prdy_vrss_sign | 전일 대비 부호 | String | N | 1 |  |
| prdy_ctrt | 전일 대비율 | String | N | 11 | 11(8.2) 정수부분 8자리, 소수부분 2자리 |
| ovrs_nmix_prdy_clpr | 전일 종가 | String | N | 16 | 16(11.4) 정수부분 11자리, 소수부분 4자리 |
| acml_vol | 누적 거래량 | String | N | 18 |  |
| hts_kor_isnm | HTS 한글 종목명 | String | N | 40 |  |
| ovrs_nmix_prpr | 현재가 | String | N | 16 | 16(11.4) 정수부분 11자리, 소수부분 4자리 |
| stck_shrn_iscd | 단축 종목코드 | String | N | 9 |  |
| prdy_vol | 전일 거래량 | String | N | 18 |  |
| ovrs_prod_oprc | 시가 | String | N | 16 | 16(11.4) 정수부분 11자리, 소수부분 4자리 |
| ovrs_prod_hgpr | 최고가 | String | N | 16 | 16(11.4) 정수부분 11자리, 소수부분 4자리 |
| ovrs_prod_lwpr | 최저가 | String | N | 16 | 16(11.4) 정수부분 11자리, 소수부분 4자리 |
| output2 | 응답상세2 | Object Array | N |  | 일자별 정보 |
| stck_bsop_date | 영업 일자 | String | N | 8 |  |
| ovrs_nmix_prpr | 현재가 | String | N | 16 | 16(11.4) 정수부분 11자리, 소수부분 4자리 |
| ovrs_nmix_oprc | 시가 | String | N | 16 | 16(11.4) 정수부분 11자리, 소수부분 4자리 |
| ovrs_nmix_hgpr | 최고가 | String | N | 16 | 16(11.4) 정수부분 11자리, 소수부분 4자리 |
| ovrs_nmix_lwpr | 최저가 | String | N | 16 | 16(11.4) 정수부분 11자리, 소수부분 4자리 |
| acml_vol | 누적 거래량 | String | N | 18 |  |
| mod_yn | 변경 여부 | String | N | 1 |  |
