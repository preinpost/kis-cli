<!-- endpoint: /uapi/elw/v1/quotations/expiration-stocks -->
<!-- category: [국내주식] ELW 시세 -->
<!-- korean_name: ELW 만기예정/만기종목 -->

# ELW 만기예정/만기종목 [국내주식-184]

## Info
- **Method**: GET
- **URL**: /uapi/elw/v1/quotations/expiration-stocks
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: FHKEW154700C0
- **모의TRID**: 모의투자 미지원

## 개요
ELW 만기예정/만기종목 API입니다.
한국투자 HTS(eFriend Plus) > [0290] ELW 만기예정/만기종목 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
최근 100건까지 데이터 조회 가능합니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHKEW154700C0 |
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
| FID_COND_MRKT_DIV_CODE | 조건시장분류코드 | String | Y | 2 | W 입력 |
| FID_COND_SCR_DIV_CODE | 조건화면분류코드 | String | Y | 5 | 11547 입력 |
| FID_INPUT_DATE_1 | 입력날짜1 | String | Y | 10 | 입력날짜 ~ (ex) 20240402) |
| FID_INPUT_DATE_2 | 입력날짜2 | String | Y | 10 | ~입력날짜 (ex) 20240408) |
| FID_DIV_CLS_CODE | 분류구분코드 | String | Y | 2 | 0(콜),1(풋),2(전체) |
| FID_ETC_CLS_CODE | 기타구분코드 | String | Y | 2 | 공백 입력 |
| FID_UNAS_INPUT_ISCD | 기초자산입력종목코드 | String | Y | 12 | 000000(전체), 2001(KOSPI 200), 기초자산코드(종목코드 ex. 삼성전자-005930) |
| FID_INPUT_ISCD_2 | 발행회사코드 | String | Y | 8 | 00000(전체), 00003(한국투자증권), 00017(KB증권), 00005(미래에셋증권) |
| FID_BLNG_CLS_CODE | 결제방법 | String | Y | 2 | 0(전체),1(일반),2(조기종료) |
| FID_INPUT_OPTION_1 | 입력옵션1 | String | Y | 10 | 공백 입력 |

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
| output1 | 응답상세 | Object Array | Y |  | array |
| elw_shrn_iscd | ELW단축종목코드 | String | Y | 9 |  |
| elw_kor_isnm | ELW한글종목명 | String | Y | 40 |  |
| unas_isnm | 기초자산종목명 | String | Y | 40 |  |
| unas_prpr | 기초자산현재가 | String | Y | 112 |  |
| acpr | 행사가 | String | Y | 112 |  |
| stck_cnvr_rate | 주식전환비율 | String | Y | 136 |  |
| elw_prpr | ELW현재가 | String | Y | 10 |  |
| stck_lstn_date | 주식상장일자 | String | Y | 8 |  |
| stck_last_tr_date | 주식최종거래일자 | String | Y | 8 |  |
| total_rdmp_amt | 총상환금액 | String | Y | 18 |  |
| rdmp_amt | 상환금액 | String | Y | 186 |  |
| lstn_stcn | 상장주수 | String | Y | 18 |  |
| lp_hvol | LP보유량 | String | Y | 18 |  |
| ccls_paym_prc | 확정지급2가격 | String | Y | 223 |  |
| mtrt_vltn_amt | 만기평가금액 | String | Y | 192 |  |
| evnt_prd_fin_date | 행사2기간종료일자 | String | Y | 8 |  |
| stlm_date | 결제일자 | String | Y | 8 |  |
| pblc_prc | 발행가격 | String | Y | 18 |  |
| unas_shrn_iscd | 기초자산단축종목코드 | String | Y | 9 |  |
| stnd_iscd | 표준종목코드 | String | Y | 12 |  |
| rdmp_ask_amt | 상환청구금액 | String | Y | 18 |  |
