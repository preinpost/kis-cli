<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-overtimeprice -->
<!-- category: [국내주식] 기본시세 -->
<!-- korean_name: 주식현재가 시간외일자별주가 -->

# 주식현재가 시간외일자별주가[v1_국내주식-026]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-daily-overtimeprice
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: FHPST02320000
- **모의TRID**: FHPST02320000

## 개요
주식현재가 시간외일자별주가 API입니다. (최근일 30건만 조회 가능)
한국투자 HTS(eFriend Plus) > [0232] 시간외 일자별주가의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요!) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자/모의투자]FHPST02320000 |
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
| FID_COND_MRKT_DIV_CODE | FID 조건 시장 분류 코드 | String | Y | 2 | J : 주식, ETF, ETN |
| FID_INPUT_ISCD | FID 입력 종목코드 | String | Y | 12 | 종목번호 (6자리)ETN의 경우, Q로 시작 (EX. Q500001) |

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
| ovtm_untp_prpr | 시간외 단일가 현재가 | String | N | 10 |  |
| ovtm_untp_prdy_vrss | 시간외 단일가 전일 대비 | String | N | 10 |  |
| ovtm_untp_prdy_vrss_sign | 시간외 단일가 전일 대비 부호 | String | N | 1 |  |
| ovtm_untp_prdy_ctrt | 시간외 단일가 전일 대비율 | String | N | 11 | 11(8.2) |
| ovtm_untp_vol | 시간외 단일가 거래량 | String | N | 18 |  |
| ovtm_untp_tr_pbmn | 시간외 단일가 거래 대금 | String | N | 18 |  |
| ovtm_untp_mxpr | 시간외 단일가 상한가 | String | N | 18 |  |
| ovtm_untp_llam | 시간외 단일가 하한가 | String | N | 18 |  |
| ovtm_untp_oprc | 시간외 단일가 시가2 | String | N | 10 |  |
| ovtm_untp_hgpr | 시간외 단일가 최고가 | String | N | 10 |  |
| ovtm_untp_lwpr | 시간외 단일가 최저가 | String | N | 10 |  |
| ovtm_untp_antc_cnpr | 시간외 단일가 예상 체결가 | String | N | 10 |  |
| ovtm_untp_antc_cntg_vrss | 시간외 단일가 예상 체결 대비 | String | N | 10 |  |
| ovtm_untp_antc_cntg_vrss_sign | 시간외 단일가 예상 체결 대비 | String | N | 1 |  |
| ovtm_untp_antc_cntg_ctrt | 시간외 단일가 예상 체결 대비율 | String | N | 11 | 11(8.2) |
| ovtm_untp_antc_vol | 시간외 단일가 예상 거래량 | String | N | 18 |  |
| output2 | 응답상세2 | Object Array | N |  | Array 일자별 정보 |
| stck_bsop_date | 주식 영업 일자 | String | N | 8 |  |
| ovtm_untp_prpr | 시간외 단일가 현재가 | String | N | 10 |  |
| ovtm_untp_prdy_vrss | 시간외 단일가 전일 대비 | String | N | 10 |  |
| ovtm_untp_prdy_vrss_sign | 시간외 단일가 전일 대비 부호 | String | N | 1 |  |
| ovtm_untp_prdy_ctrt | 시간외 단일가 전일 대비율 | String | N | 11 | 11(8.2) |
| ovtm_untp_vol | 시간외 단일가 거래량 | String | N | 18 |  |
| stck_clpr | 주식 종가 | String | N | 10 |  |
| prdy_vrss | 전일 대비 | String | N | 10 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | N | 1 |  |
| prdy_ctrt | 전일 대비율 | String | N | 11 | 11(8.2) |
| acml_vol | 누적 거래량 | String | N | 18 |  |
| ovtm_untp_tr_pbmn | 시간외 단일가 거래대금 | String | N | 18 |  |
