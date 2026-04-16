<!-- endpoint: /uapi/domestic-stock/v1/trading/inquire-balance -->
<!-- category: [국내주식] 주문/계좌 -->
<!-- korean_name: 주식잔고조회 -->

# 주식잔고조회[v1_국내주식-006]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/trading/inquire-balance
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: TTTC8434R
- **모의TRID**: VTTC8434R
- **Format**: JSON
- **Content-Type**: application/json; charset=utf-8

## 개요
주식 잔고조회 API입니다.
실전계좌의 경우, 한 번의 호출에 최대 50건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
모의계좌의 경우, 한 번의 호출에 최대 20건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
* 당일 전량매도한 잔고도 보유수량 0으로 보여질 수 있으나, 해당 보유수량 0인 잔고는 최종 D-2일 이후에는 잔고에서 사라집니다.
※ 중요 : 해당 API는 제공 정보량이 많아 조회속도가 느린 API입니다. 주문 준비를 위해서는 주식매수/매도가능수량 조회 TR 사용을 권장 드립니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용)법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자]TTTC8434R : 주식 잔고 조회[모의투자]VTTC8434R : 주식 잔고 조회 |
| tr_cont | 연속 거래 여부 | String | N | 1 | 공백 : 초기 조회N : 다음 데이터 조회 (output header의 tr_cont가 M일 경우) |
| custtype | 고객타입 | String | N | 1 | B : 법인P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| AFHR_FLPR_YN | 시간외단일가, 거래소여부 | String | Y | 1 | N : 기본값,Y : 시간외단일가,X : NXT 정규장 (프리마켓, 메인, 애프터마켓)※ NXT 선택 시 : NXT 거래종목만 시세 등 정보가 NXT 기준으로 변동됩니다. KRX 종목들은 그대로 유지 |
| OFL_YN | 오프라인여부 | String | N | 1 | 공란(Default) |
| INQR_DVSN | 조회구분 | String | Y | 2 | 01 : 대출일별 |
| UNPR_DVSN | 단가구분 | String | Y | 2 | 01 : 기본값 |
| FUND_STTL_ICLD_YN | 펀드결제분포함여부 | String | Y | 1 | N : 포함하지 않음Y : 포함 |
| FNCG_AMT_AUTO_RDPT_YN | 융자금액자동상환여부 | String | Y | 1 | N : 기본값 |
| PRCS_DVSN | 처리구분 | String | Y | 2 | 00 : 전일매매포함01 : 전일매매미포함 |
| CTX_AREA_FK100 | 연속조회검색조건100 | String | N | 100 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_FK100 값 : 다음페이지 조회시(2번째부터) |
| CTX_AREA_NK100 | 연속조회키100 | String | N | 100 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_NK100 값 : 다음페이지 조회시(2번째부터) |

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
| ctx_area_fk100 | 연속조회검색조건100 | String | Y | 100 |  |
| ctx_area_nk100 | 연속조회키100 | String | Y | 100 |  |
| output1 | 응답상세1 | Object Array | Y | - | Array |
| pdno | 상품번호 | String | Y | 12 | 종목번호(뒷 6자리) |
| prdt_name | 상품명 | String | Y | 60 | 종목명 |
| trad_dvsn_name | 매매구분명 | String | Y | 60 | 매수매도구분 |
| bfdy_buy_qty | 전일매수수량 | String | Y | 10 |  |
| bfdy_sll_qty | 전일매도수량 | String | Y | 10 |  |
| thdt_buyqty | 금일매수수량 | String | Y | 10 |  |
| thdt_sll_qty | 금일매도수량 | String | Y | 10 |  |
| hldg_qty | 보유수량 | String | Y | 19 |  |
| ord_psbl_qty | 주문가능수량 | String | Y | 10 |  |
| pchs_avg_pric | 매입평균가격 | String | Y | 22 | 매입금액 / 보유수량 |
| pchs_amt | 매입금액 | String | Y | 19 |  |
| prpr | 현재가 | String | Y | 19 |  |
| evlu_amt | 평가금액 | String | Y | 19 |  |
| evlu_pfls_amt | 평가손익금액 | String | Y | 19 | 평가금액 - 매입금액 |
| evlu_pfls_rt | 평가손익율 | String | Y | 9 |  |
| evlu_erng_rt | 평가수익율 | String | Y | 31 | 미사용항목(0으로 출력) |
| loan_dt | 대출일자 | String | Y | 8 | INQR_DVSN(조회구분)을 01(대출일별)로 설정해야 값이 나옴 |
| loan_amt | 대출금액 | String | Y | 19 |  |
| stln_slng_chgs | 대주매각대금 | String | Y | 19 |  |
| expd_dt | 만기일자 | String | Y | 8 |  |
| fltt_rt | 등락율 | String | Y | 31 |  |
| bfdy_cprs_icdc | 전일대비증감 | String | Y | 19 |  |
| item_mgna_rt_name | 종목증거금율명 | String | Y | 20 |  |
| grta_rt_name | 보증금율명 | String | Y | 20 |  |
| sbst_pric | 대용가격 | String | Y | 19 | 증권매매의 위탁보증금으로서 현금 대신에 사용되는 유가증권 가격 |
| stck_loan_unpr | 주식대출단가 | String | Y | 22 |  |
| output2 | 응답상세2 | Object Array | Y | - | Array |
| dnca_tot_amt | 예수금총금액 | String | Y | 19 | 예수금 |
| nxdy_excc_amt | 익일정산금액 | String | Y | 19 | D+1 예수금 |
| prvs_rcdl_excc_amt | 가수도정산금액 | String | Y | 19 | D+2 예수금 |
| cma_evlu_amt | CMA평가금액 | String | Y | 19 |  |
| bfdy_buy_amt | 전일매수금액 | String | Y | 19 |  |
| thdt_buy_amt | 금일매수금액 | String | Y | 19 |  |
| nxdy_auto_rdpt_amt | 익일자동상환금액 | String | Y | 19 |  |
| bfdy_sll_amt | 전일매도금액 | String | Y | 19 |  |
| thdt_sll_amt | 금일매도금액 | String | Y | 19 |  |
| d2_auto_rdpt_amt | D+2자동상환금액 | String | Y | 19 |  |
| bfdy_tlex_amt | 전일제비용금액 | String | Y | 19 |  |
| thdt_tlex_amt | 금일제비용금액 | String | Y | 19 |  |
| tot_loan_amt | 총대출금액 | String | Y | 19 |  |
| scts_evlu_amt | 유가평가금액 | String | Y | 19 |  |
| tot_evlu_amt | 총평가금액 | String | Y | 19 | 유가증권 평가금액 합계금액 + D+2 예수금 |
| nass_amt | 순자산금액 | String | Y | 19 |  |
| fncg_gld_auto_rdpt_yn | 융자금자동상환여부 | String | Y | 1 | 보유현금에 대한 융자금만 차감여부신용융자 매수체결 시점에서는 융자비율을 매매대금 100%로 계산 하였다가 수도결제일에 보증금에 해당하는 금액을 고객의 현금으로 충당하여 융자금을 감소시키는 업무 |
| pchs_amt_smtl_amt | 매입금액합계금액 | String | Y | 19 |  |
| evlu_amt_smtl_amt | 평가금액합계금액 | String | Y | 19 | 유가증권 평가금액 합계금액 |
| evlu_pfls_smtl_amt | 평가손익합계금액 | String | Y | 19 |  |
| tot_stln_slng_chgs | 총대주매각대금 | String | Y | 19 |  |
| bfdy_tot_asst_evlu_amt | 전일총자산평가금액 | String | Y | 19 |  |
| asst_icdc_amt | 자산증감액 | String | Y | 19 |  |
| asst_icdc_erng_rt | 자산증감수익율 | String | Y | 31 | 데이터 미제공 |
