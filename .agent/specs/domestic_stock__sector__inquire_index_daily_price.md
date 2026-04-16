<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-index-daily-price -->
<!-- category: [국내주식] 업종/기타 -->
<!-- korean_name: 국내업종 일자별지수 -->

# 국내업종 일자별지수[v1_국내주식-065]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-index-daily-price
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: FHPUP02120000
- **모의TRID**: 모의투자 미지원

## 개요
국내업종 일자별지수 API입니다. 한 번의 조회에 100건까지 확인 가능합니다.
한국투자 HTS(eFriend Plus) > [0212] 업종 일자별지수 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHPUP02120000 |
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
| FID_PERIOD_DIV_CODE | FID 기간 분류 코드 | String | Y | 32 | 일/주/월 구분코드 ( D:일별 , W:주별, M:월별 ) |
| FID_COND_MRKT_DIV_CODE | FID 조건 시장 분류 코드 | String | Y | 2 | 시장구분코드 (업종 U) |
| FID_INPUT_ISCD | FID 입력 종목코드 | String | Y | 12 | 코스피(0001), 코스닥(1001), 코스피200(2001)...포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조) |
| FID_INPUT_DATE_1 | FID 입력 날짜1 | String | Y | 10 | 입력 날짜(ex. 20240223) |

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
| output1 | 응답상세1 | Object | Y |  |  |
| bstp_nmix_prpr | 업종 지수 현재가 | String | Y | 112 |  |
| bstp_nmix_prdy_vrss | 업종 지수 전일 대비 | String | Y | 112 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| bstp_nmix_prdy_ctrt | 업종 지수 전일 대비율 | String | Y | 82 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
| bstp_nmix_oprc | 업종 지수 시가2 | String | Y | 112 |  |
| bstp_nmix_hgpr | 업종 지수 최고가 | String | Y | 112 |  |
| bstp_nmix_lwpr | 업종 지수 최저가 | String | Y | 112 |  |
| prdy_vol | 전일 거래량 | String | Y | 18 |  |
| ascn_issu_cnt | 상승 종목 수 | String | Y | 7 |  |
| down_issu_cnt | 하락 종목 수 | String | Y | 7 |  |
| stnr_issu_cnt | 보합 종목 수 | String | Y | 7 |  |
| uplm_issu_cnt | 상한 종목 수 | String | Y | 7 |  |
| lslm_issu_cnt | 하한 종목 수 | String | Y | 7 |  |
| prdy_tr_pbmn | 전일 거래 대금 | String | Y | 18 |  |
| dryy_bstp_nmix_hgpr_date | 연중업종지수최고가일자 | String | Y | 8 |  |
| dryy_bstp_nmix_hgpr | 연중업종지수최고가 | String | Y | 112 |  |
| dryy_bstp_nmix_lwpr | 연중업종지수최저가 | String | Y | 112 |  |
| dryy_bstp_nmix_lwpr_date | 연중업종지수최저가일자 | String | Y | 8 |  |
| output2 | 응답상세2 | Object Array | Y |  | array |
| stck_bsop_date | 주식 영업 일자 | String | Y | 8 |  |
| bstp_nmix_prpr | 업종 지수 현재가 | String | Y | 112 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| bstp_nmix_prdy_vrss | 업종 지수 전일 대비 | String | Y | 112 |  |
| bstp_nmix_prdy_ctrt | 업종 지수 전일 대비율 | String | Y | 82 |  |
| bstp_nmix_oprc | 업종 지수 시가2 | String | Y | 112 |  |
| bstp_nmix_hgpr | 업종 지수 최고가 | String | Y | 112 |  |
| bstp_nmix_lwpr | 업종 지수 최저가 | String | Y | 112 |  |
| acml_vol_rlim | 누적 거래량 비중 | String | Y | 72 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
| invt_new_psdg | 투자 신 심리도 | String | Y | 112 |  |
| d20_dsrt | 20일 이격도 | String | Y | 112 |  |
