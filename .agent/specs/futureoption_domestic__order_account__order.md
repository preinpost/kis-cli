<!-- endpoint: /uapi/domestic-futureoption/v1/trading/order -->
<!-- category: [국내선물옵션] 주문/계좌 -->
<!-- korean_name: 선물옵션 주문 -->

# 선물옵션 주문[v1_국내선물-001]

## Info
- **Method**: POST
- **URL**: /uapi/domestic-futureoption/v1/trading/order
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: (주간 매수/매도) TTTO1101U (야간 매수/매도) (구) JTCE1001U (신) STTN1101U
- **모의TRID**: (주간 매수/매도) VTTO1101U (야간은 모의투자 미제공)
- **Format**: JSON
- **Content-Type**: application/json; charset=UTF-8

## 개요
​선물옵션 주문 API입니다.
* 선물옵션 운영시간 외 API 호출 시 애러가 발생하오니 운영시간을 확인해주세요.
※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
(EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용)※ 토큰 지정시 토큰 타입("Bearer") 지정 필요. 즉, 발급받은 접근토큰 앞에 앞에 "Bearer" 붙여서 호출EX) "Bearer eyJ..........8GA" |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자]TTTO1101U : 선물 옵션 매수 매도 주문 주간 (신) STTN1101U : 선물 옵션 매수 매도 주문 야간 [모의투자]VTTO1101U : 선물 옵션 매수 매도 주문 주간 |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객타입 | String | N | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| ORD_PRCS_DVSN_CD | 주문처리구분코드 | String | Y | 2 | 02 : 주문전송 |
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| SLL_BUY_DVSN_CD | 매도매수구분코드 | String | Y | 2 | 01 : 매도02 : 매수 |
| SHTN_PDNO | 단축상품번호 | String | Y | 12 | 종목번호선물 6자리 (예: A01603)옵션 9자리 (예: B01603955) |
| ORD_QTY | 주문수량 | String | Y | 10 |  |
| UNIT_PRICE | 주문가격1 | String | Y | 23 | 시장가나 최유리 지정가인 경우 0으로 입력 |
| NMPR_TYPE_CD | 호가유형코드 | String | N | 2 | ※ ORD_DVSN_CD(주문구분코드)를 입력한 경우 ""(공란)으로 입력해도 됨01 : 지정가02 : 시장가 03 : 조건부04 : 최유리 |
| KRX_NMPR_CNDT_CD | 한국거래소호가조건코드 | String | N | 1 | ※ ORD_DVSN_CD(주문구분코드)를 입력한 경우 ""(공란)으로 입력해도 됨0 : 없음3 : IOC4 : FOK |
| CTAC_TLNO | 연락전화번호 | String | N | 20 | 고객의 연락 가능한 전화번호 |
| FUOP_ITEM_DVSN_CD | 선물옵션종목구분코드 | String | N | 2 | 공란(Default) |
| ORD_DVSN_CD | 주문구분코드 | String | Y | 2 | 01 : 지정가02 : 시장가03 : 조건부04 : 최유리,10 : 지정가(IOC)11 : 지정가(FOK)12 : 시장가(IOC)13 : 시장가(FOK)14 : 최유리(IOC)15 : 최유리(FOK) |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 | 응답코드 |
| msg1 | 응답메세지 | String | Y | 80 | 응답메세지 |
| output | 응답상세 | Array | Y | - |  |
| ACNT_NAME | 계좌명 | String | Y | 60 | 계좌의 고객명 |
| TRAD_DVSN_NAME | 매매구분명 | String | Y | 60 | 매도/매수 등 구분값 |
| ITEM_NAME | 종목명 | String | Y | 60 | 주문 종목 명칭 |
| ORD_TMD | 주문시각 | String | Y | 6 | 주문 접수 시간 |
| ORD_GNO_BRNO | 주문채번지점번호 | String | Y | 5 | 계좌 개설 시 관리점으로 선택한 영업점의 고유번호 |
| ODNO | 주문번호 | String | Y | 10 | 접수한 주문의 일련번호 |
