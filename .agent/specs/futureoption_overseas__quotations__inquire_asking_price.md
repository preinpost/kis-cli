<!-- endpoint: /uapi/overseas-futureoption/v1/quotations/inquire-asking-price -->
<!-- category: [해외선물옵션] 기본시세 -->
<!-- korean_name: 해외선물 호가 -->

# 해외선물 호가 [해외선물-031]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-futureoption/v1/quotations/inquire-asking-price
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: HHDFC86000000
- **모의TRID**: 모의투자 미지원

## 개요
해외선물 호가 API입니다.
한국투자 HTS(eFriend Plus) > [8602] 해외선물옵션 종합주문(Ⅰ) 화면에서 "왼쪽 호가 창" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
(중요) 해외선물옵션시세 출력값을 해석하실 때 ffcode.mst(해외선물종목마스터 파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
- ffcode.mst(해외선물종목마스터 파일) 다운로드 방법 2가지
1) 한국투자증권 Github의 파이썬 샘플코드를 사용하여 mst 파일 다운로드 및 excel 파일로 정제
https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/overseas_future_code.py
2) 혹은 포럼 - FAQ - 종목정보 다운로드 - 해외선물옵션 클릭하셔서 ffcode.mst(해외선물종목마스터 파일)을 다운로드 후
Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외선물옵션정보.h)를 참고하여 해석
- 소수점 계산 시, ffcode.mst(해외선물종목마스터 파일)의 sCalcDesz(계산 소수점) 값 참고
EX) ffcode.mst 파일의 sCalcDesz(계산 소수점) 값
품목코드 6A 계산소수점 -4 → 시세 6882.5 수신 시 0.68825 로 해석
품목코드 GC 계산소수점 -1 → 시세 19225 수신 시 1922.5 로 해석
[참고자료]
※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHDFC86000000 |
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
| SRS_CD | 종목명 | String | Y | 32 | 종목코드 |

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
| output1 | 응답상세 | Object | Y |  |  |
| open_price | 시가 | String | Y | 15 |  |
| high_price | 고가 | String | Y | 15 |  |
| lowp_rice | 저가 | String | Y | 15 |  |
| last_price | 현재가 | String | Y | 15 |  |
| prev_price | 전일종가 | String | Y | 15 |  |
| vol | 거래량 | String | Y | 10 |  |
| prev_diff_price | 전일대비가 | String | Y | 15 |  |
| prev_diff_rate | 전일대비율 | String | Y | 10 |  |
| quot_date | 호가수신일자 | String | Y | 8 |  |
| quot_time | 호가수신시각 | String | Y | 6 |  |
| output2 | 응답상세 | Object Array | Y |  | array |
| bid_qntt | 매수수량 | String | Y | 10 |  |
| bid_num | 매수번호 | String | Y | 10 |  |
| bid_price | 매수호가 | String | Y | 15 |  |
| ask_qntt | 매도수량 | String | Y | 10 |  |
| ask_num | 매도번호 | String | Y | 10 |  |
| ask_price | 매도호가 | String | Y | 15 |  |
