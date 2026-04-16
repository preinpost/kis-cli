<!-- endpoint: /uapi/domestic-futureoption/v1/quotations/inquire-time-fuopchartprice -->
<!-- category: [국내선물옵션] 기본시세 -->
<!-- korean_name: 선물옵션 분봉조회 -->

# 선물옵션 분봉조회[v1_국내선물-012]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-futureoption/v1/quotations/inquire-time-fuopchartprice
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: FHKIF03020200
- **모의TRID**: 모의투자 미지원

## 개요
선물옵션 분봉조회 API입니다.
실전계좌의 경우, 한 번의 호출에 최대 102건까지 확인 가능하며,
FID_INPUT_DATE_1(입력날짜), FID_INPUT_HOUR_1(입력시간)을 이용하여 다음조회 가능합니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHKIF03020200 |
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
| FID_COND_MRKT_DIV_CODE | FID 조건 시장 분류 코드 | String | Y | 2 | F: 지수선물, O:지수옵션JF: 주식선물, JO:주식옵션,CF: 상품선물(금), 금리선물(국채), 통화선물(달러)CM: 야간선물, EU: 야간옵션 |
| FID_INPUT_ISCD | FID 입력 종목코드 | String | Y | 12 | 종목번호 (지수선물:6자리, 지수옵션 9자리) |
| FID_HOUR_CLS_CODE | FID 시간 구분 코드 | String | Y | 5 | FID 시간 구분 코드(30: 30초, 60: 1분, 3600: 1시간) |
| FID_PW_DATA_INCU_YN | FID 과거 데이터 포함 여부 | String | Y | 2 | Y(과거) / N (당일) |
| FID_FAKE_TICK_INCU_YN | FID 허봉 포함 여부 | String | Y | 2 | N으로 입력 |
| FID_INPUT_DATE_1 | FID 입력 날짜1 | String | Y | 10 | 입력 날짜 기준으로 이전 기간 조회(YYYYMMDD)ex) 20230908 입력 시, 2023년 9월 8일부터 일자 역순으로 조회 |
| FID_INPUT_HOUR_1 | FID 입력 시간1 | String | Y | 10 | 입력 시간 기준으로 이전 시간 조회(HHMMSS)ex) 093000 입력 시, 오전 9시 30분부터 역순으로 분봉 조회* CM(야간선물), EU(야간옵션)인 경우, 자정 이후 시간은 +24시간으로 입력ex) 253000 입력 시, 새벽 1시 30분부터 역순으로 분봉 조회 |

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
| Output1 | 응답상세 | Object Array | Y |  |  |
| futs_prdy_vrss | 선물 전일 대비 | String | Y | 11 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| futs_prdy_ctrt | 선물 전일 대비율 | String | Y | 8 |  |
| futs_prdy_clpr | 선물 전일 종가 | String | Y | 11 |  |
| prdy_nmix | 전일 지수 | String | Y | 11 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
| hts_kor_isnm | HTS 한글 종목명 | String | Y | 40 |  |
| futs_prpr | 선물 현재가 | String | Y | 11 |  |
| futs_shrn_iscd | 선물 단축 종목코드 | String | Y | 9 |  |
| prdy_vol | 전일 거래량 | String | Y | 18 |  |
| futs_mxpr | 선물 상한가 | String | Y | 11 |  |
| futs_llam | 선물 하한가 | String | Y | 11 |  |
| futs_oprc | 선물 시가2 | String | Y | 11 |  |
| futs_hgpr | 선물 최고가 | String | Y | 11 |  |
| futs_lwpr | 선물 최저가 | String | Y | 11 |  |
| futs_prdy_oprc | 선물 전일 시가 | String | Y | 11 |  |
| futs_prdy_hgpr | 선물 전일 최고가 | String | Y | 11 |  |
| futs_prdy_lwpr | 선물 전일 최저가 | String | Y | 11 |  |
| futs_askp | 선물 매도호가 | String | Y | 11 |  |
| futs_bidp | 선물 매수호가 | String | Y | 11 |  |
| basis | 베이시스 | String | Y | 8 |  |
| kospi200_nmix | KOSPI200 지수 | String | Y | 11 |  |
| kospi200_prdy_vrss | KOSPI200 전일 대비 | String | Y | 18 |  |
| kospi200_prdy_ctrt | KOSPI200 전일 대비율 | String | Y | 8 |  |
| kospi200_prdy_vrss_sign | KOSPI200 전일 대비 부호 | String | Y | 1 |  |
| hts_otst_stpl_qty | HTS 미결제 약정 수량 | String | Y | 18 |  |
| otst_stpl_qty_icdc | 미결제 약정 수량 증감 | String | Y | 10 |  |
| tday_rltv | 당일 체결강도 | String | Y | 11 |  |
| hts_thpr | HTS 이론가 | String | Y | 11 |  |
| dprt | 괴리율 | String | Y | 8 |  |
| Output2 | 응답상세2 | Object | Y |  | array |
| stck_bsop_date | 주식 영업 일자 | String | Y | 8 |  |
| stck_cntg_hour | 주식 체결 시간 | String | Y | 6 | CM(야간선물), EU(야간옵션)인 경우, 자정 이후 시간은 +24시간으로 표시ex) "260000"인 경우, 오전 4시를 의미 |
| futs_prpr | 선물 현재가 | String | Y | 11 |  |
| futs_oprc | 선물 시가2 | String | Y | 11 |  |
| futs_hgpr | 선물 최고가 | String | Y | 11 |  |
| futs_lwpr | 선물 최저가 | String | Y | 11 |  |
| cntg_vol | 체결 거래량 | String | Y | 18 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
