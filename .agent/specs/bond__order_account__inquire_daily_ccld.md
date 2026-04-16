<!-- endpoint: /uapi/domestic-bond/v1/trading/inquire-daily-ccld -->
<!-- category: [장내채권] 주문/계좌 -->
<!-- korean_name: 장내채권 주문체결내역 -->

# 장내채권 주문체결내역 [국내주식-127]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-bond/v1/trading/inquire-daily-ccld
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTSC8013R
- **모의TRID**: 모의투자 미지원

## 개요
장내채권 주문체결내역 API입니다.
한국투자 HTS(eFriend Plus) > [0978] 장내채권주문 '채권주문체결' 탭의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTSC8013R |
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
| CANO | 종합계좌번호 | String | Y | 8 | 종합계좌번호 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌상품코드 |
| INQR_STRT_DT | 조회시작일자 | String | Y | 8 | 일자 ~ (1주일 이내) |
| INQR_END_DT | 조회종료일자 | String | Y | 8 | ~ 일자 (조회 당일) |
| SLL_BUY_DVSN_CD | 매도매수구분코드 | String | Y | 2 | %(전체), 01(매도), 02(매수) |
| SORT_SQN_DVSN | 정렬순서구분 | String | Y | 2 | 01(주문순서), 02(주문역순) |
| PDNO | 상품번호 | String | Y | 12 |  |
| NCCS_YN | 미체결여부 | String | Y | 1 | N(전체), C(체결), Y(미체결) |
| CTX_AREA_NK200 | 연속조회키200 | String | Y | 200 |  |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | Y | 200 |  |

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
| output1 | 응답상세 | Object Array | Y | - |  |
| tot_ord_qty | 총주문수량 | String | Y | 10 |  |
| tot_ccld_qty_smtl | 총체결수량합계 | String | Y | 19 |  |
| tot_bond_ccld_avg_unpr | 총채권체결평균단가 | String | Y | 182 |  |
| tot_ccld_amt_smtl | 총체결금액합계 | String | Y | 19 |  |
| output2 | 응답상세 | Object | Y |  | array |
| ord_dt | 주문일자 | String | Y | 8 |  |
| odno | 주문번호 | String | Y | 10 |  |
| orgn_odno | 원주문번호 | String | Y | 10 |  |
| ord_dvsn_name | 주문구분명 | String | Y | 60 |  |
| sll_buy_dvsn_cd_name | 매도매수구분코드명 | String | Y | 60 |  |
| shtn_pdno | 단축상품번호 | String | Y | 12 |  |
| prdt_abrv_name | 상품약어명 | String | Y | 60 |  |
| ord_qty | 주문수량 | String | Y | 10 |  |
| bond_ord_unpr | 채권주문단가 | String | Y | 182 |  |
| ord_tmd | 주문시각 | String | Y | 6 |  |
| tot_ccld_qty | 총체결수량 | String | Y | 10 |  |
| bond_avg_unpr | 채권평균단가 | String | Y | 182 |  |
| tot_ccld_amt | 총체결금액 | String | Y | 19 |  |
| loan_dt | 대출일자 | String | Y | 8 |  |
| buy_dt | 매수일자 | String | Y | 8 |  |
| samt_mket_ptci_yn_name | 소액시장참여여부명 | String | Y | 10 |  |
| sprx_psbl_yn_ifom | 분리과세가능여부알림 | String | Y | 60 |  |
| ord_mdia_dvsn_name | 주문매체구분명 | String | Y | 60 |  |
| sll_buy_dvsn_cd | 매도매수구분코드 | String | Y | 2 |  |
| nccs_qty | 미체결수량 | String | Y | 10 |  |
| ord_gno_brno | 주문채번지점번호 | String | Y | 5 |  |
