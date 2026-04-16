<!-- endpoint: /uapi/overseas-stock/v1/trading/order-resv -->
<!-- category: [해외주식] 주문/계좌 -->
<!-- korean_name: 해외주식 예약주문접수 -->

# 해외주식 예약주문접수[v1_해외주식-002]

## Info
- **Method**: POST
- **URL**: /uapi/overseas-stock/v1/trading/order-resv
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: (미국예약매수) TTTT3014U (미국예약매도) TTTT3016U (중국/홍콩/일본/베트남 예약주문) TTTS3013U
- **모의TRID**: (미국예약매수) VTTT3014U (미국예약매도) VTTT3016U (중국/홍콩/일본/베트남 예약주문) VTTS3013U
- **Content-Type**: application/json; charset=UTF-8

## 개요
미국거래소 운영시간 외 미국주식을 예약 매매하기 위한 API입니다.
* 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
(EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
* 아래 각 국가의 시장별 예약주문 접수 가능 시간을 확인하시길 바랍니다.
미국 예약주문 접수시간
1) 10:00 ~ 23:20 / 10:00 ~ 22:20 (서머타임 시)
2) 주문제한 : 16:30 ~ 16:45 경까지 (사유 : 시스템 정산작업시간)
3) 23:30 정규장으로 주문 전송 (서머타임 시 22:30 정규장 주문 전송)
4) 미국 거래소 운영시간(한국시간 기준) : 23:30 ~ 06:00 (썸머타임 적용 시 22:30 ~ 05:00)
홍콩 예약주문 접수시간
1) 09:00 ~ 10:20 접수, 10:30 주문전송
2) 10:40 ~ 13:50 접수, 14:00 주문전송
중국 예약주문 접수시간
1) 09:00 ~ 10:20 접수, 10:30 주문전송
2) 10:40 ~ 13:50 접수, 14:00 주문전송
일본 예약주문 접수시간
1) 09:10 ~ 12:20 까지 접수, 12:30 주문전송
베트남 예약주문 접수시간
1) 09:00 ~ 11:00 까지 접수, 11:15 주문전송
2) 11:20 ~ 14:50 까지 접수, 15:00 주문전송
* 예약주문 유의사항
1) 예약주문 유효기간 : 당일
- 미국장 마감 후, 미체결주문은 자동취소
- 미국휴장 시, 익 영업일로 이전
(미국예약주문화면에서 취소 가능)
2) 증거금 및 잔고보유 : 체크 안함
3) 주문전송 불가사유
- 매수증거금 부족: 수수료 포함 매수금액부족, 환전, 시세이용료 출금, 인출에 의한 증거금 부족
- 기타 매수증거금 부족, 매도가능수량 부족, 주권변경 등 권리발생으로 인한 주문불가사유 발생
4) 지정가주문만 가능
* 단 미국 예약매도주문(TTTT3016U)의 경우, MOO(장개시시장가)로 주문 접수 가능

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용)법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자]TTTT3016U : 미국 매도 예약 주문TTTT3014U : 미국 매수 예약 주문TTTS3013U : 중국/홍콩/일본/베트남 예약 매수/매도/취소 주문[모의투자]VTTT3016U : 미국 매도 예약 주문VTTT3014U : 미국 매수 예약 주문VTTS3013U : 중국/홍콩/일본/베트남 예약 매수/매도/취소 주문 |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객타입 | String | N | 1 | B : 법인P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| SLL_BUY_DVSN_CD | 매도매수구분코드 | String | N | 2 | tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용01 : 매도02 : 매수 |
| RVSE_CNCL_DVSN_CD | 정정취소구분코드 | String | Y | 2 | tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용00 : "매도/매수 주문"시 필수 항목02 : 취소 |
| PDNO | 상품번호 | String | Y | 12 |  |
| PRDT_TYPE_CD | 상품유형코드 | String | Y | 3 | tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용515 : 일본501 : 홍콩 / 543 : 홍콩CNY / 558 : 홍콩USD507 : 베트남 하노이거래소 / 508 : 베트남 호치민거래소551 : 중국 상해A / 552 : 중국 심천A |
| OVRS_EXCG_CD | 해외거래소코드 | String | Y | 4 | NASD : 나스닥NYSE : 뉴욕AMEX : 아멕스SEHK : 홍콩SHAA : 중국상해SZAA : 중국심천TKSE : 일본HASE : 베트남 하노이VNSE : 베트남 호치민 |
| FT_ORD_QTY | FT주문수량 | String | Y | 10 |  |
| FT_ORD_UNPR3 | FT주문단가3 | String | Y | 27 |  |
| ORD_SVR_DVSN_CD | 주문서버구분코드 | String | N | 1 | "0"(Default) |
| RSVN_ORD_RCIT_DT | 예약주문접수일자 | String | N | 8 | tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용 |
| ORD_DVSN | 주문구분 | String | N | 20 | tr_id가 TTTT3014U(미국 예약 매수 주문)인 경우만 사용00 : 지정가35 : TWAP36 : VWAPtr_id가 TTTT3016U(미국 예약 매도 주문)인 경우만 사용00 : 지정가31 : MOO(장개시시장가)35 : TWAP36 : VWAP |
| OVRS_RSVN_ODNO | 해외예약주문번호 | String | N | 10 | tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용 |
| ALGO_ORD_TMD_DVSN_CD | 알고리즘주문시간구분코드 | String | N | 2 | ※ TWAP, VWAP 주문에서만 사용. 예약주문은 시간입력 불가하여 02로 값 고정※ 정규장 종료 10분전까지 가능 |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공 0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 | 응답코드 |
| msg1 | 응답메세지 | String | Y | 80 | 응답메세지 |
| output | 응답상세 | Object | Y | - |  |
| ODNO | 한국거래소전송주문조직번호 | String | Y | 10 | tr_id가 TTTT3016U(미국 예약 매도 주문) / TTTT3014U(미국 예약 매수 주문)인 경우만 출력 |
| RSVN_ORD_RCIT_DT | 예약주문접수일자 | String | Y | 8 | tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 출력 |
| OVRS_RSVN_ODNO | 해외예약주문번호 | String | Y | 10 | tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 출력 |
