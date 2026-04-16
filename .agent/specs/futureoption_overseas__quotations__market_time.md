<!-- endpoint: /uapi/overseas-futureoption/v1/quotations/market-time -->
<!-- category: [해외선물옵션] 기본시세 -->
<!-- korean_name: 해외선물옵션 장운영시간 -->

# 해외선물옵션 장운영시간 [해외선물-030]

## Info
- **Method**: GET
- **URL**: /uapi/overseas-futureoption/v1/quotations/market-time
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: OTFM2229R
- **모의TRID**: 모의투자 미지원

## 개요
해외선물 장운영시간 API입니다.
한국투자 HTS(eFriend Plus) > [6773] 해외선물 장운영시간 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | OTFM2229R |
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
| FM_PDGR_CD | FM상품군코드 | String | Y | 10 | 공백 |
| FM_CLAS_CD | FM클래스코드 | String | Y | 3 | '공백(전체), 001(통화), 002(금리), 003(지수),004(농산물),005(축산물),006(금속),007(에너지)' |
| FM_EXCG_CD | FM거래소코드 | String | Y | 10 | 'CME(CME), EUREX(EUREX), HKEx(HKEx),ICE(ICE), SGX(SGX), OSE(OSE), ASX(ASX),CBOE(CBOE), MDEX(MDEX), NYSE(NYSE),BMF(BMF),FTX(FTX), HNX(HNX), ETC(기타)' |
| OPT_YN | 옵션여부 | String | Y | 1 | %(전체), N(선물), Y(옵션) |
| CTX_AREA_NK200 | 연속조회키200 | String | Y | 200 |  |
| CTX_AREA_FK200 | 연속조회검색조건200 | String | Y | 200 |  |

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
| output | 응답상세 | Object Array | Y |  |  |
| fm_pdgr_cd | FM상품군코드 | String | Y | 10 |  |
| fm_pdgr_name | FM상품군명 | String | Y | 60 |  |
| fm_excg_cd | FM거래소코드 | String | Y | 10 |  |
| fm_excg_name | FM거래소명 | String | Y | 60 |  |
| fuop_dvsn_name | 선물옵션구분명 | String | Y | 60 |  |
| fm_clas_cd | FM클래스코드 | String | Y | 3 |  |
| fm_clas_name | FM클래스명 | String | Y | 30 |  |
| am_mkmn_strt_tmd | 오전장운영시작시각 | String | Y | 6 |  |
| am_mkmn_end_tmd | 오전장운영종료시각 | String | Y | 6 |  |
| pm_mkmn_strt_tmd | 오후장운영시작시각 | String | Y | 6 |  |
| pm_mkmn_end_tmd | 오후장운영종료시각 | String | Y | 6 |  |
| mkmn_nxdy_strt_tmd | 장운영익일시작시각 | String | Y | 6 |  |
| mkmn_nxdy_end_tmd | 장운영익일종료시각 | String | Y | 6 |  |
| base_mket_strt_tmd | 기본시장시작시각 | String | Y | 6 |  |
| base_mket_end_tmd | 기본시장종료시각 | String | Y | 6 |  |
