<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-member -->
<!-- category: [국내주식] 기본시세 -->
<!-- korean_name: 주식현재가 회원사 -->

# 주식현재가 회원사[v1_국내주식-013]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-member
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: FHKST01010600
- **모의TRID**: FHKST01010600

## 개요
주식 현재가 회원사 API입니다. 회원사의 투자 정보를 확인할 수 있습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용)법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자/모의투자]FHKST01010600 : 주식현재가 회원사 |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객타입 | String | N | 1 | B : 법인P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| FID_COND_MRKT_DIV_CODE | FID 조건 시장 분류 코드 | String | Y | 2 | J:KRX, NX:NXT, UN:통합 |
| FID_INPUT_ISCD | FID 입력 종목코드 | String | Y | 12 | 종목번호 (6자리)ETN의 경우, Q로 시작 (EX. Q500001) |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | Y | 1 | tr_cont를 이용한 다음조회 불가 API |
| gt_uid | Global UID | String | Y | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 | 성공 실패 여부 성공 : 0 실패 : 0외 값 |
| msg_cd | 응답코드 | String | Y | 8 | 응답코드 |
| msg1 | 응답메세지 | String | Y | 80 | 응답메세지 |
| output | 응답상세 | Array | Y | null |  |
| seln_mbcr_no1 | 매도 회원사 번호1 | String | Y | 5 |  |
| seln_mbcr_no2 | 매도 회원사 번호2 | String | Y | 5 |  |
| seln_mbcr_no3 | 매도 회원사 번호3 | String | Y | 5 |  |
| seln_mbcr_no4 | 매도 회원사 번호4 | String | Y | 5 |  |
| seln_mbcr_no5 | 매도 회원사 번호5 | String | Y | 5 |  |
| seln_mbcr_name1 | 매도 회원사 명1 | String | Y | 40 |  |
| seln_mbcr_name2 | 매도 회원사 명2 | String | Y | 40 |  |
| seln_mbcr_name3 | 매도 회원사 명3 | String | Y | 40 |  |
| seln_mbcr_name4 | 매도 회원사 명4 | String | Y | 40 |  |
| seln_mbcr_name5 | 매도 회원사 명5 | String | Y | 40 |  |
| total_seln_qty1 | 총 매도 수량1 | String | Y | 18 |  |
| total_seln_qty2 | 총 매도 수량2 | String | Y | 18 |  |
| total_seln_qty3 | 총 매도 수량3 | String | Y | 18 |  |
| total_seln_qty4 | 총 매도 수량4 | String | Y | 18 |  |
| total_seln_qty5 | 총 매도 수량5 | String | Y | 18 |  |
| seln_mbcr_rlim1 | 매도 회원사 비중1 | String | Y | 9 |  |
| seln_mbcr_rlim2 | 매도 회원사 비중2 | String | Y | 9 |  |
| seln_mbcr_rlim3 | 매도 회원사 비중3 | String | Y | 9 |  |
| seln_mbcr_rlim4 | 매도 회원사 비중4 | String | Y | 9 |  |
| seln_mbcr_rlim5 | 매도 회원사 비중5 | String | Y | 9 |  |
| seln_qty_icdc1 | 매도 수량 증감1 | String | Y | 10 |  |
| seln_qty_icdc2 | 매도 수량 증감2 | String | Y | 10 |  |
| seln_qty_icdc3 | 매도 수량 증감3 | String | Y | 10 |  |
| seln_qty_icdc4 | 매도 수량 증감4 | String | Y | 10 |  |
| seln_qty_icdc5 | 매도 수량 증감5 | String | Y | 10 |  |
| shnu_mbcr_no1 | 매수2 회원사 번호1 | String | Y | 5 |  |
| shnu_mbcr_no2 | 매수2 회원사 번호2 | String | Y | 5 |  |
| shnu_mbcr_no3 | 매수2 회원사 번호3 | String | Y | 5 |  |
| shnu_mbcr_no4 | 매수2 회원사 번호4 | String | Y | 5 |  |
| shnu_mbcr_no5 | 매수2 회원사 번호5 | String | Y | 5 |  |
| shnu_mbcr_name1 | 매수2 회원사 명1 | String | Y | 40 |  |
| shnu_mbcr_name2 | 매수2 회원사 명2 | String | Y | 40 |  |
| shnu_mbcr_name3 | 매수2 회원사 명3 | String | Y | 40 |  |
| shnu_mbcr_name4 | 매수2 회원사 명4 | String | Y | 40 |  |
| shnu_mbcr_name5 | 매수2 회원사 명5 | String | Y | 40 |  |
| total_shnu_qty1 | 총 매수2 수량1 | String | Y | 18 |  |
| total_shnu_qty2 | 총 매수2 수량2 | String | Y | 18 |  |
| total_shnu_qty3 | 총 매수2 수량3 | String | Y | 18 |  |
| total_shnu_qty4 | 총 매수2 수량4 | String | Y | 18 |  |
| total_shnu_qty5 | 총 매수2 수량5 | String | Y | 18 |  |
| shnu_mbcr_rlim1 | 매수2 회원사 비중1 | String | Y | 9 |  |
| shnu_mbcr_rlim2 | 매수2 회원사 비중2 | String | Y | 9 |  |
| shnu_mbcr_rlim3 | 매수2 회원사 비중3 | String | Y | 9 |  |
| shnu_mbcr_rlim4 | 매수2 회원사 비중4 | String | Y | 9 |  |
| shnu_mbcr_rlim5 | 매수2 회원사 비중5 | String | Y | 9 |  |
| shnu_qty_icdc1 | 매수2 수량 증감1 | String | Y | 10 |  |
| shnu_qty_icdc2 | 매수2 수량 증감2 | String | Y | 10 |  |
| shnu_qty_icdc3 | 매수2 수량 증감3 | String | Y | 10 |  |
| shnu_qty_icdc4 | 매수2 수량 증감4 | String | Y | 10 |  |
| shnu_qty_icdc5 | 매수2 수량 증감5 | String | Y | 10 |  |
| glob_total_seln_qty | 외국계 총 매도 수량 | String | Y | 18 |  |
| glob_seln_rlim | 외국계 매도 비중 | String | Y | 9 |  |
| glob_ntby_qty | 외국계 순매수 수량 | String | Y | 12 |  |
| glob_total_shnu_qty | 외국계 총 매수2 수량 | String | Y | 18 |  |
| glob_shnu_rlim | 외국계 매수2 비중 | String | Y | 9 |  |
| seln_mbcr_glob_yn_1 | 매도 회원사 외국계 여부1 | String | Y | 1 |  |
| seln_mbcr_glob_yn_2 | 매도 회원사 외국계 여부2 | String | Y | 1 |  |
| seln_mbcr_glob_yn_3 | 매도 회원사 외국계 여부3 | String | Y | 1 |  |
| seln_mbcr_glob_yn_4 | 매도 회원사 외국계 여부4 | String | Y | 1 |  |
| seln_mbcr_glob_yn_5 | 매도 회원사 외국계 여부5 | String | Y | 1 |  |
| shnu_mbcr_glob_yn_1 | 매수2 회원사 외국계 여부1 | String | Y | 1 |  |
| shnu_mbcr_glob_yn_2 | 매수2 회원사 외국계 여부2 | String | Y | 1 |  |
| shnu_mbcr_glob_yn_3 | 매수2 회원사 외국계 여부3 | String | Y | 1 |  |
| shnu_mbcr_glob_yn_4 | 매수2 회원사 외국계 여부4 | String | Y | 1 |  |
| shnu_mbcr_glob_yn_5 | 매수2 회원사 외국계 여부5 | String | Y | 1 |  |
| glob_total_seln_qty_icdc | 외국계 총 매도 수량 증감 | String | Y | 10 |  |
| glob_total_shnu_qty_icdc | 외국계 총 매수2 수량 증감 | String | Y | 10 |  |
