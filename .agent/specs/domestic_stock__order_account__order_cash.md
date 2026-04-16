<!-- endpoint: /uapi/domestic-stock/v1/trading/order-cash -->
<!-- category: [국내주식] 주문/계좌 -->
<!-- korean_name: 주식주문(현금) -->

# 주식주문(현금)[v1_국내주식-001]

## Info
- **Method**: POST
- **URL**: /uapi/domestic-stock/v1/trading/order-cash
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: (매도) TTTC0011U (매수) TTTC0012U
- **모의TRID**: (매도) VTTC0011U (매수) VTTC0012U
- **Format**: JSON
- **Content-Type**: application/json; charset=UTF-8

## 개요
국내주식주문(현금) API 입니다.
※ TTC0802U(현금매수) 사용하셔서 미수매수 가능합니다. 단, 거래하시는 계좌가 증거금40%계좌로 신청이 되어있어야 가능합니다.
※ 신용매수는 별도의 API가 준비되어 있습니다.
※ ORD_QTY(주문수량), ORD_UNPR(주문단가) 등을 String으로 전달해야 함에 유의 부탁드립니다.
※ ORD_UNPR(주문단가)가 없는 주문은 상한가로 주문금액을 선정하고 이후 체결이되면 체결금액로 정산됩니다.
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
| tr_id | 거래ID | String | Y | 13 | '※ 구TR은 사전고지 없이 막힐 수 있으므로 반드시 신TR로 변경이용 부탁드립니다.[실전투자]국내주식주문 매도 : (구)TTTC0801U → (신)TTTC0011U국내주식주문 매도(모의투자) : (구)VTTC0801U → (신)VTTC0011U국내주식주문 매수 : (구)TTTC0802U → (신)TTTC0012U국내주식주문 매수(모의투자) : (구)VTTC0802U → (신)VTTC0012U' |
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
| CANO | 종합계좌번호 | String | Y | 8 | 종합계좌번호 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 상품유형코드 |
| PDNO | 상품번호 | String | Y | 12 | 종목코드(6자리) , ETN의 경우 7자리 입력 |
| SLL_TYPE | 매도유형 (매도주문 시) | String | N | 2 | 01@일반매도02@임의매매05@대차매도→ 미입력시 01 일반매도로 진행 |
| ORD_DVSN | 주문구분 | String | Y | 2 | [KRX]00 : 지정가01 : 시장가02 : 조건부지정가03 : 최유리지정가04 : 최우선지정가05 : 장전 시간외06 : 장후 시간외07 : 시간외 단일가11 : IOC지정가 (즉시체결,잔량취소)12 : FOK지정가 (즉시체결,전량취소)13 : IOC시장가 (즉시체결,잔량취소)14 : FOK시장가 (즉시체결,전량취소)15 : IOC최유리 (즉시체결,잔량취소)16 : FOK최유리 (즉시체결,전량취소)21 : 중간가22 : 스톱지정가23 : 중간가IOC24 : 중간가FOK[NXT]00 : 지정가03 : 최유리지정가04 : 최우선지정가11 : IOC지정가 (즉시체결,잔량취소)12 : FOK지정가 (즉시체결,전량취소)13 : IOC시장가 (즉시체결,잔량취소)14 : FOK시장가 (즉시체결,전량취소)15 : IOC최유리 (즉시체결,잔량취소)16 : FOK최유리 (즉시체결,전량취소)21 : 중간가22 : 스톱지정가23 : 중간가IOC24 : 중간가FOK[SOR]00 : 지정가01 : 시장가03 : 최유리지정가04 : 최우선지정가11 : IOC지정가 (즉시체결,잔량취소)12 : FOK지정가 (즉시체결,전량취소)13 : IOC시장가 (즉시체결,잔량취소)14 : FOK시장가 (즉시체결,전량취소)15 : IOC최유리 (즉시체결,잔량취소)16 : FOK최유리 (즉시체결,전량취소) |
| ORD_QTY | 주문수량 | String | Y | 10 | 주문수량 |
| ORD_UNPR | 주문단가 | String | Y | 19 | 주문단가시장가 등 주문시, "0"으로 입력 |
| CNDT_PRIC | 조건가격 | String | N | 19 | 스탑지정가호가 주문 (ORD_DVSN이 22) 사용 시에만 필수 |
| EXCG_ID_DVSN_CD | 거래소ID구분코드 | String | N | 3 | 한국거래소 : KRX대체거래소 (넥스트레이드) : NXTSOR (Smart Order Routing) : SOR→ 미입력시 KRX로 진행되며, 모의투자는 KRX만 가능 |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 |  |
| msg_cd | 응답코드 | String | Y | 8 |  |
| msg1 | 응답메세지 | String | Y | 80 |  |
| output | 응답상세 | Object Array | Y |  | single |
| KRX_FWDG_ORD_ORGNO | 계좌관리점코드 | String | Y | 5 |  |
| ODNO | 주문번호 | String | Y | 10 |  |
| ORD_TMD | 주문시간 | String | Y | 6 |  |
