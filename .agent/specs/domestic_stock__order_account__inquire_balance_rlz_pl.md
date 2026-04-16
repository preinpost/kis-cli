<!-- endpoint: /uapi/domestic-stock/v1/trading/inquire-balance-rlz-pl -->
<!-- category: [국내주식] 주문/계좌 -->
<!-- korean_name: 주식잔고조회_실현손익 -->

# 주식잔고조회_실현손익[v1_국내주식-041]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/trading/inquire-balance-rlz-pl
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: TTTC8494R
- **모의TRID**: 모의투자 미지원

## 개요
주식잔고조회_실현손익 API입니다.
한국투자 HTS(eFriend Plus) [0800] 국내 체결기준잔고 화면을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
(참고: 포럼 - 공지사항 - 신규 API 추가 안내(주식잔고조회_실현손익 외 1건))

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | TTTC8494R |
| tr_cont | 연속 거래 여부 | String | N | 1 | F or M : 다음 데이터 있음D or E : 마지막 데이터 |
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
| AFHR_FLPR_YN | 시간외단일가여부 | String | Y | 1 | 'N : 기본값 Y : 시간외단일가' |
| OFL_YN | 오프라인여부 | String | Y | 1 | 공란 |
| INQR_DVSN | 조회구분 | String | Y | 2 | 00 : 전체 |
| UNPR_DVSN | 단가구분 | String | Y | 2 | 01 : 기본값 |
| FUND_STTL_ICLD_YN | 펀드결제포함여부 | String | Y | 1 | N : 포함하지 않음 Y : 포함 |
| FNCG_AMT_AUTO_RDPT_YN | 융자금액자동상환여부 | String | Y | 1 | N : 기본값 |
| PRCS_DVSN | PRCS_DVSN | String | Y | 2 | 00 : 전일매매포함 01 : 전일매매미포함 |
| COST_ICLD_YN | 비용포함여부 | String | Y | 1 |  |
| CTX_AREA_FK100 | 연속조회검색조건100 | String | Y | 100 | 공란 : 최초 조회시 이전 조회 Output CTX_AREA_FK100 값 : 다음페이지 조회시(2번째부터) |
| CTX_AREA_NK100 | 연속조회키100 | String | Y | 100 | 공란 : 최초 조회시 이전 조회 Output CTX_AREA_NK100 값 : 다음페이지 조회시(2번째부터) |

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
| output1 | 응답상세 | Object Array | Y |  | Array |
| pdno | 상품번호 | String | Y | 12 | 종목번호(뒷 6자리) |
| prdt_name | 상품명 | String | Y | 60 | 종목명 |
| trad_dvsn_name | 매매구분명 | String | Y | 60 | 매수매도구분 |
| bfdy_buy_qty | 전일매수수량 | String | Y | 10 |  |
| bfdy_sll_qty | 전일매도수량 | String | Y | 10 |  |
| thdt_buyqty | 금일매수수량 | String | Y | 10 |  |
| thdt_sll_qty | 금일매도수량 | String | Y | 10 |  |
| hldg_qty | 보유수량 | String | Y | 19 |  |
| ord_psbl_qty | 주문가능수량 | String | Y | 10 |  |
| pchs_avg_pric | 매입평균가격 | String | Y | 23 | 매입금액 / 보유수량 |
| pchs_amt | 매입금액 | String | Y | 19 |  |
| prpr | 현재가 | String | Y | 19 |  |
| evlu_amt | 평가금액 | String | Y | 19 |  |
| evlu_pfls_amt | 평가손익금액 | String | Y | 19 | 평가금액 - 매입금액 |
| evlu_pfls_rt | 평가손익율 | String | Y | 10 |  |
| evlu_erng_rt | 평가수익율 | String | Y | 32 |  |
| loan_dt | 대출일자 | String | Y | 8 |  |
| loan_amt | 대출금액 | String | Y | 19 |  |
| stln_slng_chgs | 대주매각대금 | String | Y | 19 | 신용 거래에서, 고객이 증권 회사로부터 대부받은 주식의 매각 대금 |
| expd_dt | 만기일자 | String | Y | 8 |  |
| stck_loan_unpr | 주식대출단가 | String | Y | 23 |  |
| bfdy_cprs_icdc | 전일대비증감 | String | Y | 19 |  |
| fltt_rt | 등락율 | String | Y | 32 |  |
| output2 | 응답상세2 | Object Array | Y |  | Array |
| dnca_tot_amt | 예수금총금액 | String | Y | 19 |  |
| nxdy_excc_amt | 익일정산금액 | String | Y | 19 |  |
| prvs_rcdl_excc_amt | 가수도정산금액 | String | Y | 19 |  |
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
| tot_evlu_amt | 총평가금액 | String | Y | 19 |  |
| nass_amt | 순자산금액 | String | Y | 19 |  |
| fncg_gld_auto_rdpt_yn | 융자금자동상환여부 | String | Y | 1 |  |
| pchs_amt_smtl_amt | 매입금액합계금액 | String | Y | 19 |  |
| evlu_amt_smtl_amt | 평가금액합계금액 | String | Y | 19 |  |
| evlu_pfls_smtl_amt | 평가손익합계금액 | String | Y | 19 |  |
| tot_stln_slng_chgs | 총대주매각대금 | String | Y | 19 |  |
| bfdy_tot_asst_evlu_amt | 전일총자산평가금액 | String | Y | 19 |  |
| asst_icdc_amt | 자산증감액 | String | Y | 19 |  |
| asst_icdc_erng_rt | 자산증감수익율 | String | Y | 32 |  |
| rlzt_pfls | 실현손익 | String | Y | 19 |  |
| rlzt_erng_rt | 실현수익율 | String | Y | 32 |  |
| real_evlu_pfls | 실평가손익 | String | Y | 19 |  |
| real_evlu_pfls_erng_rt | 실평가손익수익율 | String | Y | 32 |  |
