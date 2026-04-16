<!-- endpoint: /uapi/overseas-stock/v1/trading/inquire-nccs -->
<!-- category: [해외주식] 주문/계좌 -->
<!-- korean_name: 해외주식 미체결내역 -->

# 해외주식 미체결내역[v1_해외주식-005]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-stock/v1/trading/inquire-nccs
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 미지원
- **실전TRID**: TTTS3018R
- **모의TRID**: 모의투자 미지원
- **Format**: JSON

## 개요
접수된 해외주식 주문 중 체결되지 않은 미체결 내역을 조회하는 API입니다.
실전계좌의 경우, 한 번의 호출에 최대 40건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
※ 해외주식 미체결내역 API 모의투자에서는 사용이 불가합니다.
모의투자로 해외주식 미체결내역 확인시에는 해외주식 주문체결내역[v1_해외주식-007] API 조회하셔서 nccs_qty(미체결수량)으로 해외주식 미체결수량을 조회하실 수 있습니다.
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
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용)법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, Oauth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | [실전투자]TTTS3018R |
| tr_cont | 연속 거래 여부 | String | N | 1 | tr_cont를 이용한 다음조회 불가 API |
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
| OVRS_EXCG_CD | 해외거래소코드 | String | Y | 4 | NASD : 나스닥NYSE : 뉴욕 AMEX : 아멕스SEHK : 홍콩SHAA : 중국상해SZAA : 중국심천TKSE : 일본HASE : 베트남 하노이VNSE : 베트남 호치민* NASD 인 경우만 미국전체로 조회되며 나머지 거래소 코드는 해당 거래소만 조회됨* 공백 입력 시 다음조회가 불가능하므로, 반드시 거래소코드 입력해야 함 |
| SORT_SQN | 정렬순서 | String | Y | 2 | DS : 정순그외 : 역순[header tr_id: TTTS3018R]""(공란) |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | Y | 200 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터) |
| CTX_AREA_NK200 | 연속조회키200 | String | Y | 200 | 공란 : 최초 조회시이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터) |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID | String | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 | String | Y | 1 | tr_cont를 이용한 다음조회 불가 API |
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
| ord_dt | 주문일자 | String | Y | 8 | 주문접수 일자 |
| ord_gno_brno | 주문채번지점번호 | String | Y | 5 | 계좌 개설 시 관리점으로 선택한 영업점의 고유번호 |
| odno | 주문번호 | String | Y | 10 | 접수한 주문의 일련번호 |
| orgn_odno | 원주문번호 | String | Y | 10 | 정정 또는 취소 대상 주문의 일련번호 |
| pdno | 상품번호 | String | Y | 12 | 종목코드 |
| prdt_name | 상품명 | String | Y | 60 | 종목명 |
| sll_buy_dvsn_cd | 매도매수구분코드 | String | Y | 2 | 01 : 매도02 : 매수 |
| sll_buy_dvsn_cd_name | 매도매수구분코드명 | String | Y | 60 | 매수매도구분명 |
| rvse_cncl_dvsn_cd | 정정취소구분코드 | String | Y | 2 | 01 : 정정02 : 취소 |
| rvse_cncl_dvsn_cd_name | 정정취소구분코드명 | String | Y | 60 | 정정취소구분명 |
| rjct_rson | 거부사유 | String | Y | 60 | 정상 처리되지 못하고 거부된 주문의 사유 |
| rjct_rson_name | 거부사유명 | String | Y | 60 | 정상 처리되지 못하고 거부된 주문의 사유명 |
| ord_tmd | 주문시각 | String | Y | 6 | 주문 접수 시간 |
| tr_mket_name | 거래시장명 | String | Y | 60 |  |
| tr_crcy_cd | 거래통화코드 | String | Y | 3 | USD : 미국달러HKD : 홍콩달러CNY : 중국위안화JPY : 일본엔화VND : 베트남동 |
| natn_cd | 국가코드 | String | Y | 3 |  |
| natn_kor_name | 국가한글명 | String | Y | 60 |  |
| ft_ord_qty | FT주문수량 | String | Y | 10 | 주문수량 |
| ft_ccld_qty | FT체결수량 | String | Y | 10 | 체결된 수량 |
| nccs_qty | 미체결수량 | String | Y | 10 | 미체결수량 |
| ft_ord_unpr3 | FT주문단가3 | String | Y | 26 | 주문가격 |
| ft_ccld_unpr3 | FT체결단가3 | String | Y | 26 | 체결된 가격 |
| ft_ccld_amt3 | FT체결금액3 | String | Y | 23 | 체결된 금액 |
| ovrs_excg_cd | 해외거래소코드 | String | Y | 4 | NASD : 나스닥NYSE : 뉴욕AMEX : 아멕스SEHK : 홍콩SHAA : 중국상해SZAA : 중국심천TKSE : 일본HASE : 베트남 하노이VNSE : 베트남 호치민 |
| prcs_stat_name | 처리상태명 | String | Y | 60 | "" |
| loan_type_cd | 대출유형코드 | String | Y | 2 | 00 해당사항없음01 자기융자일반형03 자기융자투자형05 유통융자일반형06 유통융자투자형07 자기대주09 유통대주10 현금11 주식담보대출12 수익증권담보대출13 ELS담보대출14 채권담보대출15 해외주식담보대출16 기업신용공여31 소액자동담보대출41 매도담보대출42 환매자금대출43 매입환매자금대출44 대여매도담보대출81 대차거래82 법인CMA론91 공모주청약자금대출92 매입자금93 미수론서비스94 대여 |
| loan_dt | 대출일자 | String | Y | 8 | 대출 실행일자 |
| usa_amk_exts_rqst_yn | 미국애프터마켓연장신청여부 | String | Y | 1 | Y/N |
| splt_buy_attr_name | 분할매수속성명 | String | Y | 60 | 정규장 종료 주문 시에는 '정규장 종료', 시간 입력 시에는 from ~ to 시간 표시됨 |
