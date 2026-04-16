<!-- endpoint: /tryitout/H0ZOCNT0 -->
<!-- category: [국내선물옵션] 실시간시세 -->
<!-- korean_name: 주식옵션 실시간체결가 -->

# 주식옵션 실시간체결가 [실시간-044]

## Info
- **Method**: POST
- **URL**: /tryitout/H0ZOCNT0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 모의투자 미지원
- **실전TRID**: H0ZOCNT0
- **모의TRID**: 모의투자 미지원

## 개요
요청

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓 접속키 | String | Y | 36 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 / P : 개인 |
| tr_type | 등록/해제 | String | Y | 1 | "1: 등록, 2:해제" |
| content-type | 컨텐츠타입 | String | Y | 20 | utf-8 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 7 | H0ZOCNT0 |
| tr_key | 종목코드 | String | Y | 6 | 종목코드 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| OPTN_SHRN_ISCD | 옵션단축종목코드 | Object | Y | 9 | '각 항목사이에는 구분자로 ^ 사용,모든 데이터타입은 String으로 변환되어 push 처리됨' |
| BSOP_HOUR | 영업시간 | String | Y | 6 |  |
| OPTN_PRPR | 옵션현재가 | String | Y | 1 |  |
| PRDY_VRSS_SIGN | 전일대비부호 | String | Y | 1 |  |
| OPTN_PRDY_VRSS | 옵션전일대비 | String | Y | 1 |  |
| PRDY_CTRT | 전일대비율 | String | Y | 1 |  |
| OPTN_OPRC | 옵션시가2 | String | Y | 1 |  |
| OPTN_HGPR | 옵션최고가 | String | Y | 1 |  |
| OPTN_LWPR | 옵션최저가 | String | Y | 1 |  |
| LAST_CNQN | 최종거래량 | String | Y | 1 |  |
| ACML_VOL | 누적거래량 | String | Y | 1 |  |
| ACML_TR_PBMN | 누적거래대금 | String | Y | 1 |  |
| HTS_THPR | HTS이론가 | String | Y | 1 |  |
| HTS_OTST_STPL_QTY | HTS미결제약정수량 | String | Y | 1 |  |
| OTST_STPL_QTY_ICDC | 미결제약정수량증감 | String | Y | 1 |  |
| OPRC_HOUR | 시가시간 | String | Y | 6 |  |
| OPRC_VRSS_PRPR_SIGN | 시가2대비현재가부호 | String | Y | 1 |  |
| OPRC_VRSS_NMIX_PRPR | 시가대비지수현재가 | String | Y | 1 |  |
| HGPR_HOUR | 최고가시간 | String | Y | 6 |  |
| HGPR_VRSS_PRPR_SIGN | 최고가대비현재가부호 | String | Y | 1 |  |
| HGPR_VRSS_NMIX_PRPR | 최고가대비지수현재가 | String | Y | 1 |  |
| LWPR_HOUR | 최저가시간 | String | Y | 6 |  |
| LWPR_VRSS_PRPR_SIGN | 최저가대비현재가부호 | String | Y | 1 |  |
| LWPR_VRSS_NMIX_PRPR | 최저가대비지수현재가 | String | Y | 1 |  |
| SHNU_RATE | 매수2비율 | String | Y | 1 |  |
| PRMM_VAL | 프리미엄값 | String | Y | 1 |  |
| INVL_VAL | 내재가치값 | String | Y | 1 |  |
| TMVL_VAL | 시간가치값 | String | Y | 1 |  |
| DELTA | 델타 | String | Y | 1 |  |
| GAMA | 감마 | String | Y | 1 |  |
| VEGA | 베가 | String | Y | 1 |  |
| THETA | 세타 | String | Y | 1 |  |
| RHO | 로우 | String | Y | 1 |  |
| HTS_INTS_VLTL | HTS내재변동성 | String | Y | 1 |  |
| ESDG | 괴리도 | String | Y | 1 |  |
| OTST_STPL_RGBF_QTY_ICDC | 미결제약정직전수량증감 | String | Y | 1 |  |
| THPR_BASIS | 이론베이시스 | String | Y | 1 |  |
| UNAS_HIST_VLTL | 역사적변동성 | String | Y | 1 |  |
| CTTR | 체결강도 | String | Y | 1 |  |
| DPRT | 괴리율 | String | Y | 1 |  |
| MRKT_BASIS | 시장베이시스 | String | Y | 1 |  |
| OPTN_ASKP1 | 옵션매도호가1 | String | Y | 1 |  |
| OPTN_BIDP1 | 옵션매수호가1 | String | Y | 1 |  |
| ASKP_RSQN1 | 매도호가잔량1 | String | Y | 1 |  |
| BIDP_RSQN1 | 매수호가잔량1 | String | Y | 1 |  |
| SELN_CNTG_CSNU | 매도체결건수 | String | Y | 1 |  |
| SHNU_CNTG_CSNU | 매수체결건수 | String | Y | 1 |  |
| NTBY_CNTG_CSNU | 순매수체결건수 | String | Y | 1 |  |
| SELN_CNTG_SMTN | 총매도수량 | String | Y | 1 |  |
| SHNU_CNTG_SMTN | 총매수수량 | String | Y | 1 |  |
| TOTAL_ASKP_RSQN | 총매도호가잔량 | String | Y | 1 |  |
| TOTAL_BIDP_RSQN | 총매수호가잔량 | String | Y | 1 |  |
| PRDY_VOL_VRSS_ACML_VOL_RATE | 전일거래량대비등락율 | String | Y | 1 |  |
