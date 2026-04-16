<!-- endpoint: /uapi/overseas-stock/v1/trading/inquire-ccnl -->
<!-- category: [해외주식] 주문/계좌 -->
<!-- korean_name: 해외주식 주문체결내역 -->

# 해외주식 주문체결내역[v1_해외주식-007]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-stock/v1/trading/inquire-ccnl
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: TTTS3035R
- **모의TRID**: VTTS3035R
- **Format**: JSON
- **Content-Type**: application/json; charset=utf-8

## 개요
일정 기간의 해외주식 주문 체결 내역을 확인하는 API입니다.
실전계좌의 경우, 한 번의 호출에 최대 20건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
모의계좌의 경우, 한 번의 호출에 최대 15건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
* 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
* 해외 거래소 운영시간(한국시간 기준)
1) 미국 : 23:30 ~ 06:00 (썸머타임 적용 시 22:30 ~ 05:00)
* 프리마켓(18:00 ~ 23:30, Summer Time : 17:00 ~ 22:30), 애프터마켓(06:00 ~ 07:00, Summer Time : 05:00 ~ 07:00)
2) 일본 : (오전) 09:00 ~ 11:30, (오후) 12:30 ~ 15:00
3) 상해 : 10:30 ~ 16:00
4) 홍콩 : (오전) 10:30 ~ 13:00, (오후) 14:00 ~ 17:00

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용)법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용)※ 토큰 지정시 토큰 타입("Bearer") 지정 필요. 즉, 발급받은 접근토큰 앞에 앞에 "Bearer" 붙여서 호출EX) "Bearer eyJ..........8GA" |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자]TTTS3035R[모의투자]VTTS3035R |
| tr_cont | 연속 거래 여부 | String | N | 1 | 공백 : 초기 조회N : 다음 데이터 조회 (output header의 tr_cont가 M일 경우) |
| custtype | 고객타입 | String | N | 1 | B : 법인P : 개인 |
| seq_no | 일련번호 | String | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 | String | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 | String | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP | String | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID | String | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CANO | 종합계좌번호 | String | Y | 8 | 계좌번호 체계(8-2)의 앞 8자리 |
| ACNT_PRDT_CD | 계좌상품코드 | String | Y | 2 | 계좌번호 체계(8-2)의 뒤 2자리 |
| PDNO | 상품번호 | String | Y | 12 | 전종목일 경우 "%" 입력※ 모의투자계좌의 경우 ""(전체 조회)만 가능 |
| ORD_STRT_DT | 주문시작일자 | String | Y | 8 | YYYYMMDD 형식 (현지시각 기준) |
| ORD_END_DT | 주문종료일자 | String | Y | 8 | YYYYMMDD 형식 (현지시각 기준) |
| SLL_BUY_DVSN | 매도매수구분 | String | Y | 2 | 00 : 전체 01 : 매도 02 : 매수※ 모의투자계좌의 경우 "00"(전체 조회)만 가능 |
| CCLD_NCCS_DVSN | 체결미체결구분 | String | Y | 2 | 00 : 전체 01 : 체결 02 : 미체결※ 모의투자계좌의 경우 "00"(전체 조회)만 가능 |
| OVRS_EXCG_CD | 해외거래소코드 | String | Y | 4 | 전종목일 경우 "%" 입력NASD : 미국시장 전체(나스닥, 뉴욕, 아멕스)NYSE : 뉴욕AMEX : 아멕스SEHK : 홍콩 SHAA : 중국상해SZAA : 중국심천TKSE : 일본HASE : 베트남 하노이VNSE : 베트남 호치민※ 모의투자계좌의 경우 ""(전체 조회)만 가능 |
| SORT_SQN | 정렬순서 | String | Y | 2 | DS : 정순AS : 역순 ※ 모의투자계좌의 경우 정렬순서 사용불가(Default : DS(정순)) |
| ORD_DT | 주문일자 | String | Y | 8 | "" (Null 값 설정) |
| ORD_GNO_BRNO | 주문채번지점번호 | String | Y | 5 | "" (Null 값 설정) |
| ODNO | 주문번호 | String | Y | 10 | "" (Null 값 설정)※ 주문번호로 검색 불가능합니다. 반드시 ""(Null 값 설정) 바랍니다. |
| CTX_AREA_NK200 | 연속조회키200 | String | Y | 200 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터) |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | Y | 200 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터) |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | Y | 1 | F or M : 다음 데이터 있음D or E : 마지막 데이터 |
| gt_uid | Global UID | String | Y | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 | String | Y | 1 | 0 : 성공 0 이외의 값 : 실패 |
| msg_cd | 응답코드 | String | Y | 8 | 응답코드 |
| msg1 | 응답메세지 | String | Y | 80 | 응답메세지 |
| ctx_area_fk200 | 연속조회검색조건200 | String | Y | 200 |  |
| ctx_area_nk200 | 연속조회키200 | String | Y | 200 |  |
| output | 응답상세 | Array | Y | - |  |
| ord_dt | 주문일자 | String | Y | 8 | 주문접수 일자 (현지시각 기준) |
| ord_gno_brno | 주문채번지점번호 | String | Y | 5 | 계좌 개설 시 관리점으로 선택한 영업점의 고유번호 |
| odno | 주문번호 | String | Y | 10 | 접수한 주문의 일련번호※ 정정취소주문 시, 해당 값 odno(주문번호) 넣어서 사용 |
| orgn_odno | 원주문번호 | String | Y | 10 | 정정 또는 취소 대상 주문의 일련번호 |
| sll_buy_dvsn_cd | 매도매수구분코드 | String | Y | 2 | 01 : 매도 02 : 매수 |
| sll_buy_dvsn_cd_name | 매도매수구분코드명 | String | Y | 60 |  |
| rvse_cncl_dvsn | 정정취소구분 | String | Y | 2 | 01 : 정정 02 : 취소 |
| rvse_cncl_dvsn_name | 정정취소구분명 | String | Y | 60 |  |
| pdno | 상품번호 | String | Y | 12 |  |
| prdt_name | 상품명 | String | Y | 60 |  |
| ft_ord_qty | FT주문수량 | String | Y | 10 | 주문수량 |
| ft_ord_unpr3 | FT주문단가3 | String | Y | 26 | 주문가격 |
| ft_ccld_qty | FT체결수량 | String | Y | 10 | 체결된 수량 |
| ft_ccld_unpr3 | FT체결단가3 | String | Y | 26 | 체결된 가격 |
| ft_ccld_amt3 | FT체결금액3 | String | Y | 23 | 체결된 금액 |
| nccs_qty | 미체결수량 | String | Y | 10 | 미체결수량 |
| prcs_stat_name | 처리상태명 | String | Y | 60 | 완료, 거부, 전송 |
| rjct_rson | 거부사유 | String | Y | 60 | 정상 처리되지 못하고 거부된 주문의 사유 |
| rjct_rson_name | 거부사유명 | String | Y | 60 |  |
| ord_tmd | 주문시각 | String | Y | 6 | 주문 접수 시간 |
| tr_mket_name | 거래시장명 | String | Y | 60 |  |
| tr_natn | 거래국가 | String | Y | 3 |  |
| tr_natn_name | 거래국가명 | String | Y | 3 |  |
| ovrs_excg_cd | 해외거래소코드 | String | Y | 4 | NASD : 나스닥NYSE : 뉴욕AMEX : 아멕스SEHK : 홍콩 SHAA : 중국상해SZAA : 중국심천TKSE : 일본HASE : 베트남 하노이VNSE : 베트남 호치민 |
| tr_crcy_cd | 거래통화코드 | String | Y | 60 |  |
| dmst_ord_dt | 국내주문일자 | String | Y | 8 |  |
| thco_ord_tmd | 당사주문시각 | String | Y | 6 |  |
| loan_type_cd | 대출유형코드 | String | Y | 2 | 00 : 해당사항없음01 : 자기융자일반형03 : 자기융자투자형05 : 유통융자일반형06 : 유통융자투자형07 : 자기대주09 : 유통대주10 : 현금11 : 주식담보대출12 : 수익증권담보대출13 : ELS담보대출14 : 채권담보대출15 : 해외주식담보대출16 : 기업신용공여31 : 소액자동담보대출41 : 매도담보대출42 : 환매자금대출43 : 매입환매자금대출44 : 대여매도담보대출81 : 대차거래82 : 법인CMA론91 : 공모주청약자금대출92 : 매입자금93 : 미수론서비스94 : 대여 |
| loan_dt | 대출일자 | String | Y | 8 |  |
| mdia_dvsn_name | 매체구분명 | String | Y | 60 | ex) OpenAPI, 모바일 |
| usa_amk_exts_rqst_yn | 미국애프터마켓연장신청여부 | String | Y | 1 | Y/N |
| splt_buy_attr_name | 분할매수/매도속성명 | String | Y | 60 | 정규장 종료 주문 시에는 '정규장 종료', 시간 입력 시에는 from ~ to 시간 표시 |
