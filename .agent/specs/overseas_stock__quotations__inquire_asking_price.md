<!-- endpoint: /uapi/overseas-price/v1/quotations/inquire-asking-price -->
<!-- category: [해외주식] 기본시세 -->
<!-- korean_name: 해외주식 현재가 호가 -->

# 해외주식 현재가 호가 [해외주식-033]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-price/v1/quotations/inquire-asking-price
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: HHDFS76200100
- **모의TRID**: 모의투자 미지원

## 개요
해외주식 현재가 호가 API입니다.
미국 거래소는 10호가, 그 외 국가 거래소는 1호가만 제공됩니다.
한국투자 HTS(eFriend Plus) > [7620] 해외주식 현재가 화면에서 "왼쪽 호가 창" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHDFS76200100 |
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
| AUTH | 사용자권한정보 | String | Y | 32 | 공백 |
| EXCD | 거래소코드 | String | Y | 4 | NYS : 뉴욕NAS : 나스닥AMS : 아멕스 HKS : 홍콩SHS : 상해 SZS : 심천HSX : 호치민HNX : 하노이TSE : 도쿄 BAY : 뉴욕(주간)BAQ : 나스닥(주간)BAA : 아멕스(주간) |
| SYMB | 종목코드 | String | Y | 16 | 종목코드 예)TSLA |

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
| output1 | 응답상세 | Object | Y | 100 |  |
| rsym | 실시간조회종목코드 | String | Y | 16 |  |
| zdiv | 소수점자리수 | String | Y | 1 |  |
| curr | 통화 | String | Y | 4 |  |
| base | 전일종가 | String | Y | 12 |  |
| open | 시가 | String | Y | 12 |  |
| high | 고가 | String | Y | 12 |  |
| low | 저가 | String | Y | 12 |  |
| last | 현재가 | String | Y | 12 |  |
| dymd | 호가일자 | String | Y | 8 |  |
| dhms | 호가시간 | String | Y | 6 |  |
| bvol | 매수호가총잔량 | String | Y | 10 |  |
| avol | 매도호가총잔량 | String | Y | 10 |  |
| bdvl | 매수호가총잔량대비 | String | Y | 10 |  |
| advl | 매도호가총잔량대비 | String | Y | 10 |  |
| code | 종목코드 | String | Y | 16 |  |
| ropen | 시가율 | String | Y | 12 |  |
| rhigh | 고가율 | String | Y | 12 |  |
| rlow | 저가율 | String | Y | 12 |  |
| rclose | 현재가율 | String | Y | 12 |  |
| output2 | 응답상세 | Array | Y | 100 |  |
| pbid1 | 매수호가가격1 | String | Y | 12 |  |
| pask1 | 매도호가가격1 | String | Y | 12 |  |
| vbid1 | 매수호가잔량1 | String | Y | 10 |  |
| vask1 | 매도호가잔량1 | String | Y | 10 |  |
| dbid1 | 매수호가대비1 | String | Y | 10 |  |
| dask1 | 매도호가대비1 | String | Y | 10 |  |
| pbid2 | 매수호가가격2 | String | Y | 12 | 미국 거래소만 수신 |
| pask2 | 매도호가가격2 | String | Y | 12 | 미국 거래소만 수신 |
| vbid2 | 매수호가잔량2 | String | Y | 10 | 미국 거래소만 수신 |
| vask2 | 매도호가잔량2 | String | Y | 10 | 미국 거래소만 수신 |
| dbid2 | 매수호가대비2 | String | Y | 10 | 미국 거래소만 수신 |
| dask2 | 매도호가대비2 | String | Y | 10 | 미국 거래소만 수신 |
| pbid3 | 매수호가가격3 | String | Y | 12 | 미국 거래소만 수신 |
| pask3 | 매도호가가격3 | String | Y | 12 | 미국 거래소만 수신 |
| vbid3 | 매수호가잔량3 | String | Y | 10 | 미국 거래소만 수신 |
| vask3 | 매도호가잔량3 | String | Y | 10 | 미국 거래소만 수신 |
| dbid3 | 매수호가대비3 | String | Y | 10 | 미국 거래소만 수신 |
| dask3 | 매도호가대비3 | String | Y | 10 | 미국 거래소만 수신 |
| pbid4 | 매수호가가격4 | String | Y | 12 | 미국 거래소만 수신 |
| pask4 | 매도호가가격4 | String | Y | 12 | 미국 거래소만 수신 |
| vbid4 | 매수호가잔량4 | String | Y | 10 | 미국 거래소만 수신 |
| vask4 | 매도호가잔량4 | String | Y | 10 | 미국 거래소만 수신 |
| dbid4 | 매수호가대비4 | String | Y | 10 | 미국 거래소만 수신 |
| dask4 | 매도호가대비4 | String | Y | 10 | 미국 거래소만 수신 |
| pbid5 | 매수호가가격5 | String | Y | 12 | 미국 거래소만 수신 |
| pask5 | 매도호가가격5 | String | Y | 12 | 미국 거래소만 수신 |
| vbid5 | 매수호가잔량5 | String | Y | 10 | 미국 거래소만 수신 |
| vask5 | 매도호가잔량5 | String | Y | 10 | 미국 거래소만 수신 |
| dbid5 | 매수호가대비5 | String | Y | 10 | 미국 거래소만 수신 |
| dask5 | 매도호가대비5 | String | Y | 10 | 미국 거래소만 수신 |
| pbid6 | 매수호가가격6 | String | Y | 12 | 미국 거래소만 수신 |
| pask6 | 매도호가가격6 | String | Y | 12 | 미국 거래소만 수신 |
| vbid6 | 매수호가잔량6 | String | Y | 10 | 미국 거래소만 수신 |
| vask6 | 매도호가잔량6 | String | Y | 10 | 미국 거래소만 수신 |
| dbid6 | 매수호가대비6 | String | Y | 10 | 미국 거래소만 수신 |
| dask6 | 매도호가대비6 | String | Y | 10 | 미국 거래소만 수신 |
| pbid7 | 매수호가가격7 | String | Y | 12 | 미국 거래소만 수신 |
| pask7 | 매도호가가격7 | String | Y | 12 | 미국 거래소만 수신 |
| vbid7 | 매수호가잔량7 | String | Y | 10 | 미국 거래소만 수신 |
| vask7 | 매도호가잔량7 | String | Y | 10 | 미국 거래소만 수신 |
| dbid7 | 매수호가대비7 | String | Y | 10 | 미국 거래소만 수신 |
| dask7 | 매도호가대비7 | String | Y | 10 | 미국 거래소만 수신 |
| pbid8 | 매수호가가격8 | String | Y | 12 | 미국 거래소만 수신 |
| pask8 | 매도호가가격8 | String | Y | 12 | 미국 거래소만 수신 |
| vbid8 | 매수호가잔량8 | String | Y | 10 | 미국 거래소만 수신 |
| vask8 | 매도호가잔량8 | String | Y | 10 | 미국 거래소만 수신 |
| dbid8 | 매수호가대비8 | String | Y | 10 | 미국 거래소만 수신 |
| dask8 | 매도호가대비8 | String | Y | 10 | 미국 거래소만 수신 |
| pbid9 | 매수호가가격9 | String | Y | 12 | 미국 거래소만 수신 |
| pask9 | 매도호가가격9 | String | Y | 12 | 미국 거래소만 수신 |
| vbid9 | 매수호가잔량9 | String | Y | 10 | 미국 거래소만 수신 |
| vask9 | 매도호가잔량9 | String | Y | 10 | 미국 거래소만 수신 |
| dbid9 | 매수호가대비9 | String | Y | 10 | 미국 거래소만 수신 |
| dask9 | 매도호가대비9 | String | Y | 10 | 미국 거래소만 수신 |
| pbid10 | 매수호가가격10 | String | Y | 12 | 미국 거래소만 수신 |
| pask10 | 매도호가가격10 | String | Y | 12 | 미국 거래소만 수신 |
| vbid10 | 매수호가잔량10 | String | Y | 10 | 미국 거래소만 수신 |
| vask10 | 매도호가잔량10 | String | Y | 10 | 미국 거래소만 수신 |
| dbid10 | 매수호가대비10 | String | Y | 10 | 미국 거래소만 수신 |
| dask10 | 매도호가대비10 | String | Y | 10 | 미국 거래소만 수신 |
| output3 | 응답상세 | Object Array | Y | 100 |  |
| vstm | VCMStart시간 | String | Y | 6 | 데이터 없음 |
| vetm | VCMEnd시간 | String | Y | 6 | 데이터 없음 |
| csbp | CAS/VCM기준가 | String | Y | 12 | 데이터 없음 |
| cshi | CAS/VCMHighprice | String | Y | 12 | 데이터 없음 |
| cslo | CAS/VCMLowprice | String | Y | 12 | 데이터 없음 |
| iep | IEP | String | Y | 12 | 데이터 없음 |
| iev | IEV | String | Y | 12 | 데이터 없음 |
