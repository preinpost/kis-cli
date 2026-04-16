<!-- endpoint: /uapi/overseas-stock/v1/quotations/countries-holiday -->
<!-- category: [해외주식] 기본시세 -->
<!-- korean_name: 해외결제일자조회 -->

# 해외결제일자조회[해외주식-017]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-stock/v1/quotations/countries-holiday
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTOS5011R
- **모의TRID**: 모의투자 미지원

## 개요
해외결제일자조회 API입니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTOS5011R |
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
| TRAD_DT | 기준일자 | String | Y | 8 | 기준일자(YYYYMMDD) |
| CTX_AREA_NK | 연속조회키 | String | Y | 20 | 공백으로 입력 |
| CTX_AREA_FK | 연속조회검색조건 | String | Y | 20 | 공백으로 입력 |

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
| output | 응답상세1 | Object | Y |  |  |
| prdt_type_cd | 상품유형코드 | String | Y | 3 | 512 미국 나스닥 / 513 미국 뉴욕거래소 / 529 미국 아멕스 515 일본501 홍콩 / 543 홍콩CNY / 558 홍콩USD507 베트남 하노이거래소 / 508 베트남 호치민거래소551 중국 상해A / 552 중국 심천A |
| tr_natn_cd | 거래국가코드 | String | Y | 3 | 840 미국 / 392 일본 / 344 홍콩704 베트남 / 156 중국 |
| tr_natn_name | 거래국가명 | String | Y | 60 |  |
| natn_eng_abrv_cd | 국가영문약어코드 | String | Y | 2 | US 미국 / JP 일본 / HK 홍콩VN 베트남 / CN 중국 |
| tr_mket_cd | 거래시장코드 | String | Y | 2 |  |
| tr_mket_name | 거래시장명 | String | Y | 60 |  |
| acpl_sttl_dt | 현지결제일자 | String | Y | 8 | 현지결제일자(YYYYMMDD) |
| dmst_sttl_dt | 국내결제일자 | String | Y | 8 | 국내결제일자(YYYYMMDD) |
