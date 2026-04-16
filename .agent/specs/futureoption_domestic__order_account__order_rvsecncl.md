<!-- endpoint: /uapi/domestic-futureoption/v1/trading/order-rvsecncl -->
<!-- category: [국내선물옵션] 주문/계좌 -->
<!-- korean_name: 선물옵션 정정취소주문 -->

# 선물옵션 정정취소주문[v1_국내선물-002]

## Info
- **Method**: POST
- **URL**: /uapi/domestic-futureoption/v1/trading/order-rvsecncl
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: (주간 정정/취소) TTTO1103U (야간 정정/취소) (구) JTCE1002U (신) STTN1103U
- **모의TRID**: (주간 정정/취소) VTTO1103U (야간은 모의투자 미제공)
- **Format**: JSON
- **Content-Type**: application/json; charset=UTF-8

## 개요
선물옵션 주문 건에 대하여 정정 및 취소하는 API입니다. 단, 이미 체결된 건은 정정 및 취소가 불가합니다.
※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
(EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용)※ 토큰 지정시 토큰 타입("Bearer") 지정 필요. 즉, 발급받은 접근토큰 앞에 앞에 "Bearer" 붙여서 호출EX) "Bearer eyJ..........8GA" |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자]TTTO1103U : 선물 옵션 정정 취소 주문 주간(신) STTN1103U : 선물 옵션 정정 취소 주문 야간 [모의투자]VTTO1103U : 선물 옵션 정정 취소 주문 주간 |
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
| RVSE_CNCL_DVSN_CD | 정정취소구분코드 | String | Y | 2 | 01 : 정정02 : 취소 |
| ORGN_ODNO | 원주문번호 | String | Y | 10 | 정정 혹은 취소할 주문의 번호 |
| ORD_QTY | 주문수량 | String | Y | 10 | [Header tr_id TTTO1103U(선물옵션 정정취소 주간)]전량일경우 0으로 입력[Header tr_id JTCE1002U(선물옵션 정정취소 야간)]일부수량 정정 및 취소 불가, 주문수량 반드시 입력 (공백 불가)일부 미체결 시 잔량 전체에 대해서 취소 가능EX) 2개 매수주문 후 1개 체결, 1개 미체결인 상태에서 취소주문 시 ORD_QTY는 1로 입력※ 모의계좌의 경우, 주문수량 반드시 입력 (공백 불가) |
| UNIT_PRICE | 주문가격1 | String | Y | 23 | 시장가나 최유리의 경우 0으로 입력 (취소 시에도 0 입력) |
| NMPR_TYPE_CD | 호가유형코드 | String | Y | 2 | 01 : 지정가02 : 시장가03 : 조건부04 : 최유리 |
| KRX_NMPR_CNDT_CD | 한국거래소호가조건코드 | String | Y | 1 | 취소시 0으로 입력정정시0 : 없음3 : IOC4 : FOK |
| RMN_QTY_YN | 잔여수량여부 | String | Y | 1 | Y : 전량N : 일부 |
| FUOP_ITEM_DVSN_CD | 선물옵션종목구분코드 | String | N | 2 | [Header tr_id TTTO1103U(선물옵션 정정취소 주간)]공란(Default)[Header tr_id JTCE1002U(선물옵션 정정취소 야간)]01 : 선물02 : 콜옵션03 : 풋옵션04 : 스프레드 |
| ORD_DVSN_CD | 주문구분코드 | String | Y | 2 | [정정]01 : 지정가02 : 시장가03 : 조건부04 : 최유리,10 : 지정가(IOC)11 : 지정가(FOK)12 : 시장가(IOC)13 : 시장가(FOK)14 : 최유리(IOC)15 : 최유리(FOK)[취소]01 로 입력 |

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
| ORGN_ODNO | 원주문번호 | String | Y | 10 | 정정 또는 취소 대상 주문의 일련번호 |
| ODNO | 주문번호 | String | Y | 10 | 접수한 주문(정정 또는 취소)의 일련번호 |
