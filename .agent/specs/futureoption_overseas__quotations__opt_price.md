<!-- endpoint: /uapi/overseas-futureoption/v1/quotations/opt-price -->
<!-- category: [해외선물옵션] 기본시세 -->
<!-- korean_name: 해외옵션종목현재가 -->

# 해외옵션종목현재가 [해외선물-035]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-futureoption/v1/quotations/opt-price
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: HHDFO55010000
- **모의TRID**: 모의투자 미지원

## 개요
해외옵션종목현재가 API입니다.
(중요) 해외옵션시세 출력값을 해석하실 때 focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
- focode.mst(해외지수옵션 종목마스터파일), (해외주식옵션 종목마스터파일) 다운로드 방법
1) focode.mst(해외지수옵션 종목마스터파일)
: 포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션 클릭하여 다운로드 후
Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외옵션정보.h)를 참고하여 해석
2) fostkcode.mst(해외주식옵션 종목마스터파일)
: 포럼 > FAQ > 종목정보 다운로드(해외) - 해외주식옵션 클릭하여 다운로드 후
Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외주식옵션정보.h)를 참고하여 해석
- 소수점 계산 시, focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)의 sCalcDesz(계산 소수점) 값 참고
EX) focode.mst 파일의 sCalcDesz(계산 소수점) 값
품목코드 OES 계산소수점 -2 → 시세 7525 수신 시 75.25 로 해석
품목코드 O6E 계산소수점 -4 → 시세 54.0 수신 시 0.0054 로 해석

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHDFO55010000 |
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
| SRS_CD | 종목명 | String | Y | 32 | ex) OESU24 C5500※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고 |

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
| output1 | 응답상세 | Object | Y | - |  |
| proc_date | 최종처리일자 | String | Y | 8 |  |
| proc_time | 최종처리시각 | String | Y | 6 |  |
| open_price | 시가 | String | Y | 15 |  |
| high_price | 고가 | String | Y | 15 |  |
| low_price | 저가 | String | Y | 15 |  |
| last_price | 현재가 | String | Y | 15 | 현재가※ focode.mst, fostkcode.mst* 의 sCalcDesz(계산 소수점) 값 참고* 포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션 |
| vol | 누적거래수량 | String | Y | 10 |  |
| prev_diff_flag | 전일대비구분 | String | Y | 1 |  |
| prev_diff_price | 전일대비가격 | String | Y | 15 |  |
| prev_diff_rate | 전일대비율 | String | Y | 10 |  |
| bid_qntt | 매수1수량 | String | Y | 10 |  |
| bid_price | 매수1호가 | String | Y | 15 |  |
| ask_qntt | 매도1수량 | String | Y | 10 |  |
| ask_price | 매도1호가 | String | Y | 15 |  |
| trst_mgn | 증거금 | String | Y | 19 |  |
| exch_cd | 거래소코드 | String | Y | 10 |  |
| crc_cd | 거래통화 | String | Y | 10 |  |
| trd_fr_date | 상장일 | String | Y | 8 |  |
| expr_date | 만기일 | String | Y | 8 |  |
| trd_to_date | 최종거래일 | String | Y | 8 |  |
| remn_cnt | 잔존일수 | String | Y | 4 |  |
| last_qntt | 체결량 | String | Y | 10 |  |
| tot_ask_qntt | 총매도잔량 | String | Y | 10 |  |
| tot_bid_qntt | 총매수잔량 | String | Y | 10 |  |
| tick_size | 틱사이즈 | String | Y | 19 |  |
| open_date | 장개시일자 | String | Y | 8 |  |
| open_time | 장개시시각 | String | Y | 6 |  |
| close_date | 장종료일자 | String | Y | 8 |  |
| close_time | 장종료시각 | String | Y | 6 |  |
| sbsnsdate | 영업일자 | String | Y | 8 |  |
| sttl_price | 정산가 | String | N | 15 | 정산가 |
