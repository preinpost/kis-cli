<!-- endpoint: /uapi/domestic-stock/v1/quotations/intstock-multprice -->
<!-- category: [국내주식] 시세분석 -->
<!-- korean_name: 관심종목(멀티종목) 시세조회 -->

# 관심종목(멀티종목) 시세조회 [국내주식-205]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/intstock-multprice
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: FHKST11300006
- **모의TRID**: 모의투자 미지원

## 개요
관심종목(멀티종목) 시세조회 API입니다.
한국투자 HTS(eFriend Plus) > [0161] 관심종목 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHKST11300006 |
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
| FID_COND_MRKT_DIV_CODE_1 | 조건 시장 분류 코드1 | String | Y | 2 | 그룹별종목조회 결과 fid_mrkt_cls_code(시장구분) 1 입력J: KRX, NX: NXT, UN: 통합ex) J |
| FID_INPUT_ISCD_1 | 입력 종목코드1 | String | Y | 16 | 그룹별종목조회 결과 jong_code(종목코드) 1 입력ex) 005930 |
| FID_COND_MRKT_DIV_CODE_2 | 조건 시장 분류 코드2 | String | Y | 2 |  |
| FID_INPUT_ISCD_2 | 입력 종목코드2 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_3 | 조건 시장 분류 코드3 | String | Y | 2 |  |
| FID_INPUT_ISCD_3 | 입력 종목코드3 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_4 | 조건 시장 분류 코드4 | String | Y | 2 |  |
| FID_INPUT_ISCD_4 | 입력 종목코드4 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_5 | 조건 시장 분류 코드5 | String | Y | 2 |  |
| FID_INPUT_ISCD_5 | 입력 종목코드5 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_6 | 조건 시장 분류 코드6 | String | Y | 2 |  |
| FID_INPUT_ISCD_6 | 입력 종목코드6 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_7 | 조건 시장 분류 코드7 | String | Y | 2 |  |
| FID_INPUT_ISCD_7 | 입력 종목코드7 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_8 | 조건 시장 분류 코드8 | String | Y | 2 |  |
| FID_INPUT_ISCD_8 | 입력 종목코드8 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_9 | 조건 시장 분류 코드9 | String | Y | 2 |  |
| FID_INPUT_ISCD_9 | 입력 종목코드9 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_10 | 조건 시장 분류 코드10 | String | Y | 12 |  |
| FID_INPUT_ISCD_10 | 입력 종목코드10 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_11 | 조건 시장 분류 코드11 | String | Y | 2 |  |
| FID_INPUT_ISCD_11 | 입력 종목코드11 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_12 | 조건 시장 분류 코드12 | String | Y | 2 |  |
| FID_INPUT_ISCD_12 | 입력 종목코드12 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_13 | 조건 시장 분류 코드13 | String | Y | 2 |  |
| FID_INPUT_ISCD_13 | 입력 종목코드13 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_14 | 조건 시장 분류 코드14 | String | Y | 2 |  |
| FID_INPUT_ISCD_14 | 입력 종목코드14 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_15 | 조건 시장 분류 코드15 | String | Y | 2 |  |
| FID_INPUT_ISCD_15 | 입력 종목코드15 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_16 | 조건 시장 분류 코드16 | String | Y | 2 |  |
| FID_INPUT_ISCD_16 | 입력 종목코드16 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_17 | 조건 시장 분류 코드17 | String | Y | 2 |  |
| FID_INPUT_ISCD_17 | 입력 종목코드17 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_18 | 조건 시장 분류 코드18 | String | Y | 2 |  |
| FID_INPUT_ISCD_18 | 입력 종목코드18 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_19 | 조건 시장 분류 코드19 | String | Y | 2 |  |
| FID_INPUT_ISCD_19 | 입력 종목코드19 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_20 | 조건 시장 분류 코드20 | String | Y | 2 |  |
| FID_INPUT_ISCD_20 | 입력 종목코드20 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_21 | 조건 시장 분류 코드21 | String | Y | 2 |  |
| FID_INPUT_ISCD_21 | 입력 종목코드21 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_22 | 조건 시장 분류 코드22 | String | Y | 2 |  |
| FID_INPUT_ISCD_22 | 입력 종목코드22 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_23 | 조건 시장 분류 코드23 | String | Y | 2 |  |
| FID_INPUT_ISCD_23 | 입력 종목코드23 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_24 | 조건 시장 분류 코드24 | String | Y | 2 |  |
| FID_INPUT_ISCD_24 | 입력 종목코드24 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_25 | 조건 시장 분류 코드25 | String | Y | 2 |  |
| FID_INPUT_ISCD_25 | 입력 종목코드25 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_26 | 조건 시장 분류 코드26 | String | Y | 16 |  |
| FID_INPUT_ISCD_26 | 입력 종목코드26 | String | Y | 2 |  |
| FID_COND_MRKT_DIV_CODE_27 | 조건 시장 분류 코드27 | String | Y | 2 |  |
| FID_INPUT_ISCD_27 | 입력 종목코드27 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_28 | 조건 시장 분류 코드28 | String | Y | 2 |  |
| FID_INPUT_ISCD_28 | 입력 종목코드28 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_29 | 조건 시장 분류 코드29 | String | Y | 2 |  |
| FID_INPUT_ISCD_29 | 입력 종목코드29 | String | Y | 16 |  |
| FID_COND_MRKT_DIV_CODE_30 | 조건 시장 분류 코드30 | String | Y | 2 |  |
| FID_INPUT_ISCD_30 | 입력 종목코드30 | String | Y | 16 |  |

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
| output | 응답상세 | Object | Y |  |  |
| kospi_kosdaq_cls_name | 코스피 코스닥 구분 명 | String | Y | 10 |  |
| mrkt_trtm_cls_name | 시장 조치 구분 명 | String | Y | 10 |  |
| hour_cls_code | 시간 구분 코드 | String | Y | 1 |  |
| inter_shrn_iscd | 관심 단축 종목코드 | String | Y | 16 |  |
| inter_kor_isnm | 관심 한글 종목명 | String | Y | 40 |  |
| inter2_prpr | 관심2 현재가 | String | Y | 11 |  |
| inter2_prdy_vrss | 관심2 전일 대비 | String | Y | 11 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| prdy_ctrt | 전일 대비율 | String | Y | 82 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| inter2_oprc | 관심2 시가 | String | Y | 11 |  |
| inter2_hgpr | 관심2 고가 | String | Y | 11 |  |
| inter2_lwpr | 관심2 저가 | String | Y | 11 |  |
| inter2_llam | 관심2 하한가 | String | Y | 11 |  |
| inter2_mxpr | 관심2 상한가 | String | Y | 11 |  |
| inter2_askp | 관심2 매도호가 | String | Y | 11 |  |
| inter2_bidp | 관심2 매수호가 | String | Y | 11 |  |
| seln_rsqn | 매도 잔량 | String | Y | 12 |  |
| shnu_rsqn | 매수2 잔량 | String | Y | 12 |  |
| total_askp_rsqn | 총 매도호가 잔량 | String | Y | 12 |  |
| total_bidp_rsqn | 총 매수호가 잔량 | String | Y | 12 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
| inter2_prdy_clpr | 관심2 전일 종가 | String | Y | 11 |  |
| oprc_vrss_hgpr_rate | 시가 대비 최고가 비율 | String | Y | 84 |  |
| intr_antc_cntg_vrss | 관심 예상 체결 대비 | String | Y | 11 |  |
| intr_antc_cntg_vrss_sign | 관심 예상 체결 대비 부호 | String | Y | 1 |  |
| intr_antc_cntg_prdy_ctrt | 관심 예상 체결 전일 대비율 | String | Y | 72 |  |
| intr_antc_vol | 관심 예상 거래량 | String | Y | 18 |  |
| inter2_sdpr | 관심2 기준가 | String | Y | 11 |  |
