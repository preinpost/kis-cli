<!-- endpoint: /uapi/overseas-price/v1/quotations/inquire-ccnl -->
<!-- category: [해외주식] 기본시세 -->
<!-- korean_name: 해외주식 체결추이 -->

# 해외주식 체결추이[해외주식-037]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-price/v1/quotations/inquire-ccnl
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: HHDFS76200300
- **모의TRID**: 모의투자 미지원

## 개요
해외주식 체결추이 API입니다.
한국투자 HTS(eFriend Plus) > [7625] 해외주식 체결추이 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHDFS76200300 |
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
| EXCD | 거래소명 | String | Y | 4 | 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스 HKS : 홍콩, SHS : 상해 , SZS : 심천HSX : 호치민, HNX : 하노이TSE : 도쿄 ' |
| AUTH | 사용자권한정보 | String | Y | 32 | 공백 |
| KEYB | NEXT KEY BUFF | String | Y | 32 | 공백 |
| TDAY | 당일전일구분 | String | Y | 1 | 0:전일, 1:당일 |
| SYMB | 종목코드 | String | Y | 16 | 해외종목코드 |

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
| output2 | 응답상세 | Object Array | Y |  | array |
| khms | 한국기준시간 | String | Y | 6 |  |
| last | 체결가 | String | Y | 12 |  |
| sign | 기호 | String | Y | 1 |  |
| diff | 대비 | String | Y | 12 |  |
| rate | 등락율 | String | Y | 12 |  |
| evol | 체결량 | String | Y | 10 |  |
| tvol | 거래량 | String | Y | 14 |  |
| mtyp | 시장구분 | String | Y | 1 | 0: 장중 1:장전 2:장후 |
| pbid | 매수호가 | String | Y | 12 |  |
| pask | 매도호가 | String | Y | 12 |  |
| vpow | 체결강도 | String | Y | 10 |  |
| output1 | 응답상세 | Object | Y | - |  |
| rsym | 실시간조회종목코드 | String | Y | 16 |  |
| ZDIV | 소수점자리수 | String | Y | 1 |  |
| NREC | Record Count | String | Y | 4 |  |
