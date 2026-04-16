<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-indexchartprice -->
<!-- category: [국내주식] 업종/기타 -->
<!-- korean_name: 국내주식업종기간별시세(일/주/월/년) -->

# 국내주식업종기간별시세(일/주/월/년)[v1_국내주식-021]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-daily-indexchartprice
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: FHKUP03500100
- **모의TRID**: FHKUP03500100

## 개요
국내주식 업종기간별시세(일/주/월/년) API입니다.
실전계좌/모의계좌의 경우, 한 번의 호출에 최대 50건까지 확인 가능합니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 40 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | '[실전투자/모의투자]FHKUP03500100' |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 필수] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| FID_COND_MRKT_DIV_CODE | 조건 시장 분류 코드 | String | Y | 2 | 업종 : U |
| FID_INPUT_ISCD | 업종 상세코드 | String | Y | 2 | '0001 : 종합0002 : 대형주...포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)' |
| FID_INPUT_DATE_1 | 조회 시작일자 | String | Y | 10 | 조회 시작일자 (ex. 20220501) |
| FID_INPUT_DATE_2 | 조회 종료일자 | String | Y | 10 | 조회 종료일자 (ex. 20220530) |
| FID_PERIOD_DIV_CODE | ' 기간분류코드' | String | Y | 32 | ' D:일봉 W:주봉, M:월봉, Y:년봉' |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| gt_uid | Global UID | String | N | 32 | [법인 필수] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 |  |
| msg_cd | 응답코드 | String | Y | 8 |  |
| msg1 | 응답메세지 | String | Y | 80 |  |
| output1 | 응답상세 | Object | Y |  | Single |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| bstp_nmix_prdy_ctrt | 업종 지수 전일 대비율 | String | Y | 82 |  |
| prdy_nmix | 전일 지수 | String | Y | 112 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
| hts_kor_isnm | HTS 한글 종목명 | String | Y | 40 |  |
| bstp_nmix_prpr | 업종 지수 현재가 | String | Y | 112 |  |
| bstp_cls_code | 업종 구분 코드 | String | Y | 4 |  |
| prdy_vol | 전일 거래량 | String | Y | 18 |  |
| bstp_nmix_oprc | 업종 지수 시가2 | String | Y | 112 |  |
| bstp_nmix_hgpr | 업종 지수 최고가 | String | Y | 112 |  |
| bstp_nmix_lwpr | 업종 지수 최저가 | String | Y | 112 |  |
| futs_prdy_oprc | 선물 전일 시가 | String | Y | 112 |  |
| futs_prdy_hgpr | 선물 전일 최고가 | String | Y | 112 |  |
| futs_prdy_lwpr | 선물 전일 최저가 | String | Y | 112 |  |
| output2 | 응답상세 | Object Array | Y |  | Array |
| stck_bsop_date | 주식 영업 일자 | String | Y | 8 |  |
| bstp_nmix_prpr | 업종 지수 현재가 | String | Y | 112 |  |
| bstp_nmix_oprc | 업종 지수 시가2 | String | Y | 112 |  |
| bstp_nmix_hgpr | 업종 지수 최고가 | String | Y | 112 |  |
| bstp_nmix_lwpr | 업종 지수 최저가 | String | Y | 112 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
| mod_yn | 변경 여부 | String | Y | 1 |  |
