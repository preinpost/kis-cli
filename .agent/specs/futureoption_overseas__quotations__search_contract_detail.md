<!-- endpoint: /uapi/overseas-futureoption/v1/quotations/search-contract-detail -->
<!-- category: [해외선물옵션] 기본시세 -->
<!-- korean_name: 해외선물 상품기본정보 -->

# 해외선물 상품기본정보 [해외선물-023]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-futureoption/v1/quotations/search-contract-detail
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: HHDFC55200000
- **모의TRID**: 모의투자 미지원

## 개요
해외선물옵션 상품기본정보 API입니다.
한국투자 HTS(eFriend Plus) > [0054] 해외선물옵션 상품기본정보 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
QRY_CNT에 SRS_CD 요청 개수 입력, SRS_CD_01 ~SRS_CD_32 까지 최대 32건의 상품코드 추가 입력하여 해외선물옵션 상품기본정보 확인이 가능합니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | HHDFC55200000 |
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
| QRY_CNT | 요청개수 | String | Y | 4 | 입력한 코드 개수 |
| SRS_CD_01 | 품목종류 | String | Y | 32 | 최대 32개 까지 가능 |
| SRS_CD_02… | 품목종류… | String | Y | 32 |  |
| SRS_CD_32 | 품목종류 | String | Y | 32 |  |

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
| exch_cd | 거래소코드 | String | Y | 10 |  |
| clas_cd | 품목종류 | String | Y | 3 |  |
| crc_cd | 거래통화 | String | Y | 10 |  |
| sttl_price | 정산가 | String | Y | 15 |  |
| sttl_date | 정산일 | String | Y | 8 |  |
| trst_mgn | 증거금 | String | Y | 19 |  |
| disp_digit | 가격표시진법 | String | Y | 10 |  |
| tick_sz | 틱사이즈 | String | Y | 19 |  |
| tick_val | 틱가치 | String | Y | 19 |  |
| mrkt_open_date | 장개시일자 | String | Y | 8 |  |
| mrkt_open_time | 장개시시각 | String | Y | 6 |  |
| mrkt_close_date | 장마감일자 | String | Y | 8 |  |
| mrkt_close_time | 장마감시각 | String | Y | 6 |  |
| trd_fr_date | 상장일 | String | Y | 8 |  |
| expr_date | 만기일 | String | Y | 8 |  |
| trd_to_date | 최종거래일 | String | Y | 8 |  |
| remn_cnt | 잔존일수 | String | Y | 4 |  |
| stat_tp | 매매여부 | String | Y | 1 |  |
| ctrt_size | 계약크기 | String | Y | 19 |  |
| stl_tp | 최종결제구분 | String | Y | 20 |  |
| frst_noti_date | 최초식별일 | String | Y | 8 |  |
| sub_exch_nm | 서브거래소코드 | String | Y | 32 |  |
