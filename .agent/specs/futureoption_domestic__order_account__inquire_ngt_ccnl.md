<!-- endpoint: /uapi/domestic-futureoption/v1/trading/inquire-ngt-ccnl -->
<!-- category: [국내선물옵션] 주문/계좌 -->
<!-- korean_name: (야간)선물옵션 주문체결 내역조회 -->

# (야간)선물옵션 주문체결 내역조회 [국내선물-009]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-futureoption/v1/trading/inquire-ngt-ccnl
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: (구) JTCE5005R (신) STTN5201R
- **모의TRID**: 모의투자 미지원

## 개요
(야간)선물옵션 주문체결 내역조회 API입니다.
1. 야간 시장이 종료(06:00)된 이후 약 06:10경 야간시장의 주문체결내역이 주간으로 이관됩니다.
> 주간 API를 사용한다면 야간 장 중 주문체결내역을 실시간으로 조회할 수 없습니다.
> 주문체결내역의 이관이 완료되는 시점부터 주간 테이블에서 야간의 주문체결내역을 조회할 수 있습니다.
2. KRX야간시장의 경우 주문일자는 (T+1)일 입니다.
> 금요일의 경우 주문일자는 주말 및 공휴일을 제외하고 익 영업일인 월요일로 설정됩니다.
> 위 내용은 당사의 기준이 아닌 KRX 거래소의 기준으로 전 회원사 동일한 기준으로 주문체결이 이루어지고 있습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | (구) JTCE5005R (신) STTN5201R |
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
| STRT_ORD_DT | 시작주문일자 | String | Y | 8 |  |
| END_ORD_DT | 종료주문일자 | String | Y | 8 | 조회하려는 마지막 일자 다음일자로 조회(ex. 20221011 까지의 내역을 조회하고자 할 경우, 20221012로 종료주문일자 설정) |
| SLL_BUY_DVSN_CD | 매도매수구분코드 | String | Y | 2 | 공란 : default (00: 전체 ,01 : 매도, 02 : 매수) |
| CCLD_NCCS_DVSN | 체결미체결구분 | String | Y | 2 | 00 : 전체01 : 체결02 : 미체결 |
| SORT_SQN | 정렬순서 | String | Y | 2 | 공란 : default (DS : 정순, 그외 : 역순) |
| STRT_ODNO | 시작주문번호 | String | Y | 10 | 공란 : default |
| PDNO | 상품번호 | String | Y | 12 | 공란 : default |
| MKET_ID_CD | 시장ID코드 | String | Y | 3 | 공란 : default |
| FUOP_DVSN_CD | 선물옵션구분코드 | String | Y | 2 | 공란 : 전체, 01 : 선물, 02 : 옵션 |
| SCRN_DVSN | 화면구분 | String | Y | 2 | 02(Default) |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | Y | 200 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터) |
| CTX_AREA_NK200 | 연속조회키200 | String | Y | 200 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터) |

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
| output2 | 응답상세1 | Object | Y |  |  |
| tot_ord_qty | 총주문수량 | String | Y | 10 |  |
| tot_ccld_qty | 총체결수량 | String | Y | 10 |  |
| tot_ccld_qty_SMTL | 총체결수량 | String | Y | 19 | 신규 TR 사용 필드 |
| tot_ccld_amt | 총체결금액 | String | Y | 19 |  |
| tot_ccld_amt_SMTL | 총체결금액 | String | Y | 11 | 신규 TR 사용 필드 |
| fee | 수수료 | String | Y | 19 |  |
| ctac_tlno | 연락전화번호 | String | Y | 20 | 신규 TR 사용 필드 |
| output1 | 응답상세2 | Object Array | Y |  | 시간별체결 정보 |
| ord_gno_brno | 주문채번지점번호 | String | Y | 5 |  |
| cano | 종합계좌번호 | String | Y | 8 |  |
| csac_name | 종합계좌명 | String | Y | 60 |  |
| acnt_prdt_cd | 계좌상품코드 | String | Y | 2 |  |
| ord_dt | 주문일자 | String | Y | 8 |  |
| odno | 주문번호 | String | Y | 10 |  |
| orgn_odno | 원주문번호 | String | Y | 10 |  |
| sll_buy_dvsn_cd | 매도매수구분코드 | String | Y | 2 |  |
| trad_dvsn_name | 매매구분명 | String | Y | 60 |  |
| nmpr_type_name | 호가유형명 | String | Y | 60 |  |
| pdno | 상품번호 | String | Y | 12 |  |
| prdt_name | 상품명 | String | Y | 60 |  |
| prdt_type_cd | 상품유형코드 | String | Y | 3 |  |
| ord_qty | 주문수량 | String | Y | 10 |  |
| ord_idx4 | 주문지수 | String | Y | 20 | 신규 TR 사용 필드 |
| qty | 잔량 | String | Y | 10 |  |
| ord_tmd | 주문시각 | String | Y | 6 |  |
| tot_ccld_qty | 총체결수량 | String | Y | 10 |  |
| avg_idx | 평균지수 | String | Y | 19 |  |
| tot_ccld_amt | 총체결금액 | String | Y | 19 |  |
| rjct_qty | 거부수량 | String | Y | 10 |  |
| ingr_trad_rjct_rson_cd | 장내매매거부사유코드 | String | Y | 5 |  |
| ingr_trad_rjct_rson_name | 장내매매거부사유명 | String | Y | 60 |  |
| ord_stfno | 주문직원번호 | String | Y | 6 |  |
| sprd_item_yn | 스프레드종목여부 | String | Y | 1 |  |
| ord_ip_addr | 주문IP주소 | String | Y | 200 |  |
