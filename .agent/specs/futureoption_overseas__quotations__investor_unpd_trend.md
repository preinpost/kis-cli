<!-- endpoint: /uapi/overseas-futureoption/v1/quotations/investor-unpd-trend -->
<!-- category: [해외선물옵션] 기본시세 -->
<!-- korean_name: 해외선물 미결제추이 -->

# 해외선물 미결제추이 [해외선물-029]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-futureoption/v1/quotations/investor-unpd-trend
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: HHDDB95030000
- **모의TRID**: 모의투자 미지원

## 개요
해외선물 미결제추이 API입니다.
한국투자 HTS(eFriend Plus) > [1959] 해외선물 미결제추이의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHDDB95030000 |
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
| PROD_ISCD | 상품 | String | Y | 5 | 금리 (GE, ZB, ZF,ZN,ZT), 금속(GC, PA, PL,SI, HG), 농산물(CC, CT,KC, OJ, SB, ZC,ZL, ZM, ZO, ZR, ZS, ZW), 에너지(CL, HO, NG, WBS), 지수(ES, NQ, TF, YM, VX), 축산물(GF, HE, LE), 통화(6A, 6B, 6C, 6E, 6J, 6N, 6S, DX) |
| BSOP_DATE | 일자 | String | Y | 8 | 기준일(ex)20240513) |
| UPMU_GUBUN | 구분 | String | Y | 1 | 0(수량), 1(증감) |
| CTS_KEY | CTS_KEY | String | Y | 16 | 공백 |

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
| output1 | 응답상세 | Object | Y |  |  |
| row_cnt | 응답레코드카운트 | String | Y | 4 |  |
| output2 | 응답상세 | Object Array | Y |  | array |
| prod_iscd | 상품 | String | Y | 5 |  |
| cftc_iscd | CFTC코드 | String | Y | 10 |  |
| bsop_date | 일자 | String | Y | 8 |  |
| bidp_spec | 매수투기 | String | Y | 10 |  |
| askp_spec | 매도투기 | String | Y | 10 |  |
| spread_spec | 스프레드투기 | String | Y | 10 |  |
| bidp_hedge | 매수헤지 | String | Y | 10 |  |
| askp_hedge | 매도헤지 | String | Y | 10 |  |
| hts_otst_smtn | 미결제합계 | String | Y | 10 |  |
| bidp_missing | 매수누락 | String | Y | 10 |  |
| askp_missing | 매도누락 | String | Y | 10 |  |
| bidp_spec_cust | 매수투기고객 | String | Y | 10 |  |
| askp_spec_cust | 매도투기고객 | String | Y | 10 |  |
| spread_spec_cust | 스프레드투기고객 | String | Y | 10 |  |
| bidp_hedge_cust | 매수헤지고객 | String | Y | 10 |  |
| askp_hedge_cust | 매도헤지고객 | String | Y | 10 |  |
| cust_smtn | 고객합계 | String | Y | 10 |  |
