<!-- endpoint: /uapi/domestic-stock/v1/quotations/lendable-by-company -->
<!-- category: [국내주식] 종목정보 -->
<!-- korean_name: 당사 대주가능 종목 -->

# 당사 대주가능 종목 [국내주식-195]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/lendable-by-company
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: CTSC2702R
- **모의TRID**: 모의투자 미지원

## 개요
당사 대주가능 종목 API입니다.
한국투자 HTS(eFriend Plus) > [0490] 당사 대주가능 종목 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
※ 본 API는 다음조회가 불가합니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTSC2702R |
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
| EXCG_DVSN_CD | 거래소구분코드 | String | Y | 2 | 00(전체), 02(거래소), 03(코스닥) |
| PDNO | 상품번호 | String | Y | 12 | 공백 : 전체조회, 종목코드 입력 시 해당종목만 조회 |
| THCO_STLN_PSBL_YN | 당사대주가능여부 | String | Y | 1 | Y |
| INQR_DVSN_1 | 조회구분1 | String | Y | 1 | 0 : 전체조회, 1: 종목코드순 정렬 |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | Y | 200 | 미입력 (다음조회 불가) |
| CTX_AREA_NK100 | 연속조회키100 | String | Y | 100 | 미입력 (다음조회 불가) |

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
| pdno | 상품번호 | String | Y | 12 |  |
| prdt_name | 상품명 | String | Y | 60 |  |
| papr | 액면가 | String | Y | 19 |  |
| bfdy_clpr | 전일종가 | String | Y | 19 | 전일종가 |
| sbst_prvs | 대용가 | String | Y | 19 |  |
| tr_stop_dvsn_name | 거래정지구분명 | String | Y | 60 |  |
| psbl_yn_name | 가능여부명 | String | Y | 60 |  |
| lmt_qty1 | 한도수량1 | String | Y | 19 |  |
| use_qty1 | 사용수량1 | String | Y | 19 |  |
| trad_psbl_qty2 | 매매가능수량2 | String | Y | 19 | 가능수량 |
| rght_type_cd | 권리유형코드 | String | Y | 2 |  |
| bass_dt | 기준일자 | String | Y | 8 |  |
| psbl_yn | 가능여부 | String | Y | 1 |  |
| output2 | 응답상세 | Object | Y |  |  |
| tot_stup_lmt_qty | 총설정한도수량 | String | Y | 19 |  |
| brch_lmt_qty | 지점한도수량 | String | Y | 19 |  |
| rqst_psbl_qty | 신청가능수량 | String | Y | 19 |  |
