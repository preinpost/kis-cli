<!-- endpoint: /uapi/overseas-price/v1/quotations/colable-by-company -->
<!-- category: [해외주식] 시세분석 -->
<!-- korean_name: 당사 해외주식담보대출 가능 종목 -->

# 당사 해외주식담보대출 가능 종목 [해외주식-051]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-price/v1/quotations/colable-by-company
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: CTLN4050R
- **모의TRID**: 모의투자 미지원

## 개요
당사 해외주식담보대출 가능 종목 API입니다.
한국투자 HTS(eFriend Plus) > [0497] 당사 해외주식담보대출 가능 종목 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
한 번의 호출에 20건까지 조회가 가능하며 다음조회가 불가하기에, PDNO에 데이터 확인하고자 하는 종목코드를 입력하여 단건조회용으로 사용하시기 바랍니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTLN4050R |
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
| PDNO | 상품번호 | String | Y | 12 | ex)AMD |
| PRDT_TYPE_CD | 상품유형코드 | String | Y | 3 | 공백 |
| INQR_STRT_DT | 조회시작일자 | String | Y | 8 | 공백 |
| INQR_END_DT | 조회종료일자 | String | Y | 8 | 공백 |
| INQR_DVSN | 조회구분 | String | Y | 2 | 공백 |
| NATN_CD | 국가코드 | String | Y | 3 | 840(미국), 344(홍콩), 156(중국) |
| INQR_SQN_DVSN | 조회순서구분 | String | Y | 2 | 01(이름순), 02(코드순) |
| RT_DVSN_CD | 비율구분코드 | String | Y | 2 | 공백 |
| RT | 비율 | String | Y | 238 | 공백 |
| LOAN_PSBL_YN | 대출가능여부 | String | Y | 1 | 공백 |
| CTX_AREA_FK100 | 연속조회검색조건100 | String | Y | 100 | 공백 |
| CTX_AREA_NK100 | 연속조회키100 | String | Y | 100 | 공백 |

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
| output1 | 응답상세 | Array | Y |  |  |
| pdno | 상품번호 | String | Y | 12 |  |
| ovrs_item_name | 해외종목명 | String | Y | 60 |  |
| loan_rt | 대출비율 | String | Y | 238 |  |
| mgge_mntn_rt | 담보유지비율 | String | Y | 238 |  |
| mgge_ensu_rt | 담보확보비율 | String | Y | 238 |  |
| loan_exec_psbl_yn | 대출실행가능여부 | String | Y | 1 |  |
| stff_name | 직원명 | String | Y | 60 |  |
| erlm_dt | 등록일자 | String | Y | 8 |  |
| tr_mket_name | 거래시장명 | String | Y | 60 |  |
| crcy_cd | 통화코드 | String | Y | 3 |  |
| natn_kor_name | 국가한글명 | String | Y | 60 |  |
| ovrs_excg_cd | 해외거래소코드 | String | Y | 4 |  |
| output2 | 응답상세 | Object | Y |  | array |
| loan_psbl_item_num | 대출가능종목수 | String | Y | 20 |  |
