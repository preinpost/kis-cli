<!-- endpoint: /uapi/overseas-stock/v1/trading/inquire-paymt-stdr-balance -->
<!-- category: [해외주식] 주문/계좌 -->
<!-- korean_name: 해외주식 결제기준잔고 -->

# 해외주식 결제기준잔고 [해외주식-064]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-stock/v1/trading/inquire-paymt-stdr-balance
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTRP6010R
- **모의TRID**: 모의투자 미지원

## 개요
해외주식 결제기준잔고 API입니다.
한국투자 HTS(eFriend Plus) > [0829] 해외 결제기준잔고 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
※ 적용환율은 당일 매매기준이며, 현재가의 경우 지연된 시세로 평가되므로 실제매도금액과 상이할 수 있습니다.
※ 주문가능수량 : 보유수량 - 미결제 매도수량
※ 매입금액 계산 시 결제일의 최초고시환율을 적용하므로, 금일 최초고시환율을 적용하는 체결기준 잔고와는 상이합니다.
※ 해외증권 투자 및 업무문의 안내: 한국투자증권 해외투자지원부 02)3276-5300

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTRP6010R |
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
| CANO | 종합계좌번호 | String | Y | 8 |  |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 |  |
| BASS_DT | 기준일자 | String | Y | 8 |  |
| WCRC_FRCR_DVSN_CD | 원화외화구분코드 | String | Y | 2 | 01(원화기준),02(외화기준) |
| INQR_DVSN_CD | 조회구분코드 | String | Y | 2 | 00(전체), 01(일반), 02(미니스탁) |

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
| output1 | 응답상세 | Object Array | Y |  | array |
| pdno | 상품번호 | String | Y | 12 |  |
| prdt_name | 상품명 | String | Y | 60 |  |
| cblc_qty13 | 잔고수량13 | String | Y | 238 |  |
| ord_psbl_qty1 | 주문가능수량1 | String | Y | 238 |  |
| avg_unpr3 | 평균단가3 | String | Y | 244 |  |
| ovrs_now_pric1 | 해외현재가격1 | String | Y | 235 |  |
| frcr_pchs_amt | 외화매입금액 | String | Y | 235 |  |
| frcr_evlu_amt2 | 외화평가금액2 | String | Y | 236 |  |
| evlu_pfls_amt2 | 평가손익금액2 | String | Y | 255 |  |
| bass_exrt | 기준환율 | String | Y | 238 |  |
| oprt_dtl_dtime | 조작상세일시 | String | Y | 17 |  |
| buy_crcy_cd | 매수통화코드 | String | Y | 3 |  |
| thdt_sll_ccld_qty1 | 당일매도체결수량1 | String | Y | 238 |  |
| thdt_buy_ccld_qty1 | 당일매수체결수량1 | String | Y | 238 |  |
| evlu_pfls_rt1 | 평가손익율1 | String | Y | 238 |  |
| tr_mket_name | 거래시장명 | String | Y | 60 |  |
| natn_kor_name | 국가한글명 | String | Y | 60 |  |
| std_pdno | 표준상품번호 | String | Y | 12 |  |
| mgge_qty | 담보수량 | String | Y | 19 |  |
| loan_rmnd | 대출잔액 | String | Y | 19 |  |
| prdt_type_cd | 상품유형코드 | String | Y | 3 |  |
| ovrs_excg_cd | 해외거래소코드 | String | Y | 4 |  |
| scts_dvsn_name | 유가증권구분명 | String | Y | 60 |  |
| ldng_cblc_qty | 대여잔고수량 | String | Y | 19 |  |
| output2 | 응답상세 | Object Array | Y |  | array |
| crcy_cd | 통화코드 | String | Y | 3 |  |
| crcy_cd_name | 통화코드명 | String | Y | 60 |  |
| frcr_dncl_amt_2 | 외화예수금액2 | String | Y | 236 |  |
| frst_bltn_exrt | 최초고시환율 | String | Y | 238 |  |
| frcr_evlu_amt2 | 외화평가금액2 | String | Y | 236 |  |
| output3 | 응답상세 | Object | Y |  |  |
| pchs_amt_smtl_amt | 매입금액합계금액 | String | Y | 19 |  |
| tot_evlu_pfls_amt | 총평가손익금액 | String | Y | 238 |  |
| evlu_erng_rt1 | 평가수익율1 | String | Y | 201 |  |
| tot_dncl_amt | 총예수금액 | String | Y | 19 |  |
| wcrc_evlu_amt_smtl | 원화평가금액합계 | String | Y | 236 |  |
| tot_asst_amt2 | 총자산금액2 | String | Y | 236 |  |
| frcr_cblc_wcrc_evlu_amt_smtl | 외화잔고원화평가금액합계 | String | Y | 236 |  |
| tot_loan_amt | 총대출금액 | String | Y | 19 |  |
| tot_ldng_evlu_amt | 총대여평가금액 | String | Y | 9 |  |
