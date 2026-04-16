<!-- endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-present-balance -->
<!-- category: [국내주식] 주문/계좌 -->
<!-- korean_name: 퇴직연금 체결기준잔고 -->

# 퇴직연금 체결기준잔고[v1_국내주식-032]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/trading/pension/inquire-present-balance
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: TTTC2202R
- **모의TRID**: 모의투자 미지원

## 개요
​※ 55번 계좌(DC가입자계좌)의 경우 해당 API 이용이 불가합니다.
KIS Developers API의 경우 HTS ID에 반드시 연결되어있어야만 API 신청 및 앱정보 발급이 가능한 서비스로 개발되어서 실물계좌가 아닌 55번 계좌는 API 이용이 불가능한 점 양해 부탁드립니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | TTTC2202R |
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
| CANO | 종합계좌번호 | String | Y | 8 |  |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 29 |
| USER_DVSN_CD | 사용자구분코드 | String | Y | 2 | 00 |
| CTX_AREA_FK100 | 연속조회검색조건100 | String | Y | 100 |  |
| CTX_AREA_NK100 | 연속조회키100 | String | Y | 100 |  |
| PRCS_DVSN_CD | 처리구분코드 | String | N | 2 | 00 : 보유 주식 전체 조회01 : 보유 주식 중 0주 주식 숨김 |

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
| output1 | 응답상세1 | Object Array | Y |  | Array |
| cblc_dvsn | 잔고구분 | String | Y | 2 |  |
| cblc_dvsn_name | 잔고구분명 | String | Y | 60 |  |
| pdno | 상품번호 | String | Y | 12 |  |
| prdt_name | 상품명 | String | Y | 60 |  |
| hldg_qty | 보유수량 | String | Y | 19 |  |
| slpsb_qty | 매도가능수량 | String | Y | 10 |  |
| pchs_avg_pric | 매입평균가격 | String | Y | 184 |  |
| evlu_pfls_amt | 평가손익금액 | String | Y | 19 |  |
| evlu_pfls_rt | 평가손익율 | String | Y | 72 |  |
| prpr | 현재가 | String | Y | 19 |  |
| evlu_amt | 평가금액 | String | Y | 19 |  |
| pchs_amt | 매입금액 | String | Y | 19 |  |
| cblc_weit | 잔고비중 | String | Y | 238 |  |
| output2 | 응답상세2 | Object Array | Y |  | Array |
| pchs_amt_smtl_amt | 매입금액합계금액 | String | Y | 19 |  |
| evlu_amt_smtl_amt | 평가금액합계금액 | String | Y | 19 |  |
| evlu_pfls_smtl_amt | 평가손익합계금액 | String | Y | 19 |  |
| trad_pfls_smtl | 매매손익합계 | String | Y | 19 |  |
| thdt_tot_pfls_amt | 당일총손익금액 | String | Y | 19 |  |
| pftrt | 수익률 | String | Y | 238 |  |
