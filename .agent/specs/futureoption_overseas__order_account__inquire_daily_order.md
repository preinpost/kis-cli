<!-- endpoint: /uapi/overseas-futureoption/v1/trading/inquire-daily-order -->
<!-- category: [해외선물옵션] 주문/계좌 -->
<!-- korean_name: 해외선물옵션 일별 주문내역 -->

# 해외선물옵션 일별 주문내역[해외선물-013]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-futureoption/v1/trading/inquire-daily-order
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: OTFM3120R
- **모의TRID**: 모의투자 미지원
- **Format**: JSON
- **Content-Type**: application/json; charset=utf-8

## 개요
해외선물옵션 일별 주문내역 API입니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용)※ 토큰 지정시 토큰 타입("Bearer") 지정 필요. 즉, 발급받은 접근토큰 앞에 앞에 "Bearer" 붙여서 호출EX) "Bearer eyJ..........8GA" |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | OTFM3120R |
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
| STRT_DT | 시작일자 | String | Y | 8 |  |
| END_DT | 종료일자 | String | Y | 8 |  |
| FM_PDGR_CD | FM상품군코드 | String | Y | 10 |  |
| CCLD_NCCS_DVSN | 체결미체결구분 | String | Y | 2 | 01:전체 / 02:체결 / 03:미체결 |
| SLL_BUY_DVSN_CD | 매도매수구분코드 | String | Y | 2 | %%전체 / 01 : 매도 / 02 : 매수 |
| FUOP_DVSN | 선물옵션구분 | String | Y | 2 | 00:전체 / 01:선물 / 02:옵션 |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | Y | 200 |  |
| CTX_AREA_NK200 | 연속조회키200 | String | Y | 200 |  |

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
| cano | 종합계좌번호 | String | Y | 8 |  |
| acnt_prdt_cd | 계좌상품코드 | String | Y | 2 |  |
| dt | 일자 | String | Y | 8 |  |
| ord_dt | 주문일자 | String | Y | 8 |  |
| odno | 주문번호 | String | Y | 8 | 접수한 주문의 일련번호(ex. 00360686)* 정정/취소시 문자열처럼 "0"을 포함해서 전송 (ex. ORGN_ODNO : 00360686)* 정정/취소시 문자열처럼 "0"을 포함해서 전송 (ex. ORGN_ODNO : 00360686) |
| orgn_ord_dt | 원주문일자 | String | Y | 8 |  |
| orgn_odno | 원주문번호 | String | Y | 8 | 원주문번호(ex. 00360685) |
| ovrs_futr_fx_pdno | 해외선물FX상품번호 | String | Y | 32 |  |
| rvse_cncl_dvsn_cd | 정정취소구분코드 | String | Y | 2 | 청산체결이 없는 신규 00청산체결이 없는 정정 01청산체결이 없는 취소 02청산체결이 있는 취소 02청산체결이 있는 신규 03청산체결이 있는 정정 04행사 05배정 06소멸 07만기 08 |
| sll_buy_dvsn_cd | 매도매수구분코드 | String | Y | 2 |  |
| cplx_ord_dvsn_cd | 복합주문구분코드 | String | Y | 1 |  |
| pric_dvsn_cd | 가격구분코드 | String | Y | 1 |  |
| rcit_dvsn_cd | 접수구분코드 | String | Y | 2 |  |
| fm_ord_qty | FM주문수량 | String | Y | 10 |  |
| fm_ord_pric | FM주문가격 | String | Y | 20 |  |
| fm_stop_ord_pric | FMSTOP주문가격 | String | Y | 20 |  |
| ecis_rsvn_ord_yn | 행사예약주문여부 | String | Y | 1 |  |
| fm_ccld_qty | FM체결수량 | String | Y | 10 |  |
| fm_ccld_pric | FM체결가격 | String | Y | 20 |  |
| fm_ord_rmn_qty | FM주문잔여수량 | String | Y | 10 |  |
| ord_grp_name | 주문그룹명 | String | Y | 60 |  |
| rcit_dtl_dtime | 접수상세일시 | String | Y | 17 |  |
| ccld_dtl_dtime | 체결상세일시 | String | Y | 17 |  |
| ordr_emp_no | 주문자사원번호 | String | Y | 6 |  |
| rjct_rson_name | 거부사유명 | String | Y | 60 |  |
| ccld_cndt_cd | 체결조건코드 | String | Y | 1 |  |
| trad_end_dt | 매매종료일자 | String | Y | 8 |  |
