<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-price-2 -->
<!-- category: [국내주식] 기본시세 -->
<!-- korean_name: 주식현재가 시세2 -->

# 주식현재가 시세2[v1_국내주식-054]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-price-2
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: 모의투자 미지원
- **실전TRID**: FHPST01010000
- **모의TRID**: 모의투자 미지원
- **Format**: JSON

## 개요
주식현재가 시세2 API입니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHPST01010000 |
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
| FID_COND_MRKT_DIV_CODE | FID 조건 시장 분류 코드 | String | Y | 2 | J:KRX, NX:NXT, UN:통합 |
| FID_INPUT_ISCD | FID 입력 종목코드 | String | Y | 12 | 000660 |

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
| rprs_mrkt_kor_name | 대표 시장 한글 명 | String | Y | 40 |  |
| new_hgpr_lwpr_cls_code | 신 고가 저가 구분 코드 | String | Y | 10 | 특정 경우에만 데이터 출력 |
| mxpr_llam_cls_code | 상하한가 구분 코드 | String | Y | 10 | 특정 경우에만 데이터 출력 |
| crdt_able_yn | 신용 가능 여부 | String | Y | 1 |  |
| stck_mxpr | 주식 상한가 | String | Y | 10 |  |
| elw_pblc_yn | ELW 발행 여부 | String | Y | 1 |  |
| prdy_clpr_vrss_oprc_rate | 전일 종가 대비 시가2 비율 | String | Y | 84 |  |
| crdt_rate | 신용 비율 | String | Y | 84 |  |
| marg_rate | 증거금 비율 | String | Y | 84 |  |
| lwpr_vrss_prpr | 최저가 대비 현재가 | String | Y | 10 |  |
| lwpr_vrss_prpr_sign | 최저가 대비 현재가 부호 | String | Y | 1 |  |
| prdy_clpr_vrss_lwpr_rate | 전일 종가 대비 최저가 비율 | String | Y | 84 |  |
| stck_lwpr | 주식 최저가 | String | Y | 10 |  |
| hgpr_vrss_prpr | 최고가 대비 현재가 | String | Y | 10 |  |
| hgpr_vrss_prpr_sign | 최고가 대비 현재가 부호 | String | Y | 1 |  |
| prdy_clpr_vrss_hgpr_rate | 전일 종가 대비 최고가 비율 | String | Y | 84 |  |
| stck_hgpr | 주식 최고가 | String | Y | 10 |  |
| oprc_vrss_prpr | 시가2 대비 현재가 | String | Y | 10 |  |
| oprc_vrss_prpr_sign | 시가2 대비 현재가 부호 | String | Y | 1 |  |
| mang_issu_yn | 관리 종목 여부 | String | Y | 1 |  |
| divi_app_cls_code | 동시호가배분처리코드 | String | Y | 2 | 11:매수상한배분 12:매수하한배분 13: 매도상한배분 14:매도하한배분 |
| short_over_yn | 단기과열여부 | String | Y | 1 |  |
| mrkt_warn_cls_code | 시장경고코드 | String | Y | 2 | 00: 없음 01: 투자주의 02:투자경고 03:투자위험 |
| invt_caful_yn | 투자유의여부 | String | Y | 1 |  |
| stange_runup_yn | 이상급등여부 | String | Y | 1 |  |
| ssts_hot_yn | 공매도과열 여부 | String | Y | 1 |  |
| low_current_yn | 저유동성 종목 여부 | String | Y | 1 |  |
| vi_cls_code | VI적용구분코드 | String | Y | 1 |  |
| short_over_cls_code | 단기과열구분코드 | String | Y | 10 |  |
| stck_llam | 주식 하한가 | String | Y | 10 |  |
| new_lstn_cls_name | 신규 상장 구분 명 | String | Y | 40 |  |
| vlnt_deal_cls_name | 임의 매매 구분 명 | String | Y | 16 |  |
| flng_cls_name | 락 구분 이름 | String | Y | 40 | 특정 경우에만 데이터 출력 |
| revl_issu_reas_name | 재평가 종목 사유 명 | String | Y | 40 | 특정 경우에만 데이터 출력 |
| mrkt_warn_cls_name | 시장 경고 구분 명 | String | Y | 40 | 특정 경우에만 데이터 출력"투자환기" / "투자경고" |
| stck_sdpr | 주식 기준가 | String | Y | 10 |  |
| bstp_cls_code | 업종 구분 코드 | String | Y | 4 |  |
| stck_prdy_clpr | 주식 전일 종가 | String | Y | 10 |  |
| insn_pbnt_yn | 불성실 공시 여부 | String | Y | 1 |  |
| fcam_mod_cls_name | 액면가 변경 구분 명 | String | Y | 10 | 특정 경우에만 데이터 출력 |
| stck_prpr | 주식 현재가 | String | Y | 10 |  |
| prdy_vrss | 전일 대비 | String | Y | 10 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| prdy_ctrt | 전일 대비율 | String | Y | 82 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| prdy_vrss_vol_rate | 전일 대비 거래량 비율 | String | Y | 84 |  |
| bstp_kor_isnm | 업종 한글 종목명 | String | Y | 40 | ※ 거래소 정보로 특정 종목은 업종구분이 없어 데이터 미회신 |
| sltr_yn | 정리매매 여부 | String | Y | 1 |  |
| trht_yn | 거래정지 여부 | String | Y | 1 |  |
| oprc_rang_cont_yn | 시가 범위 연장 여부 | String | Y | 1 |  |
| vlnt_fin_cls_code | 임의 종료 구분 코드 | String | Y | 1 |  |
| stck_oprc | 주식 시가2 | String | Y | 10 |  |
| prdy_vol | 전일 거래량 | String | Y | 18 |  |
