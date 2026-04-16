<!-- endpoint: /uapi/domestic-futureoption/v1/trading/inquire-deposit -->
<!-- category: [국내선물옵션] 주문/계좌 -->
<!-- korean_name: 선물옵션 총자산현황 -->

# 선물옵션 총자산현황[v1_국내선물-014]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-futureoption/v1/trading/inquire-deposit
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTRP6550R
- **모의TRID**: 모의투자 미지원

## 개요
선물옵션 총자산현황 API 입니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTRP6550R |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
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
| output | 응답상세 | Object | Y |  |  |
| dnca_tota | 예수금총액 | String | Y | 19 |  |
| bfdy_chck_amt | 전일수표금액 | String | Y | 19 |  |
| thdt_chck_amt | 당일수표금액 | String | Y | 19 |  |
| rlth_uwdl_dpos_amt | 실물인수도예치금액 | String | Y | 19 |  |
| brkg_mgna_cash | 위탁증거금현금 | String | Y | 19 |  |
| wdrw_psbl_tot_amt | 인출가능총금액 | String | Y | 19 |  |
| ord_psbl_cash | 주문가능현금 | String | Y | 19 |  |
| ord_psbl_tota | 주문가능총액 | String | Y | 19 |  |
| dnca_sbst | 예수금대용 | String | Y | 19 |  |
| scts_sbst_amt | 유가증권대용금액 | String | Y | 19 |  |
| frcr_evlu_amt | 외화평가금액 | String | Y | 19 |  |
| brkg_mgna_sbst | 위탁증거금대용 | String | Y | 19 |  |
| sbst_rlse_psbl_amt | 대용해제가능금액 | String | Y | 19 |  |
| mtnc_rt | 유지비율 | String | Y | 238 |  |
| add_mgna_tota | 추가증거금총액 | String | Y | 19 |  |
| add_mgna_cash | 추가증거금현금 | String | Y | 19 |  |
| rcva | 미수금 | String | Y | 19 |  |
| futr_trad_pfls | 선물매매손익 | String | Y | 19 |  |
| opt_trad_pfls_amt | 옵션매매손익금액 | String | Y | 19 |  |
| trad_pfls_smtl | 매매손익합계 | String | Y | 19 |  |
| futr_evlu_pfls_amt | 선물평가손익금액 | String | Y | 19 |  |
| opt_evlu_pfls_amt | 옵션평가손익금액 | String | Y | 19 |  |
| evlu_pfls_smtl | 평가손익합계 | String | Y | 19 |  |
| excc_dfpa | 정산차금 | String | Y | 19 |  |
| opt_dfpa | 옵션차금 | String | Y | 19 |  |
| brkg_fee | 위탁수수료 | String | Y | 19 |  |
| nxdy_dnca | 익일예수금 | String | Y | 19 |  |
| prsm_dpast_amt | 추정예탁자산금액 | String | Y | 19 |  |
| cash_mntn_amt | 현금유지금액 | String | Y | 19 |  |
| hack_acdt_acnt_move_amt | 해킹사고계좌이전금액 | String | Y | 19 |  |
