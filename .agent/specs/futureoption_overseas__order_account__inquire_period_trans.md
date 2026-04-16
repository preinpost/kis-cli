<!-- endpoint: /uapi/overseas-futureoption/v1/trading/inquire-period-trans -->
<!-- category: [해외선물옵션] 주문/계좌 -->
<!-- korean_name: 해외선물옵션 기간계좌거래내역 -->

# 해외선물옵션 기간계좌거래내역[해외선물-014]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-futureoption/v1/trading/inquire-period-trans
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: OTFM3114R
- **모의TRID**: 모의투자 미지원
- **Format**: JSON
- **Content-Type**: application/json; charset=utf-8

## 개요
해외선물옵션 기간계좌거래내역 API입니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용)※ 토큰 지정시 토큰 타입("Bearer") 지정 필요. 즉, 발급받은 접근토큰 앞에 앞에 "Bearer" 붙여서 호출EX) "Bearer eyJ..........8GA" |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | OTFM3114R |
| tr_cont | 연속 거래 여부 | String | N | 1 | 공백 : 초기 조회N : 다음 데이터 조회 (output header의 tr_cont가 M일 경우) |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| INQR_TERM_FROM_DT | 조회기간FROM일자 | String | Y | 8 |  |
| INQR_TERM_TO_DT | 조회기간TO일자 | String | Y | 8 |  |
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| ACNT_TR_TYPE_CD | 계좌거래유형코드 | String | Y | 2 | 1: 전체, 2:입출금 , 3: 결제 |
| CRCY_CD | 통화코드 | String | Y | 3 | '%%% : 전체TUS: TOT_USD / TKR: TOT_KRWKRW: 한국 / USD: 미국EUR: EUR / HKD: 홍콩CNY: 중국 / JPY: 일본VND: 베트남 ' |
| CTX_AREA_FK100 | 연속조회검색조건100 | String | Y | 100 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_FK100값 : 다음페이지 조회시(2번째부터) |
| CTX_AREA_NK100 | 연속조회키100 | String | Y | 100 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_NK100값 : 다음페이지 조회시(2번째부터) |
| PWD_CHK_YN | 비밀번호체크여부 | String | Y | 1 | 공란(Default) |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | N | 1 | F or M : 다음 데이터 있음D or E : 마지막 데이터 |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 |  |
| msg_cd | 응답코드 | String | Y | 8 |  |
| msg1 | 응답메세지 | String | Y | 80 |  |
| output | 응답상세1 | Object Array | Y |  | Array |
| bass_dt | 기준일자 | String | Y | 8 |  |
| cano | 종합계좌번호 | String | Y | 8 |  |
| acnt_prdt_cd | 계좌상품코드 | String | Y | 2 |  |
| fm_ldgr_inog_seq | FM원장출납순번 | String | Y | 10 |  |
| acnt_tr_type_name | 계좌거래유형명 | String | Y | 60 |  |
| crcy_cd | 통화코드 | String | Y | 3 |  |
| tr_itm_name | 거래항목명 | String | Y | 60 |  |
| fm_iofw_amt | FM입출금액 | String | Y | 20 |  |
| fm_fee | FM수수료 | String | Y | 20 |  |
| fm_tax_amt | FM세금금액 | String | Y | 20 |  |
| fm_sttl_amt | FM결제금액 | String | Y | 20 |  |
| fm_bf_dncl_amt | FM이전예수금액 | String | Y | 20 |  |
| fm_dncl_amt | FM예수금액 | String | Y | 20 |  |
| fm_rcvb_occr_amt | FM미수발생금액 | String | Y | 20 |  |
| fm_rcvb_pybk_amt | FM미수변제금액 | String | Y | 20 |  |
| ovdu_int_pybk_amt | 연체이자변제금액 | String | Y | 20 |  |
| rmks_text | 비고내용 | String | Y | 500 |  |
