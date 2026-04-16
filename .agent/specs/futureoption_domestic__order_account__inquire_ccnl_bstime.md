<!-- endpoint: /uapi/domestic-futureoption/v1/trading/inquire-ccnl-bstime -->
<!-- category: [국내선물옵션] 주문/계좌 -->
<!-- korean_name: 선물옵션 기준일체결내역 -->

# 선물옵션 기준일체결내역[v1_국내선물-016]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-futureoption/v1/trading/inquire-ccnl-bstime
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTFO5139R
- **모의TRID**: 모의투자 미지원

## 개요
선물옵션 기준일체결내역 API입니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTFO5139R |
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
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| ORD_DT | 주문일자 | String | Y | 8 | 주문일자(YYYYMMDD) |
| FUOP_TR_STRT_TMD | 선물옵션거래시작시각 | String | Y | 6 | 선물옵션거래시작시간(HHMMSS) |
| FUOP_TR_END_TMD | 선물옵션거래종료시각 | String | Y | 6 | 선물옵션거래종료시간(HHMMSS) |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | Y | 200 | 연속조회검색조건200 |
| CTX_AREA_NK200 | 연속조회키200 | String | Y | 200 | 연속조회키200 |

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
| output1 | 응답상세 | Array | Y |  | array |
| pdno | 상품번호 | String | Y | 12 |  |
| prdt_name | 상품명 | String | Y | 60 |  |
| odno | 주문번호 | String | Y | 10 |  |
| tr_type_name | 거래유형명 | String | Y | 60 |  |
| last_sttldt | 최종결제일 | String | Y | 8 |  |
| ccld_idx | 체결지수 | String | Y | 24 |  |
| ccld_qty | 체결량 | String | Y | 10 |  |
| trad_amt | 매매금액 | String | Y | 19 |  |
| fee | 수수료 | String | Y | 19 |  |
| ccld_btwn | 체결시간 | String | Y | 6 |  |
| output2 | 응답상세2 | Object | Y |  |  |
| tot_ccld_qty_smtl | 총체결수량합계 | String | Y | 19 |  |
| tot_ccld_amt_smtl | 총체결금액합계 | String | Y | 19 |  |
| fee_adjt | 수수료조정 | String | Y | 19 |  |
| fee_smtl | 수수료합계 | String | Y | 19 |  |
