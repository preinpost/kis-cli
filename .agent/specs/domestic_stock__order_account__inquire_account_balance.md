<!-- endpoint: /uapi/domestic-stock/v1/trading/inquire-account-balance -->
<!-- category: [국내주식] 주문/계좌 -->
<!-- korean_name: 투자계좌자산현황조회 -->

# 투자계좌자산현황조회[v1_국내주식-048]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/trading/inquire-account-balance
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTRP6548R
- **모의TRID**: 모의투자 미지원

## 개요
투자계좌자산현황조회 API입니다.
output1은 한국투자 HTS(eFriend Plus) > [0891] 계좌 자산비중(결제기준) 화면 아래 테이블의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTRP6548R |
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
| INQR_DVSN_1 | 조회구분1 | String | Y | 1 | 공백입력 |
| BSPR_BF_DT_APLY_YN | 기준가이전일자적용여부 | String | Y | 1 | 공백입력 |

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
| Output1 | 응답상세 | Object Array | Y |  | Array [아래 순서대로 출력 : 20항목]1: 주식2: 펀드/MMW3: IMA4: 채권5: ELS/DLS6: WRAP7: 신탁8: RP/발행어음9: 해외주식10: 해외채권11: 금현물12: CD/CP13: 전자단기사채14: 타사상품15: 외화전자단기사채16: 외화 ELS/DLS17: 외화18: 예수금19: 청약자예수금20: 합계[21번 계좌일 경우 : 17항목]1: 수익증권2: IMA3: 채권4: ELS/DLS5: WRAP6: 신탁7: RP8: 외화rp9: 해외채권10: CD/CP11: 전자단기사채12: 외화전자단기사채13: 외화ELS/DLS14: 외화평가금액15: 예수금+cma16: 청약자예수금17: 합계 |
| pchs_amt | 매입금액 | String | Y | 19 |  |
| evlu_amt | 평가금액 | String | Y | 19 |  |
| evlu_pfls_amt | 평가손익금액 | String | Y | 19 |  |
| crdt_lnd_amt | 신용대출금액 | String | Y | 19 |  |
| real_nass_amt | 실제순자산금액 | String | Y | 19 |  |
| whol_weit_rt | 전체비중율 | String | Y | 228 |  |
| Output2 | 응답상세2 | Object | Y |  |  |
| pchs_amt_smtl | 매입금액합계 | String | Y | 19 | 유가매입금액 |
| nass_tot_amt | 순자산총금액 | String | Y | 19 |  |
| loan_amt_smtl | 대출금액합계 | String | Y | 19 |  |
| evlu_pfls_amt_smtl | 평가손익금액합계 | String | Y | 19 | 평가손익금액 |
| evlu_amt_smtl | 평가금액합계 | String | Y | 19 | 유가평가금액 |
| tot_asst_amt | 총자산금액 | String | Y | 19 | 총 자산금액 |
| tot_lnda_tot_ulst_lnda | 총대출금액총융자대출금액 | String | Y | 19 |  |
| cma_auto_loan_amt | CMA자동대출금액 | String | Y | 19 |  |
| tot_mgln_amt | 총담보대출금액 | String | Y | 19 |  |
| stln_evlu_amt | 대주평가금액 | String | Y | 19 |  |
| crdt_fncg_amt | 신용융자금액 | String | Y | 19 |  |
| ocl_apl_loan_amt | OCL_APL대출금액 | String | Y | 19 |  |
| pldg_stup_amt | 질권설정금액 | String | Y | 19 |  |
| frcr_evlu_tota | 외화평가총액 | String | Y | 19 |  |
| tot_dncl_amt | 총예수금액 | String | Y | 19 |  |
| cma_evlu_amt | CMA평가금액 | String | Y | 19 |  |
| dncl_amt | 예수금액 | String | Y | 19 |  |
| tot_sbst_amt | 총대용금액 | String | Y | 19 |  |
| thdt_rcvb_amt | 당일미수금액 | String | Y | 20 |  |
| ovrs_stck_evlu_amt1 | 해외주식평가금액1 | String | Y | 236 |  |
| ovrs_bond_evlu_amt | 해외채권평가금액 | String | Y | 236 |  |
| mmf_cma_mgge_loan_amt | MMFCMA담보대출금액 | String | Y | 19 |  |
| sbsc_dncl_amt | 청약예수금액 | String | Y | 19 |  |
| pbst_sbsc_fnds_loan_use_amt | 공모주청약자금대출사용금액 | String | Y | 20 |  |
| etpr_crdt_grnt_loan_amt | 기업신용공여대출금액 | String | Y | 19 |  |
