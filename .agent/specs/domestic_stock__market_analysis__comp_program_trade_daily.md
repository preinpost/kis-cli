<!-- endpoint: /uapi/domestic-stock/v1/quotations/comp-program-trade-daily -->
<!-- category: [국내주식] 시세분석 -->
<!-- korean_name: 프로그램매매 종합현황(일별) -->

# 프로그램매매 종합현황(일별)[국내주식-115]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/comp-program-trade-daily
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: FHPPG04600001
- **모의TRID**: 모의투자 미지원

## 개요
프로그램매매 종합현황(일별) API입니다.
한국투자 HTS(eFriend Plus) > [0460] 프로그램매매 종합현황 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
* 8개월 이상 과거 조회는 불가하며 에러메시지가 발생합니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | '※ 구TR은 사전고지 없이 막힐 수 있으므로 반드시 신TR로 변경이용 부탁드립니다.[실전투자](구)FHPPG04600000 → (신)FHPPG04600001' |
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
| FID_COND_MRKT_DIV_CODE | 시장 분류 코드 | String | Y | 2 | J : KRX, NX : NXT, UN : 통합 |
| FID_MRKT_CLS_CODE | 시장 구분 코드 | String | Y | 2 | K:코스피, Q:코스닥 |
| FID_INPUT_DATE_1 | 검색시작일 | String | Y | 10 | 공백 입력, 입력 시 ~ 입력일자까지 조회됨* 8개월 이상 과거 조회 불가 |
| FID_INPUT_DATE_2 | 검색종료일 | String | Y | 10 | 공백 입력 |

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
| nabt_entm_seln_tr_pbmn | 비차익 위탁 매도 거래 대금 | String | Y | 18 |  |
| nabt_onsl_seln_vol | 비차익 자기 매도 거래량 | String | Y | 18 |  |
| whol_onsl_seln_tr_pbmn | 전체 자기 매도 거래 대금 | String | Y | 18 |  |
| arbt_smtn_shnu_vol | 차익 합계 매수2 거래량 | String | Y | 18 |  |
| nabt_smtn_shnu_tr_pbmn | 비차익 합계 매수2 거래 대금 | String | Y | 18 |  |
| arbt_entm_ntby_qty | 차익 위탁 순매수 수량 | String | Y | 18 |  |
| nabt_entm_ntby_tr_pbmn | 비차익 위탁 순매수 거래 대금 | String | Y | 18 |  |
| arbt_entm_seln_vol | 차익 위탁 매도 거래량 | String | Y | 18 |  |
| nabt_entm_seln_vol_rate | 비차익 위탁 매도 거래량 비율 | String | Y | 82 |  |
| nabt_onsl_seln_vol_rate | 비차익 자기 매도 거래량 비율 | String | Y | 82 |  |
| whol_onsl_seln_tr_pbmn_rate | 전체 자기 매도 거래 대금 비율 | String | Y | 82 |  |
| arbt_smtm_shun_vol_rate | 차익 합계 매수 거래량 비율 | String | Y | 72 |  |
| nabt_smtm_shun_tr_pbmn_rate | 비차익 합계 매수 거래대금 비율 | String | Y | 72 |  |
| arbt_entm_ntby_qty_rate | 차익 위탁 순매수 수량 비율 | String | Y | 82 |  |
| nabt_entm_ntby_tr_pbmn_rate | 비차익 위탁 순매수 거래 대금 | String | Y | 82 |  |
| arbt_entm_seln_vol_rate | 차익 위탁 매도 거래량 비율 | String | Y | 82 |  |
| nabt_entm_seln_tr_pbmn_rate | 비차익 위탁 매도 거래 대금 비 | String | Y | 82 |  |
| nabt_onsl_seln_tr_pbmn | 비차익 자기 매도 거래 대금 | String | Y | 18 |  |
| whol_smtn_seln_vol | 전체 합계 매도 거래량 | String | Y | 18 |  |
| arbt_smtn_shnu_tr_pbmn | 차익 합계 매수2 거래 대금 | String | Y | 18 |  |
| whol_entm_shnu_vol | 전체 위탁 매수2 거래량 | String | Y | 18 |  |
| arbt_entm_ntby_tr_pbmn | 차익 위탁 순매수 거래 대금 | String | Y | 18 |  |
| nabt_onsl_ntby_qty | 비차익 자기 순매수 수량 | String | Y | 18 |  |
| arbt_entm_seln_tr_pbmn | 차익 위탁 매도 거래 대금 | String | Y | 18 |  |
| nabt_onsl_seln_tr_pbmn_rate | 비차익 자기 매도 거래 대금 비 | String | Y | 82 |  |
| whol_seln_vol_rate | 전체 매도 거래량 비율 | String | Y | 72 |  |
| arbt_smtm_shun_tr_pbmn_rate | 차익 합계 매수 거래대금 비율 | String | Y | 72 |  |
| whol_entm_shnu_vol_rate | 전체 위탁 매수 거래량 비율 | String | Y | 82 |  |
| arbt_entm_ntby_tr_pbmn_rate | 차익 위탁 순매수 거래 대금 비 | String | Y | 82 |  |
| nabt_onsl_ntby_qty_rate | 비차익 자기 순매수 수량 비율 | String | Y | 82 |  |
| arbt_entm_seln_tr_pbmn_rate | 차익 위탁 매도 거래 대금 비율 | String | Y | 82 |  |
| nabt_smtn_seln_vol | 비차익 합계 매도 거래량 | String | Y | 18 |  |
| whol_smtn_seln_tr_pbmn | 전체 합계 매도 거래 대금 | String | Y | 18 |  |
| nabt_entm_shnu_vol | 비차익 위탁 매수2 거래량 | String | Y | 18 |  |
| whol_entm_shnu_tr_pbmn | 전체 위탁 매수2 거래 대금 | String | Y | 18 |  |
| arbt_onsl_ntby_qty | 차익 자기 순매수 수량 | String | Y | 18 |  |
| nabt_onsl_ntby_tr_pbmn | 비차익 자기 순매수 거래 대금 | String | Y | 18 |  |
| arbt_onsl_seln_tr_pbmn | 차익 자기 매도 거래 대금 | String | Y | 18 |  |
| nabt_smtm_seln_vol_rate | 비차익 합계 매도 거래량 비율 | String | Y | 72 |  |
| whol_seln_tr_pbmn_rate | 전체 매도 거래대금 비율 | String | Y | 72 |  |
| nabt_entm_shnu_vol_rate | 비차익 위탁 매수 거래량 비율 | String | Y | 82 |  |
| whol_entm_shnu_tr_pbmn_rate | 전체 위탁 매수 거래 대금 비율 | String | Y | 82 |  |
| arbt_onsl_ntby_qty_rate | 차익 자기 순매수 수량 비율 | String | Y | 82 |  |
| nabt_onsl_ntby_tr_pbmn_rate | 비차익 자기 순매수 거래 대금 | String | Y | 82 |  |
| arbt_onsl_seln_tr_pbmn_rate | 차익 자기 매도 거래 대금 비율 | String | Y | 82 |  |
| nabt_smtn_seln_tr_pbmn | 비차익 합계 매도 거래 대금 | String | Y | 18 |  |
| arbt_entm_shnu_vol | 차익 위탁 매수2 거래량 | String | Y | 18 |  |
| nabt_entm_shnu_tr_pbmn | 비차익 위탁 매수2 거래 대금 | String | Y | 18 |  |
| whol_onsl_shnu_vol | 전체 자기 매수2 거래량 | String | Y | 18 |  |
| arbt_onsl_ntby_tr_pbmn | 차익 자기 순매수 거래 대금 | String | Y | 18 |  |
| nabt_smtn_ntby_qty | 비차익 합계 순매수 수량 | String | Y | 18 |  |
| arbt_onsl_seln_vol | 차익 자기 매도 거래량 | String | Y | 18 |  |
| nabt_smtm_seln_tr_pbmn_rate | 비차익 합계 매도 거래대금 비율 | String | Y | 72 |  |
| arbt_entm_shnu_vol_rate | 차익 위탁 매수 거래량 비율 | String | Y | 82 |  |
| nabt_entm_shnu_tr_pbmn_rate | 비차익 위탁 매수 거래 대금 비 | String | Y | 82 |  |
| whol_onsl_shnu_tr_pbmn | 전체 자기 매수2 거래 대금 | String | Y | 18 |  |
| arbt_onsl_ntby_tr_pbmn_rate | 차익 자기 순매수 거래 대금 비 | String | Y | 82 |  |
| nabt_smtm_ntby_qty_rate | 비차익 합계 순매수 수량 비율 | String | Y | 72 |  |
| arbt_onsl_seln_vol_rate | 차익 자기 매도 거래량 비율 | String | Y | 82 |  |
| whol_entm_seln_vol | 전체 위탁 매도 거래량 | String | Y | 18 |  |
| arbt_entm_shnu_tr_pbmn | 차익 위탁 매수2 거래 대금 | String | Y | 18 |  |
| nabt_onsl_shnu_vol | 비차익 자기 매수2 거래량 | String | Y | 18 |  |
| whol_onsl_shnu_tr_pbmn_rate | 전체 자기 매수 거래 대금 비율 | String | Y | 82 |  |
| arbt_smtn_ntby_qty | 차익 합계 순매수 수량 | String | Y | 18 |  |
| nabt_smtn_ntby_tr_pbmn | 비차익 합계 순매수 거래 대금 | String | Y | 18 |  |
| arbt_smtn_seln_vol | 차익 합계 매도 거래량 | String | Y | 18 |  |
| whol_entm_seln_tr_pbmn | 전체 위탁 매도 거래 대금 | String | Y | 18 |  |
| arbt_entm_shnu_tr_pbmn_rate | 차익 위탁 매수 거래 대금 비율 | String | Y | 82 |  |
| nabt_onsl_shnu_vol_rate | 비차익 자기 매수 거래량 비율 | String | Y | 82 |  |
| whol_onsl_shnu_vol_rate | 전체 자기 매수 거래량 비율 | String | Y | 82 |  |
| arbt_smtm_ntby_qty_rate | 차익 합계 순매수 수량 비율 | String | Y | 72 |  |
| nabt_smtm_ntby_tr_pbmn_rate | 비차익 합계 순매수 거래대금 비 | String | Y | 72 |  |
| arbt_smtm_seln_vol_rate | 차익 합계 매도 거래량 비율 | String | Y | 72 |  |
| whol_entm_seln_vol_rate | 전체 위탁 매도 거래량 비율 | String | Y | 82 |  |
| arbt_onsl_shnu_vol | 차익 자기 매수2 거래량 | String | Y | 18 |  |
| nabt_onsl_shnu_tr_pbmn | 비차익 자기 매수2 거래 대금 | String | Y | 18 |  |
| whol_smtn_shnu_vol | 전체 합계 매수2 거래량 | String | Y | 18 |  |
| arbt_smtn_ntby_tr_pbmn | 차익 합계 순매수 거래 대금 | String | Y | 18 |  |
| whol_entm_ntby_qty | 전체 위탁 순매수 수량 | String | Y | 18 |  |
| arbt_smtn_seln_tr_pbmn | 차익 합계 매도 거래 대금 | String | Y | 18 |  |
| whol_entm_seln_tr_pbmn_rate | 전체 위탁 매도 거래 대금 비율 | String | Y | 82 |  |
| arbt_onsl_shnu_vol_rate | 차익 자기 매수 거래량 비율 | String | Y | 82 |  |
| nabt_onsl_shnu_tr_pbmn_rate | 비차익 자기 매수 거래 대금 비 | String | Y | 82 |  |
| whol_shun_vol_rate | 전체 매수 거래량 비율 | String | Y | 72 |  |
| arbt_smtm_ntby_tr_pbmn_rate | 차익 합계 순매수 거래대금 비율 | String | Y | 72 |  |
| whol_entm_ntby_qty_rate | 전체 위탁 순매수 수량 비율 | String | Y | 82 |  |
| arbt_smtm_seln_tr_pbmn_rate | 차익 합계 매도 거래대금 비율 | String | Y | 72 |  |
| whol_onsl_seln_vol | 전체 자기 매도 거래량 | String | Y | 18 |  |
| arbt_onsl_shnu_tr_pbmn | 차익 자기 매수2 거래 대금 | String | Y | 18 |  |
| nabt_smtn_shnu_vol | 비차익 합계 매수2 거래량 | String | Y | 18 |  |
| whol_smtn_shnu_tr_pbmn | 전체 합계 매수2 거래 대금 | String | Y | 18 |  |
| nabt_entm_ntby_qty | 비차익 위탁 순매수 수량 | String | Y | 18 |  |
| whol_entm_ntby_tr_pbmn | 전체 위탁 순매수 거래 대금 | String | Y | 18 |  |
| nabt_entm_seln_vol | 비차익 위탁 매도 거래량 | String | Y | 18 |  |
| whol_onsl_seln_vol_rate | 전체 자기 매도 거래량 비율 | String | Y | 82 |  |
| arbt_onsl_shnu_tr_pbmn_rate | 차익 자기 매수 거래 대금 비율 | String | Y | 82 |  |
| nabt_smtm_shun_vol_rate | 비차익 합계 매수 거래량 비율 | String | Y | 72 |  |
| whol_shun_tr_pbmn_rate | 전체 매수 거래대금 비율 | String | Y | 72 |  |
| nabt_entm_ntby_qty_rate | 비차익 위탁 순매수 수량 비율 | String | Y | 82 |  |
