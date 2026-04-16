<!-- endpoint: /uapi/overseas-futureoption/v1/trading/order -->
<!-- category: [해외선물옵션] 주문/계좌 -->
<!-- korean_name: 해외선물옵션 주문 -->

# 해외선물옵션 주문 [v1_해외선물-001]

## Info
- **Method**: POST
- **URL**: /uapi/overseas-futureoption/v1/trading/order
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: OTFM3001U
- **모의TRID**: 모의투자 미지원
- **Format**: JSON
- **Content-Type**: application/json; charset=UTF-8

## 개요
해외선물옵션 주문 API 입니다.
※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
(EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용)※ 토큰 지정시 토큰 타입("Bearer") 지정 필요. 즉, 발급받은 접근토큰 앞에 앞에 "Bearer" 붙여서 호출EX) "Bearer eyJ..........8GA" |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자]OTFM3001U : ASFM선물옵션주문신규 |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| OVRS_FUTR_FX_PDNO | 해외선물FX상품번호 | String | Y | 32 |  |
| SLL_BUY_DVSN_CD | 매도매수구분코드 | String | Y | 2 | 01 : 매도02 : 매수 |
| FM_LQD_USTL_CCLD_DT | FM청산미결제체결일자 | String | N | 8 | 빈칸 (hedge청산만 이용) |
| FM_LQD_USTL_CCNO | FM청산미결제체결번호 | String | N | 10 | 빈칸 (hedge청산만 이용) |
| PRIC_DVSN_CD | 가격구분코드 | String | Y | 1 | 1.지정, 2. 시장, 3. STOP, 4 S/L |
| FM_LIMIT_ORD_PRIC | FMLIMIT주문가격 | String | Y | 20 | 지정가인 경우 가격 입력* 시장가, STOP주문인 경우, 빈칸("") 입력 |
| FM_STOP_ORD_PRIC | FMSTOP주문가격 | String | Y | 20 | STOP 주문 가격 입력* 시장가, 지정가인 경우, 빈칸("") 입력 |
| FM_ORD_QTY | FM주문수량 | String | Y | 10 |  |
| FM_LQD_LMT_ORD_PRIC | FM청산LIMIT주문가격 | String | N | 20 | 빈칸 (hedge청산만 이용) |
| FM_LQD_STOP_ORD_PRIC | FM청산STOP주문가격 | String | N | 20 | 빈칸 (hedge청산만 이용) |
| CCLD_CNDT_CD | 체결조건코드 | String | Y | 1 | 일반적으로 6 (EOD, 지정가) GTD인 경우 5, 시장가인 경우만 2 |
| CPLX_ORD_DVSN_CD | 복합주문구분코드 | String | Y | 1 | 0 (hedge청산만 이용) |
| ECIS_RSVN_ORD_YN | 행사예약주문여부 | String | Y | 1 | N |
| FM_HDGE_ORD_SCRN_YN | FM_HEDGE주문화면여부 | String | Y | 1 | N |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 |  |
| msg1 | 응답메세지 | String | Y | 80 |  |
| output |  | Object | N |  |  |
| ORD_DT | 주문일자 | String | N | 8 |  |
| ODNO | 주문번호 | String | N | 8 | 접수한 주문의 일련번호(ex. 00360686)* 정정/취소시 문자열처럼 "0"을 포함해서 전송 (ex. ORGN_ODNO : 00360686) |
