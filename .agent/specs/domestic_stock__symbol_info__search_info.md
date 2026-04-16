<!-- endpoint: /uapi/domestic-stock/v1/quotations/search-info -->
<!-- category: [국내주식] 종목정보 -->
<!-- korean_name: 상품기본조회 -->

# 상품기본조회[v1_국내주식-029]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/search-info
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTPF1604R
- **모의TRID**: 모의투자 미지원

## 개요
요청

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTPF1604R |
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
| PDNO | 상품번호 | String | Y | 12 | '주식(하이닉스) : 000660 (코드 : 300)선물(101S12) : KR4101SC0009 (코드 : 301)미국(AAPL) : AAPL (코드 : 512)' |
| PRDT_TYPE_CD | 상품유형코드 | String | Y | 3 | '300 주식301 선물옵션302 채권512 미국 나스닥 / 513 미국 뉴욕 / 529 미국 아멕스 515 일본501 홍콩 / 543 홍콩CNY / 558 홍콩USD507 베트남 하노이 / 508 베트남 호치민551 중국 상해A / 552 중국 심천A' |

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
| pdno | 상품번호 | String | Y | 12 |  |
| prdt_type_cd | 상품유형코드 | String | Y | 3 |  |
| prdt_name | 상품명 | String | Y | 60 |  |
| prdt_name120 | 상품명120 | String | Y | 120 |  |
| prdt_abrv_name | 상품약어명 | String | Y | 60 |  |
| prdt_eng_name | 상품영문명 | String | Y | 60 |  |
| prdt_eng_name120 | 상품영문명120 | String | Y | 120 |  |
| prdt_eng_abrv_name | 상품영문약어명 | String | Y | 60 |  |
| std_pdno | 표준상품번호 | String | Y | 12 |  |
| shtn_pdno | 단축상품번호 | String | Y | 12 |  |
| prdt_sale_stat_cd | 상품판매상태코드 | String | Y | 2 |  |
| prdt_risk_grad_cd | 상품위험등급코드 | String | Y | 2 |  |
| prdt_clsf_cd | 상품분류코드 | String | Y | 6 |  |
| prdt_clsf_name | 상품분류명 | String | Y | 60 |  |
| sale_strt_dt | 판매시작일자 | String | Y | 8 |  |
| sale_end_dt | 판매종료일자 | String | Y | 8 |  |
| wrap_asst_type_cd | 랩어카운트자산유형코드 | String | Y | 2 |  |
| ivst_prdt_type_cd | 투자상품유형코드 | String | Y | 4 |  |
| ivst_prdt_type_cd_name | 투자상품유형코드명 | String | Y | 60 |  |
| frst_erlm_dt | 최초등록일자 | String | Y | 8 |  |
