<!-- endpoint: /uapi/domestic-stock/v1/quotations/mktfunds -->
<!-- category: [국내주식] 시세분석 -->
<!-- korean_name: 국내 증시자금 종합 -->

# 국내 증시자금 종합 [국내주식-193]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/mktfunds
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: FHKST649100C0
- **모의TRID**: 모의투자 미지원

## 개요
국내 증시자금 종합 API입니다.
한국투자 HTS(eFriend Plus) > [0470] 증시자금 종합 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다. (단위: 억원)
※ 해당자료는 금융투자협회의 자료를 제공하고 있으며, 오류와 지연이 발생할 수 있습니다.
※ 위 정보에 의한 투자판단의 최종책임은 정보이용자에게 있으며, 당사와 한국금융투자협회는 어떠한 법적인 책임도 지지 않사오니 투자에 참고로만 이용하시기 바랍니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHKST649100C0 |
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
| FID_INPUT_DATE_1 | 입력날짜1 | String | Y | 10 |  |

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
| output | 응답상세 | Object Array | Y |  | array |
| bsop_date | 영업일자 | String | Y | 8 |  |
| bstp_nmix_prpr | 업종지수현재가 | String | Y | 112 |  |
| bstp_nmix_prdy_vrss | 업종지수전일대비 | String | Y | 112 |  |
| prdy_vrss_sign | 전일대비부호 | String | Y | 1 | 1. 상한 2. 상승 3. 보합 4. 하한 5. 하락 |
| prdy_ctrt | 전일대비율 | String | Y | 82 |  |
| hts_avls | HTS시가총액 | String | Y | 18 | 단위: 백만원 |
| cust_dpmn_amt | 고객예탁금금액 | String | Y | 18 | 단위: 억원 |
| cust_dpmn_amt_prdy_vrss | 고객예탁금금액전일대비 | String | Y | 18 |  |
| amt_tnrt | 금액회전율 | String | Y | 84 |  |
| uncl_amt | 미수금액 | String | Y | 18 | 단위: 억원 |
| crdt_loan_rmnd | 신용융자잔고 | String | Y | 18 | 단위: 억원 |
| futs_tfam_amt | 선물예수금금액 | String | Y | 18 | 단위: 억원 |
| sttp_amt | 주식형금액 | String | Y | 18 | 단위: 억원 |
| mxtp_amt | 혼합형금액 | String | Y | 18 | 단위: 억원 |
| bntp_amt | 채권형금액 | String | Y | 18 | 단위: 억원 |
| mmf_amt | MMF금액 | String | Y | 18 | 단위: 억원 |
| secu_lend_amt | 담보대출잔고금액 | String | Y | 18 | 단위: 억원 |
