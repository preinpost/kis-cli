<!-- endpoint: /uapi/overseas-stock/v1/trading/inquire-balance -->
<!-- category: [해외주식] 주문/계좌 -->
<!-- korean_name: 해외주식 잔고 -->

# 해외주식 잔고[v1_해외주식-006]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-stock/v1/trading/inquire-balance
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: TTTS3012R
- **모의TRID**: VTTS3012R

## 개요
해외주식 잔고를 조회하는 API 입니다.
한국투자 HTS(eFriend Plus) > [7600] 해외주식 종합주문 화면의 좌측 하단 '실시간잔고' 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
다만 미국주간거래 가능종목에 대해서는 frcr_evlu_pfls_amt(외화평가손익금액), evlu_pfls_rt(평가손익율), ovrs_stck_evlu_amt(해외주식평가금액), now_pric2(현재가격2) 값이 HTS와는 상이하게 표출될 수 있습니다.
(주간시간 시간대에 HTS는 주간시세로 노출, API로는 야간시세로 노출)
실전계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
* 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
* 미니스탁 잔고는 해당 API로 확인이 불가합니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용)법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, Oauth 2.0의 Authorization Code Grant 절차를 준용)※ 토큰 지정시 토큰 타입("Bearer") 지정 필요. 즉, 발급받은 접근토큰 앞에 앞에 "Bearer" 붙여서 호출EX) "Bearer eyJ..........8GA" |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자]TTTS3012R[모의투자]VTTS3012R |
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
| OVRS_EXCG_CD | 해외거래소코드 | String | Y | 4 | [모의]NASD : 나스닥NYSE : 뉴욕 AMEX : 아멕스[실전]NASD : 미국전체NAS : 나스닥NYSE : 뉴욕 AMEX : 아멕스[모의/실전 공통]SEHK : 홍콩SHAA : 중국상해SZAA : 중국심천TKSE : 일본HASE : 베트남 하노이VNSE : 베트남 호치민 |
| TR_CRCY_CD | 거래통화코드 | String | Y | 3 | USD : 미국달러HKD : 홍콩달러CNY : 중국위안화JPY : 일본엔화VND : 베트남동 |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | N | 200 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터) |
| CTX_AREA_NK200 | 연속조회키200 | String | N | 200 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터) |

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
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공 0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 | 응답코드 |
| msg1 | 응답메세지 | String | Y | 80 | 응답메세지 |
| ctx_area_fk200 | 연속조회검색조건200 | String | Y | 200 |  |
| ctx_area_nk200 | 연속조회키200 | String | Y | 200 |  |
| output1 | 응답상세1 | Array | Y | - |  |
| cano | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| acnt_prdt_cd | 계좌상품코드 | String | Y | 2 | 계좌상품코드 |
| prdt_type_cd | 상품유형코드 | String | Y | 3 |  |
| ovrs_pdno | 해외상품번호 | String | Y | 12 |  |
| ovrs_item_name | 해외종목명 | String | Y | 60 |  |
| frcr_evlu_pfls_amt | 외화평가손익금액 | String | Y | 30 | 해당 종목의 매입금액과 평가금액의 외회기준 비교 손익 |
| evlu_pfls_rt | 평가손익율 | String | Y | 10 | 해당 종목의 평가손익을 기준으로 한 수익률 |
| pchs_avg_pric | 매입평균가격 | String | Y | 23 | 해당 종목의 매수 평균 단가 |
| ovrs_cblc_qty | 해외잔고수량 | String | Y | 19 |  |
| ord_psbl_qty | 주문가능수량 | String | Y | 10 | 매도 가능한 주문 수량 |
| frcr_pchs_amt1 | 외화매입금액1 | String | Y | 23 | 해당 종목의 외화 기준 매입금액 |
| ovrs_stck_evlu_amt | 해외주식평가금액 | String | Y | 32 | 해당 종목의 외화 기준 평가금액 |
| now_pric2 | 현재가격2 | String | Y | 25 | 해당 종목의 현재가 |
| tr_crcy_cd | 거래통화코드 | String | Y | 3 | USD : 미국달러HKD : 홍콩달러CNY : 중국위안화JPY : 일본엔화VND : 베트남동 |
| ovrs_excg_cd | 해외거래소코드 | String | Y | 4 | NASD : 나스닥NYSE : 뉴욕AMEX : 아멕스SEHK : 홍콩SHAA : 중국상해SZAA : 중국심천TKSE : 일본HASE : 하노이거래소VNSE : 호치민거래소 |
| loan_type_cd | 대출유형코드 | String | Y | 2 | 00 : 해당사항없음01 : 자기융자일반형03 : 자기융자투자형05 : 유통융자일반형06 : 유통융자투자형07 : 자기대주09 : 유통대주10 : 현금11 : 주식담보대출12 : 수익증권담보대출13 : ELS담보대출14 : 채권담보대출15 : 해외주식담보대출16 : 기업신용공여31 : 소액자동담보대출41 : 매도담보대출42 : 환매자금대출43 : 매입환매자금대출44 : 대여매도담보대출81 : 대차거래82 : 법인CMA론91 : 공모주청약자금대출92 : 매입자금93 : 미수론서비스94 : 대여 |
| loan_dt | 대출일자 | String | Y | 8 | 대출 실행일자 |
| expd_dt | 만기일자 | String | Y | 8 | 대출 만기일자 |
| output2 | 응답상세2 | Object | Y | - |  |
| frcr_pchs_amt1 | 외화매입금액1 | String | Y | 24 |  |
| ovrs_rlzt_pfls_amt | 해외실현손익금액 | String | Y | 20 |  |
| ovrs_tot_pfls | 해외총손익 | String | Y | 24 |  |
| rlzt_erng_rt | 실현수익율 | String | Y | 32 |  |
| tot_evlu_pfls_amt | 총평가손익금액 | String | Y | 32 |  |
| tot_pftrt | 총수익률 | String | Y | 32 |  |
| frcr_buy_amt_smtl1 | 외화매수금액합계1 | String | Y | 25 |  |
| ovrs_rlzt_pfls_amt2 | 해외실현손익금액2 | String | Y | 24 |  |
| frcr_buy_amt_smtl2 | 외화매수금액합계2 | String | Y | 25 |  |
