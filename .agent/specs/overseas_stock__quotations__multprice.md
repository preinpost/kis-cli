<!-- endpoint: /uapi/overseas-price/v1/quotations/multprice -->
<!-- category: [해외주식] 기본시세 -->
<!-- korean_name: 해외주식 복수종목 시세조회 -->

# 해외주식 복수종목 시세조회

## Info
- **Method**: GET
- **URL**: /uapi/overseas-price/v1/quotations/multprice
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: HHDFS76220000
- **모의TRID**: 미지원
- **Format**: JSON
- **Content-Type**: application/json; charset=UTF-8

## 개요
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
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 40 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHDFS76220000 |
| tr_cont | 연속 거래 여부 | String | N | 1 | 공백 : 초기 조회 N : 다음 데이터 조회 (output header의 tr_cont가 M일 경우) |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 필수] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| AUTH | 사용자권한정보 | String | Y | 32 | 공백 입력 필수 |
| NREC | 종목요청개수 | String | Y | 4 | 최대 10 |
| EXCD_01 ~ 10 | 거래소코드 | String | Y | 4 | HKS : 홍콩NYS : 뉴욕NAS : 나스닥AMS : 아멕스TSE : 도쿄SHS : 상해SZS : 심천SHI : 상해지수SZI : 심천지수HSX : 호치민HNX : 하노이BAY : 뉴욕(주간)BAQ : 나스닥(주간)BAA : 아멕스(주간) |
| SYMB_01 ~ 10 | 종목코드 | String | Y | 16 | API 문서 -> 종목정보파일 -> 마스터 파일 참조 |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | N | 1 | 공백 : 초기 조회 N : 다음 데이터 조회 (output header의 tr_cont가 M일 경우) |
| gt_uid | Global UID | String | N | 32 | [법인 필수] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 |  |
| msg_cd | 응답코드 | String | Y | 8 |  |
| msg1 | 응답메세지 | String | Y | 80 |  |
| output | 응답상세 | Object | Y |  |  |
| nrec | Output 개수 | String | Y | 4 |  |
| output2 | 응답상세 | Object Array | Y |  | Array |
| rsym | 실시간조회심볼 | String | Y | 16 |  |
| excd | 거래소코드 | String | Y | 4 |  |
| symb | 종목코드 | String | Y | 16 |  |
| knam | 종목명 | String | Y | 48 |  |
| exnm | 거래소명 | String | Y | 20 |  |
| nnam | 국가명 | String | Y | 20 |  |
| stat1 | 실 지 휴 정 재 | String | Y | 4 |  |
| stat2 | 실시간 지연15분 휴장 거래정지 거래재개 | String | Y | 16 |  |
| zdiv | 소수점자리수 | String | Y | 1 |  |
| last | Last Price | String | Y | 12 |  |
| sign | 대비기호 | String | Y | 1 |  |
| diff | 대비 | String | Y | 12 |  |
| rate | 등락율 | String | Y | 12 |  |
| open | 시가 | String | Y | 12 |  |
| high | 고가 | String | Y | 12 |  |
| low | 저가 | String | Y | 12 |  |
| pbid | Bid Price | String | Y | 12 |  |
| pask | Ask Price | String | Y | 12 |  |
| vbid | 매수호가잔량 | String | Y | 10 |  |
| vask | 매도호가잔량 | String | Y | 10 |  |
| bvol | 매수호가총잔량 | String | Y | 10 |  |
| avol | 매도호가총잔량 | String | Y | 10 |  |
| evol | 체결량 | String | Y | 10 |  |
| tvol | 거래량 | String | Y | 14 |  |
| tamt | 거래대금 | String | Y | 14 |  |
| powx | 체결강도 | String | Y | 10 |  |
| xhms | 현지기준시간(HHMMSS) | String | Y | 6 |  |
| khms | 한국기준시간(HHMMSS) | String | Y | 6 |  |
| curr | 통화코드 | String | Y | 4 |  |
| base | Base Price | String | Y | 12 |  |
| pvol | Previous Volume | String | Y | 14 |  |
| pamt | 전일거래대금 | String | Y | 14 |  |
| popen | 전일시가 | String | Y | 12 |  |
| phigh | 전일고가 | String | Y | 12 |  |
| plow | 전일저가 | String | Y | 12 |  |
| shar | 상장주수 | String | Y | 16 |  |
| mcap | 자본금 | String | Y | 16 |  |
| tomv | 시가총액 | String | Y | 16 |  |
| h52p | 52주최고가 | String | Y | 12 |  |
| l52p | 52주최저가 | String | Y | 12 |  |
| h52d | 52주최고일자 | String | Y | 8 |  |
| l52d | 52주최저일자 | String | Y | 8 |  |
| hanp | High Anual Price | String | Y | 12 |  |
| lanp | Low Anual Price | String | Y | 12 |  |
| hand | 연중최고일자 | String | Y | 8 |  |
| land | 연중최저일자 | String | Y | 8 |  |
| bnit | 매매단위 | String | Y | 6 |  |
| t_xprc | 원환산당일가격 | String | Y | 12 |  |
