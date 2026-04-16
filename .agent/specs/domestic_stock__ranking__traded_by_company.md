<!-- endpoint: /uapi/domestic-stock/v1/ranking/traded-by-company -->
<!-- category: [국내주식] 순위분석 -->
<!-- korean_name: 국내주식 당사매매종목 상위 -->

# 국내주식 당사매매종목 상위[v1_국내주식-104]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/ranking/traded-by-company
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: FHPST01860000
- **모의TRID**: 모의투자 미지원

## 개요
국내주식 당사매매종목 상위 API입니다.
한국투자 HTS(eFriend Plus) > [0186] 당사매매종목 상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
최대 30건 확인 가능하며, 다음 조회가 불가합니다.
※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 |  | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 |  | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 |  | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 |  | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 |  | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID |  | Y | 13 | FHPST01860000 |
| tr_cont | 연속 거래 여부 |  | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| custtype | 고객 타입 |  | Y | 1 | B : 법인 P : 개인 |
| seq_no | 일련번호 |  | N | 2 | [법인 필수] 001 |
| mac_address | 맥주소 |  | N | 12 | 법인고객 혹은 개인고객의 Mac address 값 |
| phone_number | 핸드폰번호 |  | N | 12 | [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호 ex) 01011112222 (하이픈 등 구분값 제거) |
| ip_addr | 접속 단말 공인 IP |  | N | 12 | [법인 필수] 사용자(회원)의 IP Address |
| gt_uid | Global UID |  | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Query Parameter
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| fid_trgt_exls_cls_code | 대상 제외 구분 코드 |  | Y | 32 | 0: 전체 |
| fid_cond_mrkt_div_code | 조건 시장 분류 코드 |  | Y | 2 | 시장구분코드 (J:KRX, NX:NXT) |
| fid_cond_scr_div_code | 조건 화면 분류 코드 |  | Y | 5 | Unique key(20186) |
| fid_div_cls_code | 분류 구분 코드 |  | Y | 2 | 0:전체, 1:관리종목, 2:투자주의, 3:투자경고, 4:투자위험예고, 5:투자위험, 6:보통주, 7:우선주 |
| fid_rank_sort_cls_code | 순위 정렬 구분 코드 |  | Y | 2 | 0:매도상위,1:매수상위 |
| fid_input_date_1 | 입력 날짜1 |  | Y | 10 | 기간~ |
| fid_input_date_2 | 입력 날짜2 |  | Y | 10 | ~기간 |
| fid_input_iscd | 입력 종목코드 |  | Y | 12 | 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100 |
| fid_trgt_cls_code | 대상 구분 코드 |  | Y | 32 | 0: 전체 |
| fid_aply_rang_vol | 적용 범위 거래량 |  | Y | 18 | 0: 전체, 100: 100주 이상 |
| fid_aply_rang_prc_2 | 적용 범위 가격2 |  | Y | 18 | ~ 가격 |
| fid_aply_rang_prc_1 | 적용 범위 가격1 |  | Y | 18 | 가격 ~ |

## Response Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 |  | Y | 40 | application/json; charset=utf-8 |
| tr_id | 거래ID |  | Y | 13 | 요청한 tr_id |
| tr_cont | 연속 거래 여부 |  | N | 1 | tr_cont를 이용한 다음조회 불가 API |
| gt_uid | Global UID |  | N | 32 | [법인 전용] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| rt_cd | 성공 실패 여부 |  | Y | 1 |  |
| msg_cd | 응답코드 |  | Y | 8 |  |
| msg1 | 응답메세지 |  | Y | 80 |  |
| output | 응답상세 |  | Y |  | array |
| data_rank | 데이터 순위 |  | Y | 10 |  |
| mksc_shrn_iscd | 유가증권 단축 종목코드 |  | Y | 9 |  |
| hts_kor_isnm | HTS 한글 종목명 |  | Y | 40 |  |
| stck_prpr | 주식 현재가 |  | Y | 10 |  |
| prdy_vrss_sign | 전일 대비 부호 |  | Y | 1 |  |
| prdy_vrss | 전일 대비 |  | Y | 10 |  |
| prdy_ctrt | 전일 대비율 |  | Y | 82 |  |
| acml_vol | 누적 거래량 |  | Y | 18 |  |
| acml_tr_pbmn | 누적 거래 대금 |  | Y | 18 |  |
| seln_cnqn_smtn | 매도 체결량 합계 |  | Y | 18 |  |
| shnu_cnqn_smtn | 매수2 체결량 합계 |  | Y | 18 |  |
| ntby_cnqn | 순매수 체결량 |  | Y | 18 |  |
