<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-elw-price -->
<!-- category: [국내주식] ELW 시세 -->
<!-- korean_name: ELW 현재가 시세 -->

# ELW 현재가 시세[v1_국내주식-014]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-elw-price
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: FHKEW15010000
- **모의TRID**: FHKEW15010000

## 개요
ELW 현재가 시세 API입니다. ELW 관련 정보를 얻을 수 있습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 40 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHKEW15010000 |
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
| FID_COND_MRKT_DIV_CODE | 조건 시장 분류 코드 | String | Y | 2 | W |
| FID_INPUT_ISCD | 입력 종목코드 | String | Y | 12 | 종목번호 (6자리) |

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
| elw_shrn_iscd | ELW 단축 종목코드 | String | Y | 9 |  |
| hts_kor_isnm | HTS 한글 종목명 | String | Y | 40 |  |
| elw_prpr | ELW 현재가 | String | Y | 10 |  |
| prdy_vrss | 전일 대비 | String | Y | 10 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| prdy_ctrt | 전일 대비율 | String | Y | 11 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| prdy_vrss_vol_rate | 전일 대비 거래량 비율 | String | Y | 13 |  |
| unas_shrn_iscd | 기초자산 단축 종목코드 | String | Y | 9 |  |
| unas_isnm | 기초자산 종목명 | String | Y | 40 |  |
| unas_prpr | 기초자산 현재가 | String | Y | 14 |  |
| unas_prdy_vrss | 기초자산 전일 대비 | String | Y | 14 |  |
| unas_prdy_vrss_sign | 기초자산 전일 대비 부호 | String | Y | 1 |  |
| unas_prdy_ctrt | 기초자산 전일 대비율 | String | Y | 11 |  |
| bidp | 매수호가 | String | Y | 10 |  |
| askp | 매도호가 | String | Y | 10 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
| vol_tnrt | 거래량 회전율 | String | Y | 11 |  |
| elw_oprc | ELW 시가2 | String | Y | 10 |  |
| elw_hgpr | ELW 최고가 | String | Y | 10 |  |
| elw_lwpr | ELW 최저가 | String | Y | 10 |  |
| stck_prdy_clpr | 주식 전일 종가 | String | Y | 10 |  |
| hts_thpr | HTS 이론가 | String | Y | 14 |  |
| dprt | 괴리율 | String | Y | 11 |  |
| atm_cls_name | ATM 구분 명 | String | Y | 10 |  |
| hts_ints_vltl | HTS 내재 변동성 | String | Y | 16 |  |
| acpr | 행사가 | String | Y | 14 |  |
| pvt_scnd_dmrs_prc | 피벗 2차 디저항 가격 | String | Y | 10 |  |
| pvt_frst_dmrs_prc | 피벗 1차 디저항 가격 | String | Y | 10 |  |
| pvt_pont_val | 피벗 포인트 값 | String | Y | 10 |  |
| pvt_frst_dmsp_prc | 피벗 1차 디지지 가격 | String | Y | 10 |  |
| pvt_scnd_dmsp_prc | 피벗 2차 디지지 가격 | String | Y | 10 |  |
| dmsp_val | 디지지 값 | String | Y | 10 |  |
| dmrs_val | 디저항 값 | String | Y | 10 |  |
| elw_sdpr | ELW 기준가 | String | Y | 10 |  |
| apprch_rate | 접근도 | String | Y | 14 |  |
| tick_conv_prc | 틱환산가 | String | Y | 11 |  |
| invt_epmd_cntt | 투자 유의 내용 | String | Y | 200 |  |
