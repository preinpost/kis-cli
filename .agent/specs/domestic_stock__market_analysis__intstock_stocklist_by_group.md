<!-- endpoint: /uapi/domestic-stock/v1/quotations/intstock-stocklist-by-group -->
<!-- category: [국내주식] 시세분석 -->
<!-- korean_name: 관심종목 그룹별 종목조회 -->

# 관심종목 그룹별 종목조회 [국내주식-203]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/intstock-stocklist-by-group
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: HHKCM113004C6
- **모의TRID**: 모의투자 미지원

## 개요
관심종목 그룹별 종목조회 API입니다.
한국투자 HTS(eFriend Plus) > [0161] 관심종목 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
① 관심종목 그룹조회 → ② 관심종목 그룹별 종목조회 → ③ 관심종목(멀티종목) 시세조회 순서대로 호출하셔서 관심종목 시세 조회 가능합니다.
※ 한 번의 호출에 최대 30종목의 시세 확인 가능합니다.
한국투자증권 Github 에서 관심종목 복수시세조회 파이썬 샘플코드를 참고하실 수 있습니다.
https://github.com/koreainvestment/open-trading-api/blob/main/rest/get_interest_stocks_price.py

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHKCM113004C6 |
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
| TYPE | 관심종목구분코드 | String | Y | 1 | Unique key(1) |
| USER_ID | 사용자 ID | String | Y | 16 | HTS_ID 입력 |
| DATA_RANK | 데이터 순위 | String | Y | 10 | 공백 |
| INTER_GRP_CODE | 관심 그룹 코드 | String | Y | 3 | 관심그룹 조회 결과의 그룹 값 입력 |
| INTER_GRP_NAME | 관심 그룹 명 | String | Y | 40 | 공백 |
| HTS_KOR_ISNM | HTS 한글 종목명 | String | Y | 40 | 공백 |
| CNTG_CLS_CODE | 체결 구분 코드 | String | Y | 1 | 공백 |
| FID_ETC_CLS_CODE | 기타 구분 코드 | String | Y | 2 | Unique key(4) |

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
| data_rank | 데이터 순위 | String | Y | 10 |  |
| inter_grp_name | 관심 그룹 명 | String | Y | 40 |  |
| output2 | 응답상세 | Object Array | Y |  | array |
| fid_mrkt_cls_code | FID 시장 구분 코드 | String | Y | 2 |  |
| data_rank | 데이터 순위 | String | Y | 10 |  |
| exch_code | 거래소코드 | String | Y | 4 |  |
| jong_code | 종목코드 | String | Y | 16 |  |
| color_code | 생상 코드 | String | Y | 8 |  |
| memo | 메모 | String | Y | 128 |  |
| hts_kor_isnm | HTS 한글 종목명 | String | Y | 40 |  |
| fxdt_ntby_qty | 기준일 순매수 수량 | String | Y | 12 |  |
| cntg_unpr | 체결단가 | String | Y | 11 |  |
| cntg_cls_code | 체결 구분 코드 | String | Y | 1 |  |
