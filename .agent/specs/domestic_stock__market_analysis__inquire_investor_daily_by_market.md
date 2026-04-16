<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-investor-daily-by-market -->
<!-- category: [국내주식] 시세분석 -->
<!-- korean_name: 시장별 투자자매매동향(일별) -->

# 시장별 투자자매매동향(일별) [국내주식-075]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-investor-daily-by-market
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: FHPTJ04040000
- **모의TRID**: 모의투자 미지원

## 개요
시장별 투자자매매동향(일별) API입니다.
한국투자 HTS(eFriend Plus) > [0404] 시장별 일별동향 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHPTJ04040000 |
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
| FID_COND_MRKT_DIV_CODE | 조건 시장 분류 코드 | String | Y | 2 | 시장구분코드 (업종 U) |
| FID_INPUT_ISCD | 입력 종목코드 | String | Y | 12 | 코스피, 코스닥 : 업종분류코드 (종목정보파일 - 업종코드 참조) |
| FID_INPUT_DATE_1 | 입력 날짜1 | String | Y | 10 | ex. 20240517 |
| FID_INPUT_ISCD_1 | 입력 종목코드 | String | Y | 12 | 코스피(KSP), 코스닥(KSQ) |
| FID_INPUT_DATE_2 | 입력 날짜2 | String | Y | 10 | 입력 날짜1과 동일날짜 입력 |
| FID_INPUT_ISCD_2 | 하위 분류코드 | String | Y | 10 | 코스피, 코스닥 : 업종분류코드 (종목정보파일 - 업종코드 참조) |

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
| stck_bsop_date | 주식 영업 일자 | String | Y | 8 |  |
| bstp_nmix_prpr | 업종 지수 현재가 | String | Y | 112 |  |
| bstp_nmix_prdy_vrss | 업종 지수 전일 대비 | String | Y | 112 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| bstp_nmix_prdy_ctrt | 업종 지수 전일 대비율 | String | Y | 82 |  |
| bstp_nmix_oprc | 업종 지수 시가2 | String | Y | 112 |  |
| bstp_nmix_hgpr | 업종 지수 최고가 | String | Y | 112 |  |
| bstp_nmix_lwpr | 업종 지수 최저가 | String | Y | 112 |  |
| stck_prdy_clpr | 주식 전일 종가 | String | Y | 10 |  |
| frgn_ntby_qty | 외국인 순매수 수량 | String | Y | 12 |  |
| frgn_reg_ntby_qty | 외국인 등록 순매수 수량 | String | Y | 18 |  |
| frgn_nreg_ntby_qty | 외국인 비등록 순매수 수량 | String | Y | 18 |  |
| prsn_ntby_qty | 개인 순매수 수량 | String | Y | 12 |  |
| orgn_ntby_qty | 기관계 순매수 수량 | String | Y | 18 |  |
| scrt_ntby_qty | 증권 순매수 수량 | String | Y | 12 |  |
| ivtr_ntby_qty | 투자신탁 순매수 수량 | String | Y | 12 |  |
| pe_fund_ntby_vol | 사모 펀드 순매수 거래량 | String | Y | 18 |  |
| bank_ntby_qty | 은행 순매수 수량 | String | Y | 12 |  |
| insu_ntby_qty | 보험 순매수 수량 | String | Y | 12 |  |
| mrbn_ntby_qty | 종금 순매수 수량 | String | Y | 12 |  |
| fund_ntby_qty | 기금 순매수 수량 | String | Y | 12 |  |
| etc_ntby_qty | 기타 순매수 수량 | String | Y | 12 |  |
| etc_orgt_ntby_vol | 기타 단체 순매수 거래량 | String | Y | 18 |  |
| etc_corp_ntby_vol | 기타 법인 순매수 거래량 | String | Y | 18 |  |
| frgn_ntby_tr_pbmn | 외국인 순매수 거래 대금 | String | Y | 18 |  |
| frgn_reg_ntby_pbmn | 외국인 등록 순매수 대금 | String | Y | 18 |  |
| frgn_nreg_ntby_pbmn | 외국인 비등록 순매수 대금 | String | Y | 18 |  |
| prsn_ntby_tr_pbmn | 개인 순매수 거래 대금 | String | Y | 18 |  |
| orgn_ntby_tr_pbmn | 기관계 순매수 거래 대금 | String | Y | 18 |  |
| scrt_ntby_tr_pbmn | 증권 순매수 거래 대금 | String | Y | 18 |  |
| ivtr_ntby_tr_pbmn | 투자신탁 순매수 거래 대금 | String | Y | 18 |  |
| pe_fund_ntby_tr_pbmn | 사모 펀드 순매수 거래 대금 | String | Y | 18 |  |
| bank_ntby_tr_pbmn | 은행 순매수 거래 대금 | String | Y | 18 |  |
| insu_ntby_tr_pbmn | 보험 순매수 거래 대금 | String | Y | 18 |  |
| mrbn_ntby_tr_pbmn | 종금 순매수 거래 대금 | String | Y | 18 |  |
| fund_ntby_tr_pbmn | 기금 순매수 거래 대금 | String | Y | 18 |  |
| etc_ntby_tr_pbmn | 기타 순매수 거래 대금 | String | Y | 18 |  |
| etc_orgt_ntby_tr_pbmn | 기타 단체 순매수 거래 대금 | String | Y | 18 |  |
| etc_corp_ntby_tr_pbmn | 기타 법인 순매수 거래 대금 | String | Y | 18 |  |
