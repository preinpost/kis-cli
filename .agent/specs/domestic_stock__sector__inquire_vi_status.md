<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-vi-status -->
<!-- category: [국내주식] 업종/기타 -->
<!-- korean_name: 변동성완화장치(VI) 현황 -->

# 변동성완화장치(VI) 현황 [v1_국내주식-055]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-vi-status
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: FHPST01390000
- **모의TRID**: 모의투자 미지원

## 개요
HTS(eFriend Plus) [0139] 변동성 완화장치(VI) 현황 데이터를 확인할 수 있는 API입니다.
최근 30건까지 확인 가능합니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHPST01390000 |
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
| FID_DIV_CLS_CODE | FID 분류 구분 코드 | String | Y | 2 | 0:전체 1:상승 2:하락 |
| FID_COND_SCR_DIV_CODE | FID 조건 화면 분류 코드 | String | Y | 5 | 20139 |
| FID_MRKT_CLS_CODE | FID 시장 구분 코드 | String | Y | 2 | 0:전체 K:거래소 Q:코스닥 |
| FID_INPUT_ISCD | FID 입력 종목코드 | String | Y | 12 |  |
| FID_RANK_SORT_CLS_CODE | FID 순위 정렬 구분 코드 | String | Y | 2 | 0:전체1:정적2:동적3:정적&동적 |
| FID_INPUT_DATE_1 | FID 입력 날짜1 | String | Y | 10 | 영업일 |
| FID_TRGT_CLS_CODE | FID 대상 구분 코드 | String | Y | 32 |  |
| FID_TRGT_EXLS_CLS_CODE | FID 대상 제외 구분 코드 | String | Y | 32 |  |

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
| hts_kor_isnm | HTS 한글 종목명 | String | Y | 40 |  |
| mksc_shrn_iscd | 유가증권 단축 종목코드 | String | Y | 9 |  |
| vi_cls_code | VI발동상태 | String | Y | 1 | Y: 발동 / N: 해제 |
| bsop_date | 영업 일자 | String | Y | 8 |  |
| cntg_vi_hour | VI발동시간 | String | Y | 6 | VI발동시간 |
| vi_cncl_hour | VI해제시간 | String | Y | 6 | VI해제시간 |
| vi_kind_code | VI종류코드 | String | Y | 1 | 1:정적 2:동적 3:정적&동적 |
| vi_prc | VI발동가격 | String | Y | 10 |  |
| vi_stnd_prc | 정적VI발동기준가격 | String | Y | 10 |  |
| vi_dprt | 정적VI발동괴리율 | String | Y | 82 | % |
| vi_dmc_stnd_prc | 동적VI발동기준가격 | String | Y | 10 |  |
| vi_dmc_dprt | 동적VI발동괴리율 | String | Y | 82 | % |
| vi_count | VI발동횟수 | String | Y | 7 |  |
