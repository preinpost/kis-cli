<!-- endpoint: /uapi/overseas-price/v1/quotations/news-title -->
<!-- category: [해외주식] 시세분석 -->
<!-- korean_name: 해외뉴스종합(제목) -->

# 해외뉴스종합(제목) [해외주식-053]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-price/v1/quotations/news-title
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: HHPSTH60100C1
- **모의TRID**: 모의투자 미지원

## 개요
해외뉴스종합(제목) API입니다.
한국투자 HTS(eFriend Plus) > [7702] 해외뉴스종합 화면의 "우측 상단 뉴스목록" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 |  | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 |  | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 |  | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 |  | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 |  | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID |  | Y | 13 | HHPSTH60100C1 |
| tr_cont | 연속 거래 여부 |  | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객 타입 |  | Y | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 |  | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 |  | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 |  | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP |  | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID |  | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| INFO_GB | 뉴스구분 |  | Y | 1 | 전체: 공백 |
| CLASS_CD | 중분류 |  | Y | 2 | 전체: 공백 |
| NATION_CD | 국가코드 |  | Y | 2 | 전체: 공백CN(중국), HK(홍콩), US(미국) |
| EXCHANGE_CD | 거래소코드 |  | Y | 3 | 전체: 공백 |
| SYMB | 종목코드 |  | Y | 20 | 전체: 공백 |
| DATA_DT | 조회일자 |  | Y | 8 | 전체: 공백특정일자(YYYYMMDD) ex. 20240502 |
| DATA_TM | 조회시간 |  | Y | 6 | 전체: 공백전체: 공백특정시간(HHMMSS) ex. 093500 |
| CTS | 다음키 |  | Y | 35 | 공백 입력 |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 |  | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID |  | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 |  | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| gt_uid | Global UID |  | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 |  | Y | 1 |  |
| msg_cd | 응답코드 |  | Y | 8 |  |
| msg1 | 응답메세지 |  | Y | 80 |  |
| outblock1 | 응답상세 |  | Y |  | array |
| info_gb | 뉴스구분 |  | Y | 1 |  |
| news_key | 뉴스키 |  | Y | 20 |  |
| data_dt | 조회일자 |  | Y | 8 |  |
| data_tm | 조회시간 |  | Y | 6 |  |
| class_cd | 중분류 |  | Y | 2 |  |
| class_name | 중분류명 |  | Y | 20 |  |
| source | 자료원 |  | Y | 20 |  |
| nation_cd | 국가코드 |  | Y | 2 |  |
| exchange_cd | 거래소코드 |  | Y | 3 |  |
| symb | 종목코드 |  | Y | 20 |  |
| symb_name | 종목명 |  | Y | 48 |  |
| title | 제목 |  | Y | 128 |  |
