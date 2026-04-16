<!-- endpoint: /uapi/domestic-stock/v1/quotations/frgnmem-trade-estimate -->
<!-- category: [국내주식] 시세분석 -->
<!-- korean_name: 외국계 매매종목 가집계 -->

# 외국계 매매종목 가집계 [국내주식-161]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/frgnmem-trade-estimate
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: FHKST644100C0
- **모의TRID**: 모의투자 미지원

## 개요
외국계 매매종목 가집계 API입니다.
한국투자 HTS(eFriend Plus) > [0430] 외국계 매매종목 가집계 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHKST644100C0 |
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
| FID_COND_MRKT_DIV_CODE | 조건시장분류코드 | String | Y | 2 | 시장구분코드 (J) |
| FID_COND_SCR_DIV_CODE | 조건화면분류코드 | String | Y | 5 | Uniquekey (16441) |
| FID_INPUT_ISCD | 입력종목코드 | String | Y | 12 | 0000(전체), 1001(코스피), 2001(코스닥) |
| FID_RANK_SORT_CLS_CODE | 순위정렬구분코드 | String | Y | 2 | 0(금액순), 1(수량순) |
| FID_RANK_SORT_CLS_CODE_2 | 순위정렬구분코드2 | String | Y | 2 | 0(매수순), 1(매도순) |

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
| output | 응답상세 | Object Array | Y |  | array |
| stck_shrn_iscd | 주식단축종목코드 | String | Y | 9 |  |
| hts_kor_isnm | HTS한글종목명 | String | Y | 40 |  |
| glob_ntsl_qty | 외국계순매도수량 | String | Y | 12 |  |
| stck_prpr | 주식현재가 | String | Y | 10 |  |
| prdy_vrss | 전일대비 | String | Y | 10 |  |
| prdy_vrss_sign | 전일대비부호 | String | Y | 1 |  |
| prdy_ctrt | 전일대비율 | String | Y | 82 |  |
| acml_vol | 누적거래량 | String | Y | 18 |  |
| glob_total_seln_qty | 외국계총매도수량 | String | Y | 18 |  |
| glob_total_shnu_qty | 외국계총매수2수량 | String | Y | 18 |  |
