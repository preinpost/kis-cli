<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice -->
<!-- category: [국내주식] 기본시세 -->
<!-- korean_name: 국내주식기간별시세(일/주/월/년) -->

# 국내주식기간별시세(일/주/월/년)[v1_국내주식-016]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: FHKST03010100
- **모의TRID**: FHKST03010100

## 개요
국내주식기간별시세(일/주/월/년) API입니다.
실전계좌/모의계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능합니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHKST03010100 |
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
| FID_INPUT_DATE_1 | 입력 날짜 1 | String | Y | 10 | 조회 시작일자 |
| FID_INPUT_DATE_2 | 입력 날짜 2 | String | Y | 10 | 조회 종료일자 (최대 100개) |
| FID_PERIOD_DIV_CODE | 기간분류코드 | String | Y | 32 | D:일봉 W:주봉, M:월봉, Y:년봉 |
| FID_ORG_ADJ_PRC | 수정주가 원주가 가격 여부 | String | Y | 10 | 0:수정주가 1:원주가 |

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
| prdy_vrss | 전일 대비 | String | Y | 10 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| prdy_ctrt | 전일 대비율 | String | Y | 11 |  |
| stck_prdy_clpr | 주식 전일 종가 | String | Y | 10 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
| hts_kor_isnm | HTS 한글 종목명 | String | Y | 40 |  |
| stck_prpr | 주식 현재가 | String | Y | 10 |  |
| stck_shrn_iscd | 주식 단축 종목코드 | String | Y | 9 |  |
| prdy_vol | 전일 거래량 | String | Y | 18 |  |
| stck_mxpr | 주식 상한가 | String | Y | 10 |  |
| stck_llam | 주식 하한가 | String | Y | 10 |  |
| stck_oprc | 주식 시가2 | String | Y | 10 |  |
| stck_hgpr | 주식 최고가 | String | Y | 10 |  |
| stck_lwpr | 주식 최저가 | String | Y | 10 |  |
| stck_prdy_oprc | 주식 전일 시가 | String | Y | 10 |  |
| stck_prdy_hgpr | 주식 전일 최고가 | String | Y | 10 |  |
| stck_prdy_lwpr | 주식 전일 최저가 | String | Y | 10 |  |
| askp | 매도호가 | String | Y | 10 |  |
| bidp | 매수호가 | String | Y | 10 |  |
| prdy_vrss_vol | 전일 대비 거래량 | String | Y | 18 |  |
| vol_tnrt | 거래량 회전율 | String | Y | 11 | 11(8.2) |
| stck_fcam | 주식 액면가 | String | Y | 11 |  |
| lstn_stcn | 상장 주수 | String | Y | 18 |  |
| cpfn | 자본금 | String | Y | 22 |  |
| hts_avls | HTS 시가총액 | String | Y | 18 |  |
| per | PER | String | Y | 11 | 11(8.2) |
| eps | EPS | String | Y | 14 | 14(11.2) |
| pbr | PBR | String | Y | 11 | 11(8.2) |
| itewhol_loan_rmnd_ratem | 전체 융자 잔고 비율 | String | Y | 13 | 13(8.4) |
| output2 | 응답상세 | Object Array | Y |  | Array |
| stck_bsop_date | 주식 영업 일자 | String | Y | 8 |  |
| stck_clpr | 주식 종가 | String | Y | 10 |  |
| stck_oprc | 주식 시가2 | String | Y | 10 |  |
| stck_hgpr | 주식 최고가 | String | Y | 10 |  |
| stck_lwpr | 주식 최저가 | String | Y | 10 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
| flng_cls_code | 락 구분 코드 | String | Y | 2 | 01 : 권리락02 : 배당락03 : 분배락04 : 권배락05 : 중간(분기)배당락06 : 권리중간배당락07 : 권리분기배당락 |
| prtt_rate | 분할 비율 | String | Y | 11 | 기준가/전일 종가 |
| mod_yn | 변경 여부 | String | Y | 1 | 현재 영업일에 체결이 발생하지 않아 시가가 없을경우 Y 로 표시(차트에서 사용) |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| prdy_vrss | 전일 대비 | String | Y | 10 |  |
| revl_issu_reas | 재평가사유코드 | String | Y | 2 | 00:해당없음01:회사분할02:자본감소03:장기간정지04:초과분배05:대규모배당06:회사분할합병07:ETN증권병합/분할08:신종증권기세조정99:기타 |
