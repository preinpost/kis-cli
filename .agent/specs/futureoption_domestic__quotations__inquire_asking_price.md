<!-- endpoint: /uapi/domestic-futureoption/v1/quotations/inquire-asking-price -->
<!-- category: [국내선물옵션] 기본시세 -->
<!-- korean_name: 선물옵션 시세호가 -->

# 선물옵션 시세호가[v1_국내선물-007]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-futureoption/v1/quotations/inquire-asking-price
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: FHMIF10010000
- **모의TRID**: FHMIF10010000

## 개요
선물옵션 시세호가 API입니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용)※ 토큰 지정시 토큰 타입("Bearer") 지정 필요. 즉, 발급받은 접근토큰 앞에 앞에 "Bearer" 붙여서 호출EX) "Bearer eyJ..........8GA" |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전/모의투자]FHMIF10010000 : 선물 옵션 시세 호가 |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객타입 | String | N | 1 | B : 법인P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| FID_COND_MRKT_DIV_CODE | FID 조건 시장 분류 코드 | String | Y | 2 | F: 지수선물, O:지수옵션JF: 주식선물, JO:주식옵션CF: 상품선물(금), 금리선물(국채), 통화선물(달러)CM: 야간선물, EU: 야간옵션 |
| FID_INPUT_ISCD | FID 입력 종목코드 | String | Y | 12 | 종목코드 (예: 101S03) |

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
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 | 응답코드 |
| msg1 | 응답메세지 | String | Y | 80 | 응답메세지 |
| output1 | 응답상세1 | Object | Y | - |  |
| hts_kor_isnm | HTS 한글 종목명 | String | Y | 40 | 종목명 |
| futs_prpr | 선물 현재가 | String | Y | 14 | 선물의 현재가격 |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 | 1 : 상한 2 : 상승3 : 보합4 : 하한5 : 하락 |
| futs_prdy_vrss | 선물 전일 대비 | String | Y | 14 | 선물의 전일 종가와 당일 현재가의 차이 (당일 현재가-전일 종가) |
| futs_prdy_ctrt | 선물 전일 대비율 | String | Y | 11 | 선물 전일 대비 / 당일 현재가 * 100 |
| acml_vol | 누적 거래량 | String | Y | 18 | 당일 조회시점까지 전체 거래량 |
| futs_prdy_clpr | 선물 전일 종가 | String | Y | 14 | 해당 선물 종목의 전일 종가 |
| futs_shrn_iscd | 선물 단축 종목코드 | String | Y | 9 |  |
| output2 | 응답상세2 | Object Array | Y | - | Array |
| futs_askp1 | 선물 매도호가1 | String | Y | 14 | 해당 종목의 매도호가 중 1번째 낮은 호가 |
| futs_askp2 | 선물 매도호가2 | String | Y | 14 | 해당 종목의 매도호가 중 2번째 낮은 호가 |
| futs_askp3 | 선물 매도호가3 | String | Y | 14 | 해당 종목의 매도호가 중 3번째 낮은 호가 |
| futs_askp4 | 선물 매도호가4 | String | Y | 14 | 해당 종목의 매도호가 중 4번째 낮은 호가 |
| futs_askp5 | 선물 매도호가5 | String | Y | 14 | 해당 종목의 매도호가 중 5번째 낮은 호가 |
| futs_bidp1 | 선물 매수호가1 | String | Y | 14 | 해당 종목의 매수호가 중 가장 높은 호가 |
| futs_bidp2 | 선물 매수호가1 | String | Y | 14 | 해당 종목의 매수호가 중 2번째 높은 호가 |
| futs_bidp3 | 선물 매수호가3 | String | Y | 14 | 해당 종목의 매수호가 중 3번째 높은 호가 |
| futs_bidp4 | 선물 매수호가4 | String | Y | 14 | 해당 종목의 매수호가 중 4번째 높은 호가 |
| futs_bidp5 | 선물 매수호가5 | String | Y | 14 | 해당 종목의 매수호가 중 5번째 높은 호가 |
| askp_rsqn1 | 매도호가 잔량1 | String | Y | 12 | 매도호가 1의 미체결수량 |
| askp_rsqn2 | 매도호가 잔량2 | String | Y | 12 | 매도호가 2의 미체결수량 |
| askp_rsqn3 | 매도호가 잔량3 | String | Y | 12 | 매도호가 3의 미체결수량 |
| askp_rsqn4 | 매도호가 잔량4 | String | Y | 12 | 매도호가 4의 미체결수량 |
| askp_rsqn5 | 매도호가 잔량5 | String | Y | 12 | 매도호가 5의 미체결수량 |
| bidp_rsqn1 | 매수호가 잔량1 | String | Y | 12 | 매수호가 1의 미체결수량 |
| bidp_rsqn2 | 매수호가 잔량2 | String | Y | 12 | 매수호가 2의 미체결수량 |
| bidp_rsqn3 | 매수호가 잔량3 | String | Y | 12 | 매수호가 3의 미체결수량 |
| bidp_rsqn4 | 매수호가 잔량4 | String | Y | 12 | 매수호가 4의 미체결수량 |
| bidp_rsqn5 | 매수호가 잔량5 | String | Y | 12 | 매수호가 5의 미체결수량 |
| askp_csnu1 | 매도호가 건수1 | String | Y | 10 | 매도호가 1의 미체결 주문 건수 |
| askp_csnu2 | 매도호가 건수2 | String | Y | 10 | 매도호가 2의 미체결 주문 건수 |
| askp_csnu3 | 매도호가 건수3 | String | Y | 10 | 매도호가 3의 미체결 주문 건수 |
| askp_csnu4 | 매도호가 건수4 | String | Y | 10 | 매도호가 4의 미체결 주문 건수 |
| askp_csnu5 | 매도호가 건수5 | String | Y | 10 | 매도호가 5의 미체결 주문 건수 |
| bidp_csnu1 | 매수호가 건수1 | String | Y | 10 | 매수호가 1의 미체결 주문 건수 |
| bidp_csnu2 | 매수호가 건수2 | String | Y | 10 | 매수호가 2의 미체결 주문 건수 |
| bidp_csnu3 | 매수호가 건수3 | String | Y | 10 | 매수호가 3의 미체결 주문 건수 |
| bidp_csnu4 | 매수호가 건수4 | String | Y | 10 | 매수호가 4의 미체결 주문 건수 |
| bidp_csnu5 | 매수호가 건수5 | String | Y | 10 | 매수호가 5의 미체결 주문 건수 |
| total_askp_rsqn | 총 매도호가 잔량 | String | Y | 12 | 매도호가 1~5의 잔량 합계 |
| total_bidp_rsqn | 총 매수호가 잔량 | String | Y | 12 | 매수호가 1~5의 잔량 합계 |
| total_askp_csnu | 총 매도호가 건수 | String | Y | 10 | 매도호가 1~5의 미체결 주문 건수 합계 |
| total_bidp_csnu | 총 매수호가 건수 | String | Y | 10 | 매수호가 1~5의 미체결 주문 건수 합계 |
| aspr_acpt_hour | 호가 접수 시간 | String | Y | 6 | 가장 최근 호가의 접수 시간 |
