<!-- endpoint: /uapi/overseas-price/v1/quotations/price -->
<!-- category: [해외주식] 기본시세 -->
<!-- korean_name: 해외주식 현재체결가 -->

# 해외주식 현재체결가[v1_해외주식-009]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-price/v1/quotations/price
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: HHDFS00000300
- **모의TRID**: HHDFS00000300

## 개요
해외주식종목의 현재체결가를 확인하는 API 입니다.
해외주식 시세는 무료시세(지연시세)만이 제공되며, API로는 유료시세(실시간시세)를 받아보실 수 없습니다.
※ 지연시세 지연시간 : 미국 - 실시간무료(0분 지연, 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보)
홍콩, 베트남, 중국, 일본 - 15분지연
미국의 경우 0분 지연 시세로 제공되나, 장중 당일 시가는 상이할 수 있으며, 익일 정정 표시됩니다.
[미국주식시세 이용시 유의사항]
■ 무료 실시간 시세(나스닥 토탈뷰)를 별도 신청없이 제공하고 있으며, 유료 시세 서비스를 신청하시더라도 OpenAPI의 경우 무료 시세로만 제공하고있습니다.
※ 무료(매수/매도 각 10호가) : 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보
※ 유료(매수/매도 각 1호가) : OpenAPI 서비스 미제공
■ 무료 실시간 시세 서비스는 유료 실시간 시세 서비스 대비 평균 50% 수준에 해당하는 정보이므로 현재가/호가/순간체결량/차트 등에서 일시적·부분적 차이가 있을 수 있습니다.
■ 무료 실시간 시세 서비스의 시가, 저가, 고가, 종가는 타 매체의 유료 실시간 시세 서비스와 다를 수 있으며, 이로 인해 발생하는 손실에 대해서 당사가 책임지지 않습니다.
이용에 유의 부탁드립니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용)법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용)※ 토큰 지정시 토큰 타입("Bearer") 지정 필요. 즉, 발급받은 접근토큰 앞에 앞에 "Bearer" 붙여서 호출EX) "Bearer eyJ..........8GA" |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자/모의투자]HHDFS00000300 |
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
| AUTH | 사용자권한정보 | String | Y | 32 | "" (Null 값 설정) |
| EXCD | 거래소코드 | String | Y | 4 | HKS : 홍콩NYS : 뉴욕NAS : 나스닥AMS : 아멕스TSE : 도쿄SHS : 상해SZS : 심천SHI : 상해지수SZI : 심천지수HSX : 호치민HNX : 하노이BAY : 뉴욕(주간)BAQ : 나스닥(주간)BAA : 아멕스(주간) |
| SYMB | 종목코드 | String | Y | 16 |  |

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
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공 0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 | 응답코드 |
| msg1 | 응답메세지 | String | Y | 80 | 응답메세지 |
| output | 응답상세 | Object | Y | - |  |
| rsym | 실시간조회종목코드 | String | Y | 16 | D+시장구분(3자리)+종목코드예) DNASAAPL : D+NAS(나스닥)+AAPL(애플)[시장구분]NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스 ,TSE : 도쿄, HKS : 홍콩,SHS : 상해, SZS : 심천HSX : 호치민, HNX : 하노이 |
| zdiv | 소수점자리수 | String | Y | 1 |  |
| base | 전일종가 | String | Y | 12 | 전일의 종가 |
| pvol | 전일거래량 | String | Y | 14 | 전일의 거래량 |
| last | 현재가 | String | Y | 12 | 당일 조회시점의 현재 가격 |
| sign | 대비기호 | String | Y | 1 | 1 : 상한2 : 상승3 : 보합4 : 하한5 : 하락 |
| diff | 대비 | String | Y | 12 | 전일 종가와 당일 현재가의 차이 (당일 현재가-전일 종가) |
| rate | 등락율 | String | Y | 12 | 전일 대비 / 당일 현재가 * 100 |
| tvol | 거래량 | String | Y | 14 | 당일 조회시점까지 전체 거래량 |
| tamt | 거래대금 | String | Y | 14 | 당일 조회시점까지 전체 거래금액 |
| ordy | 매수가능여부 | String | Y | 20 | 매수주문 가능 종목 여부 |
