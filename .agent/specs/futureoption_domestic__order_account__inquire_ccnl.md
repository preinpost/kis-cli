<!-- endpoint: /uapi/domestic-futureoption/v1/trading/inquire-ccnl -->
<!-- category: [국내선물옵션] 주문/계좌 -->
<!-- korean_name: 선물옵션 주문체결내역조회 -->

# 선물옵션 주문체결내역조회[v1_국내선물-003]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-futureoption/v1/trading/inquire-ccnl
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: TTTO5201R
- **모의TRID**: VTTO5201R
- **Format**: JSON
- **Content-Type**: application/json; charset=utf-8

## 개요
선물옵션 주문체결내역조회 API입니다. 한 번의 호출에 최대 100건​까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자] TTTO5201R : 선물 옵션 주문 체결 내역 조회[모의투자] VTTO5201R : 선물 옵션 주문 체결 내역 조회 |
| tr_cont | 연속 거래 여부 | String | N | 1 | 공백 : 초기 조회N : 다음 데이터 조회 (output header의 tr_cont가 M일 경우) |
| custtype | 고객타입 | String | N | 1 | B : 법인P : 개인 |
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
| STRT_ORD_DT | 시작주문일자 | String | Y | 8 | 주문내역 조회 시작 일자, YYYYMMDD |
| END_ORD_DT | 종료주문일자 | String | Y | 8 | 주문내역 조회 마지막 일자, YYYYMMDD |
| SLL_BUY_DVSN_CD | 매도매수구분코드 | String | Y | 2 | 00 : 전체01 : 매도02 : 매수 |
| CCLD_NCCS_DVSN | 체결미체결구분 | String | Y | 2 | 00 : 전체01 : 체결02 : 미체결 |
| SORT_SQN | 정렬순서 | String | Y | 2 | AS : 정순DS : 역순 |
| STRT_ODNO | 시작주문번호 | String | Y | 10 | 조회 시작 번호 입력 |
| PDNO | 상품번호 | String | Y | 12 | 공란 시, 전체 조회선물 6자리 (예: 101S03)옵션 9자리 (예: 201S03370) |
| MKET_ID_CD | 시장ID코드 | String | Y | 3 | 공란(Default) |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | Y | 200 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터) |
| CTX_AREA_NK200 | 연속조회키200 | String | Y | 200 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터) |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | Y | 1 | F or M : 다음 데이터 있음D or E : 마지막 데이터 |
| gt_uid | Global UID | String | Y | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 | 응답코드 |
| msg1 | 응답메세지 | String | Y | 80 | 응답메세지 |
| ctx_area_fk200 | 연속조회검색조건200 | String | Y | 200 |  |
| ctx_area_nk200 | 연속조회키200 | String | Y | 200 |  |
| output1 | 응답상세1 | Array | Y | - |  |
| ord_gno_brno | 주문채번지점번호 | String | Y | 5 | 계좌 개설 시 관리점으로 선택한 영업점의 고유번호 |
| cano | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| csac_name | 종합계좌명 | String | Y | 60 | 계좌의 고객명 |
| acnt_prdt_cd | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| ord_dt | 주문일자 | String | Y | 8 | 주문의 접수일자 |
| odno | 주문번호 | String | Y | 10 | 접수한 주문의 일련번호 |
| orgn_odno | 원주문번호 | String | Y | 10 | 정정 또는 취소 대상 주문의 일련번호 |
| sll_buy_dvsn_cd | 매도매수구분코드 | String | Y | 2 | 00 : 전체 01 : 매도 02 : 매수 |
| trad_dvsn_name | 매매구분명 | String | Y | 60 | 매도/매수 등 구분값 |
| nmpr_type_cd | 호가유형코드 | String | Y | 2 | 01 : 지정가02 : 시장가03 : 조건부04 : 최유리 |
| nmpr_type_name | 호가유형명 | String | Y | 60 | 호가 유형의 명칭 |
| pdno | 상품번호 | String | Y | 12 | 선물옵션종목코드 |
| prdt_name | 상품명 | String | Y | 60 |  |
| prdt_type_cd | 상품유형코드 | String | Y | 3 |  |
| ord_qty | 주문수량 | String | Y | 10 | 주문 수량 |
| ord_idx | 주문지수 | String | Y | 24 | 주문 가격 |
| qty | 잔량 | String | Y | 10 | 주문 체결되지 않고 남은 수량 |
| ord_tmd | 주문시각 | String | Y | 6 | 주문 접수 시간 |
| tot_ccld_qty | 총체결수량 | String | Y | 10 | 주문 체결된 수량 |
| avg_idx | 평균지수 | String | Y | 27 | 체결된 주문 수량의 평균 체결 가격 |
| tot_ccld_amt | 총체결금액 | String | Y | 19 | 체결된 주문의 합계금액 |
| rjct_qty | 거부수량 | String | Y | 10 | 접수된 주문이 정상 처리되지 못하고 거부된 수량 |
| ingr_trad_rjct_rson_cd | 장내매매거부사유코드 | String | Y | 5 | 정상 처리되지 못하고 거부된 주문의 사유코드 |
| ingr_trad_rjct_rson_name | 장내매매거부사유명 | String | Y | 60 | 정상 처리되지 못하고 거부된 주문의 사유 |
| ord_stfno | 주문직원번호 | String | Y | 6 | 주문 접수한 직원의 사번 또는 온라인 주문 시 매체 유형코드 |
| sprd_item_yn | 스프레드종목여부 | String | Y | 1 | 스프레드 종목 여부 구분값 |
| ord_ip_addr | 주문IP주소 | String | Y | 200 | 주문 시 사용한 매체의 IP 주소 |
| output2 | 응답상세2 | Object | Y | - |  |
| tot_ord_qty | 총주문수량 | String | Y | 10 | 전체 주문 수량 |
| tot_ccld_amt_smtl | 총체결금액합계 | String | Y | 19 | 체결된 주문 전체의 합계 금액 |
| tot_ccld_qty_smtl | 총체결수량합계 | String | Y | 19 | 체결된 주문 전체의 합계 수량 |
| fee_smtl | 수수료합계 | String | Y | 19 | 체결된 주문에 대한 매매수수료의 합계 금액 |
| ctac_tlno | 연락전화번호 | String | Y | 20 | 고객의 연락 가능한 전화번호 |
