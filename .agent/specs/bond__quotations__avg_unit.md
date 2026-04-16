<!-- endpoint: /uapi/domestic-bond/v1/quotations/avg-unit -->
<!-- category: [장내채권] 기본시세 -->
<!-- korean_name: 장내채권 평균단가조회 -->

# 장내채권 평균단가조회 [국내주식-158]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-bond/v1/quotations/avg-unit
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTPF2005R
- **모의TRID**: 모의투자 미지원

## 개요
장내채권 평균단가조회 API입니다.
한국투자 HTS(eFriend Plus) > [7216] 채권 발행정보 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTPF2005R |
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
| INQR_STRT_DT | 조회시작일자 | String | Y | 8 | 일자 ~ |
| INQR_END_DT | 조회종료일자 | String | Y | 8 | ~ 일자 |
| PDNO | 상품번호 | String | Y | 12 | 공백: 전체, 특정종목 조회시 : 종목코드 |
| PRDT_TYPE_CD | 상품유형코드 | String | Y | 3 | Unique key(302) |
| VRFC_KIND_CD | 검증종류코드 | String | Y | 2 | Unique key(00) |
| CTX_AREA_NK30 | 연속조회키30 | String | Y | 30 | 공백 |
| CTX_AREA_FK100 | 연속조회검색조건100 | String | Y | 100 | 공백 |

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
| evlu_dt | 평가일자 | String | Y | 245 |  |
| pdno | 상품번호 | String | Y | 202 |  |
| prdt_type_cd | 상품유형코드 | String | Y | 238 |  |
| prdt_name | 상품명 | String | Y | 1 |  |
| kis_unpr | 한국신용평가단가 | String | Y | 8 |  |
| kbp_unpr | 한국채권평가단가 | String | Y | 500 |  |
| nice_evlu_unpr | 한국신용정보평가단가 | String | Y | 238 |  |
| fnp_unpr | 에프앤자산평가단가 | String | Y | 202 |  |
| avg_evlu_unpr | 평균평가단가 | String | Y | 500 |  |
| kis_crdt_grad_text | 한국신용평가신용등급내용 | String | Y | 238 |  |
| kbp_crdt_grad_text | 한국채권평가신용등급내용 | String | Y | 202 |  |
| nice_crdt_grad_text | 한국신용정보신용등급내용 | String | Y | 238 |  |
| fnp_crdt_grad_text | 에프앤자산평가신용등급내용 | String | Y | 500 |  |
| chng_yn | 변경여부 | String | Y | 238 |  |
| kis_erng_rt | 한국신용평가수익율 | String | Y | 202 |  |
| kbp_erng_rt | 한국채권평가수익율 | String | Y | 238 |  |
| nice_evlu_erng_rt | 한국신용정보평가수익율 | String | Y | 500 |  |
| fnp_erng_rt | 에프앤자산평가수익율 | String | Y | 179 |  |
| avg_evlu_erng_rt | 평균평가수익율 | String | Y | 202 |  |
| kis_rf_unpr | 한국신용평가RF단가 | String | Y | 238 |  |
| kbp_rf_unpr | 한국채권평가RF단가 | String | Y | 12 |  |
| nice_evlu_rf_unpr | 한국신용정보평가RF단가 | String | Y | 60 |  |
| avg_evlu_rf_unpr | 평균평가RF단가 | String | Y | 3 |  |
| output2 | 응답상세 | Object Array | Y |  | array |
| evlu_dt | 평가일자 | String | Y | 19 |  |
| pdno | 상품번호 | String | Y | 1 |  |
| prdt_type_cd | 상품유형코드 | String | Y | 8 |  |
| prdt_name | 상품명 | String | Y | 19 |  |
| kis_evlu_amt | 한국신용평가평가금액 | String | Y | 19 |  |
| kbp_evlu_amt | 한국채권평가평가금액 | String | Y | 19 |  |
| nice_evlu_amt | 한국신용정보평가금액 | String | Y | 19 |  |
| fnp_evlu_amt | 에프앤자산평가평가금액 | String | Y | 12 |  |
| avg_evlu_amt | 평균평가금액 | String | Y | 60 |  |
| chng_yn | 변경여부 | String | Y | 3 |  |
| output3 | 응답상세 | Object Array | Y |  | array |
| evlu_dt | 평가일자 | String | Y | 236 |  |
| pdno | 상품번호 | String | Y | 19 |  |
| prdt_type_cd | 상품유형코드 | String | Y | 1 |  |
| prdt_name | 상품명 | String | Y | 8 |  |
| kis_crcy_cd | 한국신용평가통화코드 | String | Y | 3 |  |
| kis_evlu_unit_pric | 한국신용평가평가단위가격 | String | Y | 236 |  |
| kis_evlu_pric | 한국신용평가평가가격 | String | Y | 19 |  |
| kbp_crcy_cd | 한국채권평가통화코드 | String | Y | 3 |  |
| kbp_evlu_unit_pric | 한국채권평가평가단위가격 | String | Y | 236 |  |
| kbp_evlu_pric | 한국채권평가평가가격 | String | Y | 19 |  |
| nice_crcy_cd | 한국신용정보통화코드 | String | Y | 3 |  |
| nice_evlu_unit_pric | 한국신용정보평가단위가격 | String | Y | 236 |  |
| nice_evlu_pric | 한국신용정보평가가격 | String | Y | 19 |  |
| avg_evlu_unit_pric | 평균평가단위가격 | String | Y | 12 |  |
| avg_evlu_pric | 평균평가가격 | String | Y | 60 |  |
| chng_yn | 변경여부 | String | Y | 3 |  |
