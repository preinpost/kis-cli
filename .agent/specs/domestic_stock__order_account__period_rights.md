<!-- endpoint: /uapi/domestic-stock/v1/trading/period-rights -->
<!-- category: [국내주식] 주문/계좌 -->
<!-- korean_name: 기간별계좌권리현황조회 -->

# 기간별계좌권리현황조회 [국내주식-211]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/trading/period-rights
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: CTRGA011R
- **모의TRID**: 모의투자 미지원

## 개요
기간별계좌권리현황조회 API입니다.
한국투자 HTS(eFriend Plus) > [7344] 권리유형별 현황조회 화면을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | CTRGA011R |
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
| INQR_DVSN | 조회구분 | String | Y | 2 | 03 입력 |
| CUST_RNCNO25 | 고객실명확인번호25 | String | Y | 25 | 공란 |
| HMID | 홈넷ID | String | Y | 8 | 공란 |
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 8자리 입력 (ex.12345678) |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 상품계좌번호 2자리 입력(ex. 01 or 22) |
| INQR_STRT_DT | 조회시작일자 | String | Y | 8 | 조회시작일자(YYYYMMDD) |
| INQR_END_DT | 조회종료일자 | String | Y | 8 | 조회종료일자(YYYYMMDD) |
| RGHT_TYPE_CD | 권리유형코드 | String | Y | 2 | 공란 |
| PDNO | 상품번호 | String | Y | 12 | 공란 |
| PRDT_TYPE_CD | 상품유형코드 | String | Y | 3 | 공란 |
| CTX_AREA_NK100 | 연속조회키100 | String | Y | 100 | 다음조회시 입력 |
| CTX_AREA_FK100 | 연속조회검색조건100 | String | Y | 100 | 다음조회시 입력 |

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
| output1 | 응답상세 | Object Array | Y |  | array |
| acno10 | 계좌번호10 | String | Y | 10 |  |
| rght_type_cd | 권리유형코드 | String | Y | 2 | 1 유상2 무상3 배당4 매수청구5 공개매수6 주주총회7 신주인수권증서8 반대의사9 신주인수권증권11 합병12 회사분할13 주식교환14 액면분할15 액면병합16 종목변경17 감자18 신구주합병21 후합병22 후회사분할23 후주식교환24 후액면분할25 후액면병합26 후종목변경27 후감자28 후신구주합병31 뮤츄얼펀드32 ETF33 선박투자회사34 투융자회사35 해외자원36 부동산신탁(Ritz)37 상장수익증권41 ELW만기42 ELS분배43 DLS분배44 하일드펀드45 ETN51 전환청구52 교환청구53 BW청구54 WRT청구55 채권풋옵션청구56 전환우선주청구57 전환조건부청구58 전자증권일괄입고59 클라우드펀딩일괄입고61 원리금상환62 스트립채권71 WRT소멸72 WRT증권73 DR전환74 배당옵션75 특별배당76 ISINCODE변경77 실권주청약81 해외분배금(청산)82 해외분배금(조기상환)83 해외분배금(상장폐지)84 DR FEE85 SECTION 871M86 종목전환87 재매수88 종목교환89 기타이벤트91 공모주92 청약93 환매99 기타권리사유 |
| bass_dt | 기준일자 | String | Y | 8 |  |
| rght_cblc_type_cd | 권리잔고유형코드 | String | Y | 2 | 1 입고2 출고3 출고입고4 출고입금5 출고출금10 현금입금11 단수주대금입금12 교부금입금13 유상감자대금입금14 지연이자입금15 이자지급16 대주권리금출금17 분할상환18 만기상환19 조기상환20 출금21 입고&입금22 입고&입금&단수주대금입금25 유상환불금입금26 중도상환27 분할합병세금출금 |
| rptt_pdno | 대표상품번호 | String | Y | 12 |  |
| pdno | 상품번호 | String | Y | 12 |  |
| prdt_type_cd | 상품유형코드 | String | Y | 3 |  |
| shtn_pdno | 단축상품번호 | String | Y | 12 |  |
| prdt_name | 상품명 | String | Y | 60 |  |
| cblc_qty | 잔고수량 | String | Y | 19 |  |
| last_alct_qty | 최종배정수량 | String | Y | 19 |  |
| excs_alct_qty | 초과배정수량 | String | Y | 19 |  |
| tot_alct_qty | 총배정수량 | String | Y | 19 |  |
| last_ftsk_qty | 최종단수주수량 | String | Y | 191 |  |
| last_alct_amt | 최종배정금액 | String | Y | 19 |  |
| last_ftsk_chgs | 최종단수주대금 | String | Y | 19 |  |
| rdpt_prca | 상환원금 | String | Y | 19 |  |
| dlay_int_amt | 지연이자금액 | String | Y | 19 |  |
| lstg_dt | 상장일자 | String | Y | 8 |  |
| sbsc_end_dt | 청약종료일자 | String | Y | 8 |  |
| cash_dfrm_dt | 현금지급일자 | String | Y | 8 |  |
| rqst_qty | 신청수량 | String | Y | 19 |  |
| rqst_amt | 신청금액 | String | Y | 19 |  |
| rqst_dt | 신청일자 | String | Y | 8 |  |
| rfnd_dt | 환불일자 | String | Y | 8 |  |
| rfnd_amt | 환불금액 | String | Y | 19 |  |
| lstg_stqt | 상장주수 | String | Y | 19 |  |
| tax_amt | 세금금액 | String | Y | 19 |  |
| sbsc_unpr | 청약단가 | String | Y | 224 |  |
