<!-- endpoint: /uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl -->
<!-- category: [국내주식] 주문/계좌 -->
<!-- korean_name: 주식정정취소가능주문조회 -->

# 주식정정취소가능주문조회[v1_국내주식-004]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: TTTC0084R
- **모의TRID**: 모의투자 미지원
- **Format**: JSON
- **Content-Type**: application/json; charset=utf-8

## 개요
주식정정취소가능주문조회 API입니다. 한 번의 호출에 최대 50건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
※ 주식주문(정정취소) 호출 전에 반드시 주식정정취소가능주문조회 호출을 통해 정정취소가능수량(output > psbl_qty)을 확인하신 후 정정취소주문 내시기 바랍니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | ※ 구TR은 사전고지 없이 막힐 수 있으므로 반드시 신TR로 변경이용 부탁드립니다.[실전투자](구)TTTC8036R → (신)TTTC0084R |
| tr_cont | 연속 거래 여부 | String | N | 1 | 공백 : 초기 조회N : 다음 데이터 조회 (output header의 tr_cont가 M일 경우) |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| CTX_AREA_FK100 | 연속조회검색조건100 | String | Y | 100 | '공란 : 최초 조회시는 이전 조회 Output CTX_AREA_FK100 값 : 다음페이지 조회시(2번째부터)' |
| CTX_AREA_NK100 | 연속조회키100 | String | Y | 100 | '공란 : 최초 조회시 이전 조회 Output CTX_AREA_NK100 값 : 다음페이지 조회시(2번째부터)' |
| INQR_DVSN_1 | 조회구분1 | String | Y | 1 | '0 주문1 종목' |
| INQR_DVSN_2 | 조회구분2 | String | Y | 1 | '0 전체1 매도2 매수' |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | N | 1 | F or M : 다음 데이터 있음D or E : 마지막 데이터 |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 |  |
| msg_cd | 응답코드 | String | Y | 8 |  |
| msg1 | 응답메세지 | String | Y | 80 |  |
| output | 응답상세 | Object Array | Y |  | array |
| ord_gno_brno | 주문채번지점번호 | String | Y | 5 | 주문시 한국투자증권 시스템에서 지정된 영업점코드 |
| odno | 주문번호 | String | Y | 10 | 주문시 한국투자증권 시스템에서 채번된 주문번호 |
| orgn_odno | 원주문번호 | String | Y | 6 | 정정/취소주문 인경우 원주문번호 |
| ord_dvsn_name | 주문구분명 | String | Y | 5 |  |
| pdno | 상품번호 | String | Y | 10 | 종목번호(뒤 6자리만 해당) |
| prdt_name | 상품명 | String | Y | 6 | 종목명 |
| rvse_cncl_dvsn_name | 정정취소구분명 | String | Y | 5 | 정정 또는 취소 여부 표시 |
| ord_qty | 주문수량 | String | Y | 10 |  |
| ord_unpr | 주문단가 | String | Y | 6 | 1주당 주문가격 |
| ord_tmd | 주문시각 | String | Y | 5 | 주문시각(시분초HHMMSS) |
| tot_ccld_qty | 총체결수량 | String | Y | 10 | 주문 수량 중 체결된 수량 |
| tot_ccld_amt | 총체결금액 | String | Y | 6 | 주문금액 중 체결금액 |
| psbl_qty | 가능수량 | String | Y | 5 | 정정/취소 주문 가능 수량 |
| sll_buy_dvsn_cd | 매도매수구분코드 | String | Y | 10 | 01 : 매도 / 02 : 매수 |
| ord_dvsn_cd | 주문구분코드 | String | Y | 6 | [KRX]00 : 지정가01 : 시장가02 : 조건부지정가03 : 최유리지정가04 : 최우선지정가05 : 장전 시간외06 : 장후 시간외07 : 시간외 단일가11 : IOC지정가 (즉시체결,잔량취소)12 : FOK지정가 (즉시체결,전량취소)13 : IOC시장가 (즉시체결,잔량취소)14 : FOK시장가 (즉시체결,전량취소)15 : IOC최유리 (즉시체결,잔량취소)16 : FOK최유리 (즉시체결,전량취소)21 : 중간가22 : 스톱지정가23 : 중간가IOC24 : 중간가FOK[NXT]00 : 지정가03 : 최유리지정가04 : 최우선지정가11 : IOC지정가 (즉시체결,잔량취소)12 : FOK지정가 (즉시체결,전량취소)13 : IOC시장가 (즉시체결,잔량취소)14 : FOK시장가 (즉시체결,전량취소)15 : IOC최유리 (즉시체결,잔량취소)16 : FOK최유리 (즉시체결,전량취소)21 : 중간가22 : 스톱지정가23 : 중간가IOC24 : 중간가FOK[SOR]00 : 지정가01 : 시장가03 : 최유리지정가04 : 최우선지정가11 : IOC지정가 (즉시체결,잔량취소)12 : FOK지정가 (즉시체결,전량취소)13 : IOC시장가 (즉시체결,잔량취소)14 : FOK시장가 (즉시체결,전량취소)15 : IOC최유리 (즉시체결,잔량취소)16 : FOK최유리 (즉시체결,전량취소) |
| mgco_aptm_odno | 운용사지정주문번호 | String | Y | 5 |  |
| excg_dvsn_cd | 거래소구분코드 | String | Y | 2 |  |
| excg_id_dvsn_cd | 거래소ID구분코드 | String | Y | 3 |  |
| excg_id_dvsn_name | 거래소ID구분명 | String | Y | 100 |  |
| stpm_cndt_pric | 스톱지정가조건가격 | String | Y | 9 |  |
| stpm_efct_occr_yn | 스톱지정가효력발생여부 | String | Y | 1 |  |
