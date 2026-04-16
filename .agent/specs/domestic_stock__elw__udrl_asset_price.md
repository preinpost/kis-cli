<!-- endpoint: /uapi/elw/v1/quotations/udrl-asset-price -->
<!-- category: [국내주식] ELW 시세 -->
<!-- korean_name: ELW 기초자산별 종목시세 -->

# ELW 기초자산별 종목시세 [국내주식-186]

## Info
- **Method**: GET
- **URL**: /uapi/elw/v1/quotations/udrl-asset-price
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: FHKEW154101C0
- **모의TRID**: 모의투자 미지원

## 개요
ELW 기초자산별 종목시세 API입니다.
한국투자 HTS(eFriend Plus) > [0288] ELW 기초자산별 ELW 시세 화면의 "우측 기초자산별 종목 리스트" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHKEW154101C0 |
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
| FID_COND_MRKT_DIV_CODE | 조건시장분류코드 | String | Y | 2 | 시장구분(W) |
| FID_COND_SCR_DIV_CODE | 조건화면분류코드 | String | Y | 5 | Uniquekey(11541) |
| FID_MRKT_CLS_CODE | 시장구분코드 | String | Y | 2 | 전체(A),콜(C),풋(P) |
| FID_INPUT_ISCD | 입력종목코드 | String | Y | 12 | '00000(전체), 00003(한국투자증권), 00017(KB증권), 00005(미래에셋주식회사)' |
| FID_UNAS_INPUT_ISCD | 기초자산입력종목코드 | String | Y | 12 |  |
| FID_VOL_CNT | 거래량수 | String | Y | 12 | 전일거래량(정수량미만) |
| FID_TRGT_EXLS_CLS_CODE | 대상제외구분코드 | String | Y | 32 | 거래불가종목제외(0:미체크,1:체크) |
| FID_INPUT_PRICE_1 | 입력가격1 | String | Y | 12 | 가격~원이상 |
| FID_INPUT_PRICE_2 | 입력가격2 | String | Y | 12 | 가격~월이하 |
| FID_INPUT_VOL_1 | 입력거래량1 | String | Y | 18 | 거래량~계약이상 |
| FID_INPUT_VOL_2 | 입력거래량2 | String | Y | 18 | 거래량~계약이하 |
| FID_INPUT_RMNN_DYNU_1 | 입력잔존일수1 | String | Y | 5 | 잔존일(~일이상) |
| FID_INPUT_RMNN_DYNU_2 | 입력잔존일수2 | String | Y | 5 | 잔존일(~일이하) |
| FID_OPTION | 옵션 | String | Y | 5 | 옵션상태(0:없음,1:ATM,2:ITM,3:OTM) |
| FID_INPUT_OPTION_1 | 입력옵션1 | String | Y | 10 |  |
| FID_INPUT_OPTION_2 | 입력옵션2 | String | Y | 10 |  |

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
| elw_shrn_iscd | ELW단축종목코드 | String | Y | 9 |  |
| hts_kor_isnm | HTS한글종목명 | String | Y | 40 |  |
| elw_prpr | ELW현재가 | String | Y | 10 |  |
| prdy_vrss | 전일대비 | String | Y | 10 |  |
| prdy_vrss_sign | 전일대비부호 | String | Y | 1 |  |
| prdy_ctrt | 전일대비율 | String | Y | 82 |  |
| acml_vol | 누적거래량 | String | Y | 18 |  |
| acpr | 행사가 | String | Y | 112 |  |
| prls_qryr_stpr_prc | 손익분기주가가격 | String | Y | 112 |  |
| hts_rmnn_dynu | HTS잔존일수 | String | Y | 5 |  |
| hts_ints_vltl | HTS내재변동성 | String | Y | 114 |  |
| stck_cnvr_rate | 주식전환비율 | String | Y | 136 |  |
| lp_hvol | LP보유량 | String | Y | 18 |  |
| lp_rlim | LP비중 | String | Y | 52 |  |
| lvrg_val | 레버리지값 | String | Y | 114 |  |
| gear | 기어링 | String | Y | 84 |  |
| delta_val | 델타값 | String | Y | 114 |  |
| gama | 감마 | String | Y | 84 |  |
| vega | 베가 | String | Y | 84 |  |
| theta | 세타 | String | Y | 84 |  |
| prls_qryr_rate | 손익분기비율 | String | Y | 84 |  |
| cfp | 자본지지점 | String | Y | 112 |  |
| prit | 패리티 | String | Y | 112 |  |
| invl_val | 내재가치값 | String | Y | 132 |  |
| tmvl_val | 시간가치값 | String | Y | 132 |  |
| hts_thpr | HTS이론가 | String | Y | 112 |  |
| stck_lstn_date | 주식상장일자 | String | Y | 8 |  |
| stck_last_tr_date | 주식최종거래일자 | String | Y | 8 |  |
| lp_ntby_qty | LP순매도량 | String | Y | 18 |  |
