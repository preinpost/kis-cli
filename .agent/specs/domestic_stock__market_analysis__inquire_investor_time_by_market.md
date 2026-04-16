<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-investor-time-by-market -->
<!-- category: [국내주식] 시세분석 -->
<!-- korean_name: 시장별 투자자매매동향(시세) -->

# 시장별 투자자매매동향(시세)[v1_국내주식-074]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-investor-time-by-market
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: FHPTJ04030000
- **모의TRID**: 모의투자 미지원

## 개요
시장별 투자자매매동향(시세성) API입니다.
한국투자 HTS(eFriend Plus) > [0403] 시장별 시간동향 의 상단 표 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHPTJ04030000 |
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
| fid_input_iscd | 시장구분 | String | Y | 12 | 코스피: KSP, 코스닥:KSQ,선물,콜옵션,풋옵션 : K2I, 주식선물:999,ETF: ETF, ELW:ELW, ETN: ETN, 미니: MKI, 위클리월 : WKM, 위클리목: WKI코스닥150: KQI |
| fid_input_iscd_2 | 업종구분 | String | Y | 8 | - fid_input_iscd: KSP(코스피) 혹은 KSQ(코스닥)인 경우코스피(0001_종합, .…0027_제조업 )코스닥(1001_종합, …. 1041_IT부품)...포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)- fid_input_iscd가 K2I인 경우F001(선물)OC01(콜옵션)OP01(풋옵션)- fid_input_iscd가 999인 경우S001(주식선물)- fid_input_iscd가 ETF인 경우T000(ETF)- fid_input_iscd가 ELW인 경우W000(ELW)- fid_input_iscd가 ETN인 경우E199(ETN)- fid_input_iscd가 MKI인 경우F004(미니선물)OC02(미니콜옵션)OP02(미니풋옵션)- fid_input_iscd가 WKM인 경우OC05(위클리콜(월))OP05(위클리풋(월))- fid_input_iscd가 WKI인 경우OC04(위클리콜(목))OP04(위클리풋(목)) - fid_input_iscd가 KQI인 경우F002(코스닥150선물)OC03(코스닥150콜옵션)OP03(코스닥150풋옵션) |

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
| output | 응답상세 | Object | Y |  |  |
| frgn_seln_vol | 외국인 매도 거래량 | String | Y | 18 |  |
| frgn_shnu_vol | 외국인 매수2 거래량 | String | Y | 18 |  |
| frgn_ntby_qty | 외국인 순매수 수량 | String | Y | 12 |  |
| frgn_seln_tr_pbmn | 외국인 매도 거래 대금 | String | Y | 18 |  |
| frgn_shnu_tr_pbmn | 외국인 매수2 거래 대금 | String | Y | 18 |  |
| frgn_ntby_tr_pbmn | 외국인 순매수 거래 대금 | String | Y | 18 |  |
| prsn_seln_vol | 개인 매도 거래량 | String | Y | 18 |  |
| prsn_shnu_vol | 개인 매수2 거래량 | String | Y | 18 |  |
| prsn_ntby_qty | 개인 순매수 수량 | String | Y | 12 |  |
| prsn_seln_tr_pbmn | 개인 매도 거래 대금 | String | Y | 18 |  |
| prsn_shnu_tr_pbmn | 개인 매수2 거래 대금 | String | Y | 18 |  |
| prsn_ntby_tr_pbmn | 개인 순매수 거래 대금 | String | Y | 18 |  |
| orgn_seln_vol | 기관계 매도 거래량 | String | Y | 18 |  |
| orgn_shnu_vol | 기관계 매수2 거래량 | String | Y | 18 |  |
| orgn_ntby_qty | 기관계 순매수 수량 | String | Y | 18 |  |
| orgn_seln_tr_pbmn | 기관계 매도 거래 대금 | String | Y | 18 |  |
| orgn_shnu_tr_pbmn | 기관계 매수2 거래 대금 | String | Y | 18 |  |
| orgn_ntby_tr_pbmn | 기관계 순매수 거래 대금 | String | Y | 18 |  |
| scrt_seln_vol | 증권 매도 거래량 | String | Y | 18 |  |
| scrt_shnu_vol | 증권 매수2 거래량 | String | Y | 18 |  |
| scrt_ntby_qty | 증권 순매수 수량 | String | Y | 12 |  |
| scrt_seln_tr_pbmn | 증권 매도 거래 대금 | String | Y | 18 |  |
| scrt_shnu_tr_pbmn | 증권 매수2 거래 대금 | String | Y | 18 |  |
| scrt_ntby_tr_pbmn | 증권 순매수 거래 대금 | String | Y | 18 |  |
| ivtr_seln_vol | 투자신탁 매도 거래량 | String | Y | 18 |  |
| ivtr_shnu_vol | 투자신탁 매수2 거래량 | String | Y | 18 |  |
| ivtr_ntby_qty | 투자신탁 순매수 수량 | String | Y | 12 |  |
| ivtr_seln_tr_pbmn | 투자신탁 매도 거래 대금 | String | Y | 18 |  |
| ivtr_shnu_tr_pbmn | 투자신탁 매수2 거래 대금 | String | Y | 18 |  |
| ivtr_ntby_tr_pbmn | 투자신탁 순매수 거래 대금 | String | Y | 18 |  |
| pe_fund_seln_tr_pbmn | 사모 펀드 매도 거래 대금 | String | Y | 18 |  |
| pe_fund_seln_vol | 사모 펀드 매도 거래량 | String | Y | 18 |  |
| pe_fund_ntby_vol | 사모 펀드 순매수 거래량 | String | Y | 18 |  |
| pe_fund_shnu_tr_pbmn | 사모 펀드 매수2 거래 대금 | String | Y | 18 |  |
| pe_fund_shnu_vol | 사모 펀드 매수2 거래량 | String | Y | 18 |  |
| pe_fund_ntby_tr_pbmn | 사모 펀드 순매수 거래 대금 | String | Y | 18 |  |
| bank_seln_vol | 은행 매도 거래량 | String | Y | 18 |  |
| bank_shnu_vol | 은행 매수2 거래량 | String | Y | 18 |  |
| bank_ntby_qty | 은행 순매수 수량 | String | Y | 12 |  |
| bank_seln_tr_pbmn | 은행 매도 거래 대금 | String | Y | 18 |  |
| bank_shnu_tr_pbmn | 은행 매수2 거래 대금 | String | Y | 18 |  |
| bank_ntby_tr_pbmn | 은행 순매수 거래 대금 | String | Y | 18 |  |
| insu_seln_vol | 보험 매도 거래량 | String | Y | 18 |  |
| insu_shnu_vol | 보험 매수2 거래량 | String | Y | 18 |  |
| insu_ntby_qty | 보험 순매수 수량 | String | Y | 12 |  |
| insu_seln_tr_pbmn | 보험 매도 거래 대금 | String | Y | 18 |  |
| insu_shnu_tr_pbmn | 보험 매수2 거래 대금 | String | Y | 18 |  |
| insu_ntby_tr_pbmn | 보험 순매수 거래 대금 | String | Y | 18 |  |
| mrbn_seln_vol | 종금 매도 거래량 | String | Y | 18 |  |
| mrbn_shnu_vol | 종금 매수2 거래량 | String | Y | 18 |  |
| mrbn_ntby_qty | 종금 순매수 수량 | String | Y | 12 |  |
| mrbn_seln_tr_pbmn | 종금 매도 거래 대금 | String | Y | 18 |  |
| mrbn_shnu_tr_pbmn | 종금 매수2 거래 대금 | String | Y | 18 |  |
| mrbn_ntby_tr_pbmn | 종금 순매수 거래 대금 | String | Y | 18 |  |
| fund_seln_vol | 기금 매도 거래량 | String | Y | 18 |  |
| fund_shnu_vol | 기금 매수2 거래량 | String | Y | 18 |  |
| fund_ntby_qty | 기금 순매수 수량 | String | Y | 12 |  |
| fund_seln_tr_pbmn | 기금 매도 거래 대금 | String | Y | 18 |  |
| fund_shnu_tr_pbmn | 기금 매수2 거래 대금 | String | Y | 18 |  |
| fund_ntby_tr_pbmn | 기금 순매수 거래 대금 | String | Y | 18 |  |
| etc_orgt_seln_vol | 기타 단체 매도 거래량 | String | Y | 18 |  |
| etc_orgt_shnu_vol | 기타 단체 매수2 거래량 | String | Y | 18 |  |
| etc_orgt_ntby_vol | 기타 단체 순매수 거래량 | String | Y | 18 |  |
| etc_orgt_seln_tr_pbmn | 기타 단체 매도 거래 대금 | String | Y | 18 |  |
| etc_orgt_shnu_tr_pbmn | 기타 단체 매수2 거래 대금 | String | Y | 18 |  |
| etc_orgt_ntby_tr_pbmn | 기타 단체 순매수 거래 대금 | String | Y | 18 |  |
| etc_corp_seln_vol | 기타 법인 매도 거래량 | String | Y | 18 |  |
| etc_corp_shnu_vol | 기타 법인 매수2 거래량 | String | Y | 18 |  |
| etc_corp_ntby_vol | 기타 법인 순매수 거래량 | String | Y | 18 |  |
| etc_corp_seln_tr_pbmn | 기타 법인 매도 거래 대금 | String | Y | 18 |  |
| etc_corp_shnu_tr_pbmn | 기타 법인 매수2 거래 대금 | String | Y | 18 |  |
| etc_corp_ntby_tr_pbmn | 기타 법인 순매수 거래 대금 | String | Y | 18 |  |
