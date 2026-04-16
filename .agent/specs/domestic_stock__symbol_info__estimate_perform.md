<!-- endpoint: /uapi/domestic-stock/v1/quotations/estimate-perform -->
<!-- category: [국내주식] 종목정보 -->
<!-- korean_name: 국내주식 종목추정실적 -->

# 국내주식 종목추정실적 [국내주식-187]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/estimate-perform
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: HHKST668300C0
- **모의TRID**: 모의투자 미지원

## 개요
국내주식 종목추정실적 API입니다.
한국투자 HTS(eFriend Plus) > [0613] 종목추정실적 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
※ 본 화면의 추정실적 및 투자의견은 당월 초의 애널리스트의 의견사항이므로 월중 변동 사항이 있을 수 있음을 유의하시기 바랍니다.
※ 종목별 수익추정은 리서치본부에서 매월 발표되는 거래소, 코스닥 160여개 기업에 한정합니다. 구체적인 종목 리스트는 추정종목리스트를 참고하기 바랍니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHKST668300C0 |
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
| SHT_CD | 종목코드 | String | Y | 2 | ex) 265520 |

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
| sht_cd | ELW단축종목코드 | String | Y | 9 |  |
| item_kor_nm | HTS한글종목명 | String | Y | 40 |  |
| name1 | ELW현재가 | String | Y | 10 |  |
| name2 | 전일대비 | String | Y | 10 |  |
| estdate | 전일대비부호 | String | Y | 1 |  |
| rcmd_name | 전일대비율 | String | Y | 82 |  |
| capital | 누적거래량 | String | Y | 18 |  |
| forn_item_lmtrt | 행사가 | String | Y | 112 |  |
| output2 | 응답상세 | Object Array | Y |  | '(추정손익계산서-6개 array) 매출액, 매출액증감율, 영업이익, 영업이익증감율, 순이익, 순이익증감율,' |
| data1 | DATA1 | String | Y | 15 | 결산연월(outblock4) 참조 |
| data2 | DATA2 | String | Y | 15 | 결산연월(outblock4) 참조 |
| data3 | DATA3 | String | Y | 15 | 결산연월(outblock4) 참조 |
| data4 | DATA4 | String | Y | 15 | 결산연월(outblock4) 참조 |
| data5 | DATA5 | String | Y | 15 | 결산연월(outblock4) 참조 |
| output3 | 응답상세 | Object Array | Y |  | '(투자지표-8개 array) EBITDA(십억원), EPS(원), EPS 증감율(0.1%), PER(배, 0.1%), EV/EBITDA(배, 0.1), ROE(0.1%), 부채비율(0.1%), 이자보상배율(0.1%)' |
| data1 | DATA1 | String | Y | 15 | 결산연월(outblock4) 참조 |
| data2 | DATA2 | String | Y | 15 | 결산연월(outblock4) 참조 |
| data3 | DATA3 | String | Y | 15 | 결산연월(outblock4) 참조 |
| data4 | DATA4 | String | Y | 15 | 결산연월(outblock4) 참조 |
| data5 | DATA5 | String | Y | 15 | 결산연월(outblock4) 참조 |
| output4 | 응답상세 | Object Array | Y |  | array |
| dt | 결산년월 | String | Y | 8 | DATA1 ~5 결산월 정보 |
