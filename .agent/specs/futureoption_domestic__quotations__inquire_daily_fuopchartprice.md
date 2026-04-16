<!-- endpoint: /uapi/domestic-futureoption/v1/quotations/inquire-daily-fuopchartprice -->
<!-- category: [국내선물옵션] 기본시세 -->
<!-- korean_name: 선물옵션기간별시세(일/주/월/년) -->

# 선물옵션기간별시세(일/주/월/년)[v1_국내선물-008]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-futureoption/v1/quotations/inquire-daily-fuopchartprice
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: FHKIF03020100
- **모의TRID**: FHKIF03020100

## 개요
(지수)선물옵션 기간별시세 데이터(일/주/월/년) 조회 (최대 100건 조회)
실전계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
모의계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용)법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| tr_id | 거래ID | String | Y | 13 | [실전/모의투자]FHKIF03020100 |
| custtype | 고객타입 | String | N | 1 | B : 법인P : 개인 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| FID_COND_MRKT_DIV_CODE | FID 조건 시장 분류 코드 | String | Y | 2 | F: 지수선물, O:지수옵션JF: 주식선물, JO:주식옵션,CF: 상품선물(금), 금리선물(국채), 통화선물(달러)CM: 야간선물, EU: 야간옵션 |
| FID_INPUT_ISCD | 종목코드 | String | Y | 12 | 종목번호 (지수선물:6자리, 지수옵션 9자리) |
| FID_INPUT_DATE_1 | 조회 시작일자 | String | Y | 10 | 조회 시작일자 (ex. 20220401) |
| FID_INPUT_DATE_2 | 조회 종료일자 | String | Y | 10 | 조회 종료일자 (ex. 20220524)※ 주(W), 월(M), 년(Y) 봉 조회 시에 아래 참고ㅁ FID_INPUT_DATE_2 가 현재일 까지일때. 주봉 조회 : 해당 주의 첫번째 영업일이 포함되어야함. 월봉 조회 : 해당 월의 전월 일자로 시작되어야함. 년봉 조회 : 해당 년의 전년도 일자로 시작되어야함ㅁ FID_INPUT_DATE_2 가 현재일보다 이전일 때. 주봉 조회 : 해당 주의 첫번째 영업일이 포함되어야함. 월봉 조회 : 해당 월의 영업일이 포함되어야함. 년봉 조회 : 해당 년의 영업일이 포함되어야함 |
| FID_PERIOD_DIV_CODE | 기간분류코드 | String | Y | 32 | D:일봉 W:주봉, M:월봉, Y:년봉 |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| gt_uid | Global UID | String | Y | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공 0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 | 응답코드 |
| msg1 | 응답메세지 | String | Y | 80 | 응답메세지 |
| output1 | 상세기본정보 | Object | Y | 1 | 상세기본정보 |
| -futs_prdy_vrss | 전일 대비 | String | Y | 14 | 전일 대비 |
| -prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 | 전일 대비 부호 |
| -futs_prdy_ctrt | 선물 전일 대비율 | String | Y | 11 | 선물 전일 대비율 |
| -futs_prdy_clpr | 선물 전일 종가 | String | Y | 14 | 선물 전일 종가 |
| -acml_vol | 누적 거래량 | String | Y | 18 | 누적 거래량 |
| -acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 | 누적 거래 대금 |
| -hts_kor_isnm | HTS 한글 종목명 | String | Y | 40 | HTS 한글 종목명 |
| -futs_prpr | 현재가 | String | Y | 14 | 현재가 |
| -futs_shrn_iscd | 단축 종목코드 | String | Y | 9 | 단축 종목코드 |
| -prdy_vol | 전일 거래량 | String | Y | 18 | 전일 거래량 |
| -futs_mxpr | 상한가 | String | Y | 14 | 상한가 |
| -futs_llam | 하한가 | String | Y | 14 | 하한가 |
| -futs_oprc | 시가 | String | Y | 14 | 시가 |
| -futs_hgpr | 최고가 | String | Y | 14 | 최고가 |
| -futs_lwpr | 최저가 | String | Y | 14 | 최저가 |
| -futs_prdy_oprc | 전일 시가 | String | Y | 14 | 전일 시가 |
| -futs_prdy_hgpr | 전일 최고가 | String | Y | 14 | 전일 최고가 |
| -futs_prdy_lwpr | 전일 최저가 | String | Y | 14 | 전일 최저가 |
| -futs_askp | 매도호가 | String | Y | 14 | 매도호가 |
| -futs_bidp | 매수호가 | String | Y | 14 | 매수호가 |
| -basis | 베이시스 | String | Y | 12 | 베이시스 |
| -kospi200_nmix | KOSPI200 지수 | String | Y | 14 | KOSPI200 지수 |
| -kospi200_prdy_vrss | KOSPI200 전일 대비 | String | Y | 14 | KOSPI200 전일 대비 |
| -kospi200_prdy_ctrt | KOSPI200 전일 대비율 | String | Y | 11 | KOSPI200 전일 대비율 |
| -kospi200_prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 | 전일 대비 부호 |
| -hts_otst_stpl_qty | HTS 미결제 약정 수량 | String | Y | 18 | HTS 미결제 약정 수량 |
| -otst_stpl_qty_icdc | 미결제 약정 수량 증감 | String | Y | 10 | 미결제 약정 수량 증감 |
| -tday_rltv | 당일 체결강도 | String | Y | 14 | 당일 체결강도 |
| -hts_thpr | HTS 이론가 | String | Y | 14 | HTS 이론가 |
| -dprt | 괴리율 | String | Y | 11 | 괴리율 |
| output2 | 기간별 조회데이터 (배열) | Array | Y | 1 | 기간별 조회데이터 (배열) |
| -stck_bsop_date | 영업 일자 | String | Y | 8 | 영업 일자 |
| -futs_prpr | 현재가 | String | Y | 14 | 현재가 |
| -futs_oprc | 시가 | String | Y | 14 | 시가 |
| -futs_hgpr | 최고가 | String | Y | 14 | 최고가 |
| -futs_lwpr | 최저가 | String | Y | 14 | 최저가 |
| -acml_vol | 누적 거래량 | String | Y | 18 | 누적 거래량 |
| -acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 | 누적 거래 대금 |
| -mod_yn | 변경 여부 | String | Y | 1 | 변경 여부 |
