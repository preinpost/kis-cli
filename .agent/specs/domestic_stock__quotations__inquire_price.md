<!-- endpoint: /uapi/domestic-stock/v1/quotations/inquire-price -->
<!-- category: [국내주식] 기본시세 -->
<!-- korean_name: 주식현재가 시세 -->

# 주식현재가 시세[v1_국내주식-008]

## Info
- **Method**: GET
- **URL**: /uapi/domestic-stock/v1/quotations/inquire-price
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **실전TRID**: FHKST01010100
- **모의TRID**: FHKST01010100

## 개요
주식 현재가 시세 API입니다. 실시간 시세를 원하신다면 웹소켓 API를 활용하세요.
※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | Y | 40 | application/json; charset=utf-8 |
| authorization | 접근토큰 | String | Y | 350 | OAuth 토큰이 필요한 API 경우 발급한 Access token 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용) 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용) |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| personalseckey | 고객식별키 | String | N | 180 | [법인 필수] 제휴사 회원 관리를 위한 고객식별키 |
| tr_id | 거래ID | String | Y | 13 | FHKST01010100 |
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
| FID_COND_MRKT_DIV_CODE | 조건 시장 분류 코드 | String | Y | 2 | J:KRX, NX:NXT, UN:통합 |
| FID_INPUT_ISCD | 입력 종목코드 | String | Y | 12 | 종목코드 (ex 005930 삼성전자) // ETN은 종목코드 6자리 앞에 Q 입력 필수 |

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
| iscd_stat_cls_code | 종목 상태 구분 코드 | String | Y | 3 | 51 : 관리종목52 : 투자위험53 : 투자경고54 : 투자주의55 : 신용가능57 : 증거금 100%58 : 거래정지59 : 단기과열종목 |
| marg_rate | 증거금 비율 | String | Y | 84 |  |
| rprs_mrkt_kor_name | 대표 시장 한글 명 | String | Y | 40 |  |
| new_hgpr_lwpr_cls_code | 신 고가 저가 구분 코드 | String | Y | 10 |  |
| bstp_kor_isnm | 업종 한글 종목명 | String | Y | 40 |  |
| temp_stop_yn | 임시 정지 여부 | String | Y | 1 |  |
| oprc_rang_cont_yn | 시가 범위 연장 여부 | String | Y | 1 |  |
| clpr_rang_cont_yn | 종가 범위 연장 여부 | String | Y | 1 |  |
| crdt_able_yn | 신용 가능 여부 | String | Y | 1 |  |
| grmn_rate_cls_code | 보증금 비율 구분 코드 | String | Y | 3 |  |
| elw_pblc_yn | ELW 발행 여부 | String | Y | 1 |  |
| stck_prpr | 주식 현재가 | String | Y | 10 |  |
| prdy_vrss | 전일 대비 | String | Y | 10 |  |
| prdy_vrss_sign | 전일 대비 부호 | String | Y | 1 |  |
| prdy_ctrt | 전일 대비율 | String | Y | 82 |  |
| acml_tr_pbmn | 누적 거래 대금 | String | Y | 18 |  |
| acml_vol | 누적 거래량 | String | Y | 18 |  |
| prdy_vrss_vol_rate | 전일 대비 거래량 비율 | String | Y | 84 |  |
| stck_oprc | 주식 시가2 | String | Y | 10 |  |
| stck_hgpr | 주식 최고가 | String | Y | 10 |  |
| stck_lwpr | 주식 최저가 | String | Y | 10 |  |
| stck_mxpr | 주식 상한가 | String | Y | 10 |  |
| stck_llam | 주식 하한가 | String | Y | 10 |  |
| stck_sdpr | 주식 기준가 | String | Y | 10 |  |
| wghn_avrg_stck_prc | 가중 평균 주식 가격 | String | Y | 192 |  |
| hts_frgn_ehrt | HTS 외국인 소진율 | String | Y | 82 |  |
| frgn_ntby_qty | 외국인 순매수 수량 | String | Y | 12 |  |
| pgtr_ntby_qty | 프로그램매매 순매수 수량 | String | Y | 18 |  |
| pvt_scnd_dmrs_prc | 피벗 2차 디저항 가격 | String | Y | 10 |  |
| pvt_frst_dmrs_prc | 피벗 1차 디저항 가격 | String | Y | 10 |  |
| pvt_pont_val | 피벗 포인트 값 | String | Y | 10 |  |
| pvt_frst_dmsp_prc | 피벗 1차 디지지 가격 | String | Y | 10 |  |
| pvt_scnd_dmsp_prc | 피벗 2차 디지지 가격 | String | Y | 10 |  |
| dmrs_val | 디저항 값 | String | Y | 10 |  |
| dmsp_val | 디지지 값 | String | Y | 10 |  |
| cpfn | 자본금 | String | Y | 22 |  |
| rstc_wdth_prc | 제한 폭 가격 | String | Y | 10 |  |
| stck_fcam | 주식 액면가 | String | Y | 11 |  |
| stck_sspr | 주식 대용가 | String | Y | 10 |  |
| aspr_unit | 호가단위 | String | Y | 10 |  |
| hts_deal_qty_unit_val | HTS 매매 수량 단위 값 | String | Y | 10 |  |
| lstn_stcn | 상장 주수 | String | Y | 18 |  |
| hts_avls | HTS 시가총액 | String | Y | 18 |  |
| per | PER | String | Y | 82 |  |
| pbr | PBR | String | Y | 82 |  |
| stac_month | 결산 월 | String | Y | 2 |  |
| vol_tnrt | 거래량 회전율 | String | Y | 82 |  |
| eps | EPS | String | Y | 112 |  |
| bps | BPS | String | Y | 112 |  |
| d250_hgpr | 250일 최고가 | String | Y | 10 |  |
| d250_hgpr_date | 250일 최고가 일자 | String | Y | 8 |  |
| d250_hgpr_vrss_prpr_rate | 250일 최고가 대비 현재가 비율 | String | Y | 84 |  |
| d250_lwpr | 250일 최저가 | String | Y | 10 |  |
| d250_lwpr_date | 250일 최저가 일자 | String | Y | 8 |  |
| d250_lwpr_vrss_prpr_rate | 250일 최저가 대비 현재가 비율 | String | Y | 84 |  |
| stck_dryy_hgpr | 주식 연중 최고가 | String | Y | 10 |  |
| dryy_hgpr_vrss_prpr_rate | 연중 최고가 대비 현재가 비율 | String | Y | 84 |  |
| dryy_hgpr_date | 연중 최고가 일자 | String | Y | 8 |  |
| stck_dryy_lwpr | 주식 연중 최저가 | String | Y | 10 |  |
| dryy_lwpr_vrss_prpr_rate | 연중 최저가 대비 현재가 비율 | String | Y | 84 |  |
| dryy_lwpr_date | 연중 최저가 일자 | String | Y | 8 |  |
| w52_hgpr | 52주일 최고가 | String | Y | 10 |  |
| w52_hgpr_vrss_prpr_ctrt | 52주일 최고가 대비 현재가 대비 | String | Y | 82 |  |
| w52_hgpr_date | 52주일 최고가 일자 | String | Y | 8 |  |
| w52_lwpr | 52주일 최저가 | String | Y | 10 |  |
| w52_lwpr_vrss_prpr_ctrt | 52주일 최저가 대비 현재가 대비 | String | Y | 82 |  |
| w52_lwpr_date | 52주일 최저가 일자 | String | Y | 8 |  |
| whol_loan_rmnd_rate | 전체 융자 잔고 비율 | String | Y | 84 |  |
| ssts_yn | 공매도가능여부 | String | Y | 1 |  |
| stck_shrn_iscd | 주식 단축 종목코드 | String | Y | 9 |  |
| fcam_cnnm | 액면가 통화명 | String | Y | 20 |  |
| cpfn_cnnm | 자본금 통화명 | String | Y | 20 |  |
| apprch_rate | 접근도 | String | Y | 112 |  |
| frgn_hldn_qty | 외국인 보유 수량 | String | Y | 18 |  |
| vi_cls_code | VI적용구분코드 | String | Y | 1 |  |
| ovtm_vi_cls_code | 시간외단일가VI적용구분코드 | String | Y | 1 |  |
| last_ssts_cntg_qty | 최종 공매도 체결 수량 | String | Y | 12 |  |
| invt_caful_yn | 투자유의여부 | String | Y | 1 |  |
| mrkt_warn_cls_code | 시장경고코드 | String | Y | 2 |  |
| short_over_yn | 단기과열여부 | String | Y | 1 |  |
| sltr_yn | 정리매매여부 | String | Y | 1 |  |
| mang_issu_cls_code | 관리종목여부 | String | Y | 1 |  |
