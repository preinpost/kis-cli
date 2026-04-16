<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-investor -->
<!-- category: [국내주식] 기본시세 -->
<!-- korean_name: 주식현재가 투자자 -->

# 주식현재가 투자자[v1_국내주식-012]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-investor
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: FHKST01010900
- **모의TRID**: FHKST01010900

## 개요
주식현재가 투자자 API입니다. 개인, 외국인, 기관 등 투자 정보를 확인할 수 있습니다.
[유의사항]
- 외국인은 외국인(외국인투자등록 고유번호가 있는 경우)+기타 외국인을 지칭합니다.
- 당일 데이터는 장 종료 후 제공됩니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHKST01010900 |
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
| FID_COND_MRKT_DIV_CODE | 조건 시장 분류 코드 | String | Y | 2 | J : KRX, NX : NXT, UN : 통합 |
| FID_INPUT_ISCD | 입력 종목코드 | String | Y | 12 | 종목코드 (ex 005930 삼성전자) |

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
| output | 응답상세 | Object Array | Y |  | Array |
| stck_bsop_date | 주식 영업 일자 | String | Y | 8 |  |
| stck_clpr | 주식 종가 | String | Y | 10 |  |
| prdy_vrss | 전일 대비 | String | Y | 10 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| prsn_ntby_qty | 개인 순매수 수량 | String | Y | 12 |  |
| frgn_ntby_qty | 외국인 순매수 수량 | String | Y | 12 |  |
| orgn_ntby_qty | 기관계 순매수 수량 | String | Y | 18 |  |
| prsn_ntby_tr_pbmn | 개인 순매수 거래 대금 | String | Y | 18 |  |
| frgn_ntby_tr_pbmn | 외국인 순매수 거래 대금 | String | Y | 18 |  |
| orgn_ntby_tr_pbmn | 기관계 순매수 거래 대금 | String | Y | 18 |  |
| prsn_shnu_vol | 개인 매수2 거래량 | String | Y | 18 |  |
| frgn_shnu_vol | 외국인 매수2 거래량 | String | Y | 18 |  |
| orgn_shnu_vol | 기관계 매수2 거래량 | String | Y | 18 |  |
| prsn_shnu_tr_pbmn | 개인 매수2 거래 대금 | String | Y | 18 |  |
| frgn_shnu_tr_pbmn | 외국인 매수2 거래 대금 | String | Y | 18 |  |
| orgn_shnu_tr_pbmn | 기관계 매수2 거래 대금 | String | Y | 18 |  |
| prsn_seln_vol | 개인 매도 거래량 | String | Y | 18 |  |
| frgn_seln_vol | 외국인 매도 거래량 | String | Y | 18 |  |
| orgn_seln_vol | 기관계 매도 거래량 | String | Y | 18 |  |
| prsn_seln_tr_pbmn | 개인 매도 거래 대금 | String | Y | 18 |  |
| frgn_seln_tr_pbmn | 외국인 매도 거래 대금 | String | Y | 18 |  |
| orgn_seln_tr_pbmn | 기관계 매도 거래 대금 | String | Y | 18 |  |
