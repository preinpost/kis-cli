<!-- endpoint: /uapi/domestic-futureoption/v1/trading/inquire-balance-settlement-pl -->
<!-- category: [국내선물옵션] 주문/계좌 -->
<!-- korean_name: 선물옵션 잔고정산손익내역 -->

# 선물옵션 잔고정산손익내역[v1_국내선물-013]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-futureoption/v1/trading/inquire-balance-settlement-pl
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTFO6117R
- **모의TRID**: 모의투자 미지원

## 개요
선물옵션 잔고정산손익내역 API입니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTFO6117R |
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
| INQR_DT | 조회일자 | String | Y | 8 | 조회일자(YYYYMMDD) |
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
| output2 | 응답상세 | Object | Y |  |  |
| nxdy_dnca | 익일예수금 | String | Y | 19 |  |
| mmga_cash | 유지증거금현금 | String | Y | 19 |  |
| brkg_mgna_cash | 위탁증거금현금 | String | Y | 19 |  |
| opt_buy_chgs | 옵션매수대금 | String | Y | 19 |  |
| opt_lqd_evlu_amt | 옵션청산평가금액 | String | Y | 19 |  |
| dnca_sbst | 예수금대용 | String | Y | 19 |  |
| mmga_tota | 유지증거금총액 | String | Y | 19 |  |
| brkg_mgna_tota | 위탁증거금총액 | String | Y | 19 |  |
| opt_sll_chgs | 옵션매도대금 | String | Y | 19 |  |
| fee | 수수료 | String | Y | 19 |  |
| thdt_dfpa | 당일차금 | String | Y | 19 |  |
| rnwl_dfpa | 갱신차금 | String | Y | 19 |  |
| dnca_cash | 예수금현금 | String | Y | 19 |  |
| output1 | 응답상세2 | Array | Y |  | array |
| pdno | 상품번호 | String | Y | 12 |  |
| prdt_name | 상품명 | String | Y | 60 |  |
| trad_dvsn_name | 매매구분명 | String | Y | 60 |  |
| bfdy_cblc_qty | 전일잔고수량 | String | Y | 19 |  |
| new_qty | 신규수량 | String | Y | 10 |  |
| mnpl_rpch_qty | 전매환매수량 | String | Y | 10 |  |
| cblc_qty | 잔고수량 | String | Y | 19 |  |
| cblc_amt | 잔고금액 | String | Y | 19 |  |
| trad_pfls_amt | 매매손익금액 | String | Y | 19 |  |
| evlu_amt | 평가금액 | String | Y | 19 |  |
| evlu_pfls_amt | 평가손익금액 | String | Y | 19 |  |
