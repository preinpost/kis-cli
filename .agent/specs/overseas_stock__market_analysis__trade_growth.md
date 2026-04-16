<!-- endpoint: /uapi/overseas-stock/v1/ranking/trade-growth -->
<!-- category: [해외주식] 시세분석 -->
<!-- korean_name: 해외주식 거래증가율순위 -->

# 해외주식 거래증가율순위[해외주식-045]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-stock/v1/ranking/trade-growth
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: HHDFS76330000
- **모의TRID**: 모의투자 미지원

## 개요
해외주식 거래증가율순위 API입니다.
한국투자 HTS(eFriend Plus) > [7633] 거래증가율순위 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHDFS76330000 |
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
| KEYB | NEXT KEY BUFF | String | Y | 8 | 공백 |
| AUTH | 사용자권한정보 | String | Y | 32 | 공백 |
| EXCD | 거래소코드 | String | Y | 4 | 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스 HKS : 홍콩, SHS : 상해 , SZS : 심천HSX : 호치민, HNX : 하노이TSE : 도쿄 ' |
| NDAY | N일자값 | String | Y | 1 | N일전 : 0(당일), 1(2일), 2(3일), 3(5일), 4(10일), 5(20일전), 6(30일), 7(60일), 8(120일), 9(1년) |
| VOL_RANG | 거래량조건 | String | Y | 1 | 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상) |

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
| msg1 | 응답메시지 | String | Y | 80 |  |
| output1 | 응답상세 | Object | Y | - |  |
| zdiv | 소수점자리수 | String | Y | 1 |  |
| stat | 거래상태정보 | String | Y | 20 |  |
| crec | 현재조회종목수 | String | Y | 6 |  |
| trec | 전체조회종목수 | String | Y | 6 |  |
| nrec | RecordCount | String | Y | 4 |  |
| output2 | 응답상세 | Object Array | Y |  | array |
| rsym | 실시간조회심볼 | String | Y | 16 |  |
| excd | 거래소코드 | String | Y | 4 |  |
| symb | 종목코드 | String | Y | 1 |  |
| name | 종목명 | String | Y | 48 |  |
| last | 현재가 | String | Y | 16 |  |
| sign | 기호 | String | Y | 1 |  |
| diff | 대비 | String | Y | 12 |  |
| rate | 등락율 | String | Y | 12 |  |
| pask | 매도호가 | String | Y | 12 |  |
| pbid | 매수호가 | String | Y | 12 |  |
| tvol | 거래량 | String | Y | 14 |  |
| n_tvol | 평균거래량 | String | Y | 14 |  |
| n_rate | 증가율 | String | Y | 12 |  |
| rank | 순위 | String | Y | 6 |  |
| ename | 영문종목명 | String | Y | 48 |  |
| e_ordyn | 매매가능 | String | Y | 2 |  |
