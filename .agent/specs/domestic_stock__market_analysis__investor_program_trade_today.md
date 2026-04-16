<!-- endpoint: /uapi/domestic-stock/v1/quotations/investor-program-trade-today -->
<!-- category: [국내주식] 시세분석 -->
<!-- korean_name: 프로그램매매 투자자매매동향(당일) -->

# 프로그램매매 투자자매매동향(당일) [국내주식-116]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/investor-program-trade-today
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: HHPPG046600C1
- **모의TRID**: 모의투자 미지원

## 개요
프로그램매매 투자자매매동향(당일) API입니다.
한국투자 HTS(eFriend Plus) > [0466] 프로그램매매 투자자별 동향 화면 의 "당일동향" 표의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | '※ 구TR은 사전고지 없이 막힐 수 있으므로 반드시 신TR로 변경이용 부탁드립니다.[실전투자](구)HHPPG046600C0 → (신)HHPPG046600C1' |
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
| EXCH_DIV_CLS_CODE | 거래소 구분 코드 | String | Y | 2 | J : KRX, NX : NXT, UN : 통합 |
| MRKT_DIV_CLS_CODE | 시장 구분 코드 | String | Y | 1 | 1:코스피, 4:코스닥 |

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
| output1 | 응답상세 | Object Array | Y |  | array |
| invr_cls_code | 투자자코드 | String | Y | 4 |  |
| all_seln_qty | 전체매도수량 | String | Y | 18 |  |
| all_seln_amt | 전체매도대금 | String | Y | 18 |  |
| invr_cls_name | 투자자 구분 명 | String | Y | 20 |  |
| all_shnu_qty | 전체매수수량 | String | Y | 18 |  |
| all_shnu_amt | 전체매수대금 | String | Y | 18 |  |
| all_ntby_amt | 전체순매수대금 | String | Y | 12 |  |
| arbt_seln_qty | 차익매도수량 | String | Y | 18 |  |
| all_ntby_qty | 전체순매수수량 | String | Y | 12 |  |
| arbt_shnu_qty | 차익매수수량 | String | Y | 18 |  |
| arbt_ntby_qty | 차익순매수수량 | String | Y | 12 |  |
| arbt_seln_amt | 차익매도대금 | String | Y | 18 |  |
| arbt_shnu_amt | 차익매수대금 | String | Y | 18 |  |
| arbt_ntby_amt | 차익순매수대금 | String | Y | 12 |  |
| nabt_seln_qty | 비차익매도수량 | String | Y | 18 |  |
| nabt_shnu_qty | 비차익매수수량 | String | Y | 18 |  |
| nabt_ntby_qty | 비차익순매수수량 | String | Y | 12 |  |
| nabt_seln_amt | 비차익매도대금 | String | Y | 18 |  |
| nabt_shnu_amt | 비차익매수대금 | String | Y | 18 |  |
| nabt_ntby_amt | 비차익순매수대금 | String | Y | 12 |  |
