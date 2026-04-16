<!-- endpoint: /tryitout/H0NXCNT0 -->
<!-- category: [국내주식] 실시간시세 -->
<!-- korean_name: 국내주식 실시간체결가 (NXT) -->

# 국내주식 실시간체결가 (NXT)

## Info
- **Method**: POST
- **URL**: /tryitout/H0NXCNT0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 모의투자 미지원
- **실전TRID**: H0NXCNT0
- **모의TRID**: 모의투자 미지원

## 개요
요청

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓 접속키 | String | N | 286 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| custtype | 고객타입 | String | N | 1 | 'B : 법인P : 개인' |
| tr_type | 거래타입 | String | N | 1 | '1 : 등록2 : 해제' |
| content-type | 컨텐츠타입 | String | N | 1 | ' utf-8' |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 2 | H0NXCNT0 : 주식종목체결 (NXT) |
| tr_key | 구분값 | String | Y | 12 | 종목코드 (ex 005930 삼성전자) |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| MKSC_SHRN_ISCD | 유가증권 단축 종목코드 | String | Y | 9 |  |
| STCK_CNTG_HOUR | 주식 체결 시간 | String | Y | 6 |  |
| STCK_PRPR | 주식 현재가 | String | Y | 4 |  |
| PRDY_VRSS_SIGN | 전일 대비 부호 | String | Y | 1 |  |
| PRDY_VRSS | 전일 대비 | String | Y | 4 |  |
| PRDY_CTRT | 전일 대비율 | String | Y | 8 |  |
| WGHN_AVRG_STCK_PRC | 가중 평균 주식 가격 | String | Y | 8 |  |
| STCK_OPRC | 주식 시가 | String | Y | 4 |  |
| STCK_HGPR | 주식 최고가 | String | Y | 4 |  |
| STCK_LWPR | 주식 최저가 | String | Y | 4 |  |
| ASKP1 | 매도호가1 | String | Y | 4 |  |
| BIDP1 | 매수호가1 | String | Y | 4 |  |
| CNTG_VOL | 체결 거래량 | String | Y | 8 |  |
| ACML_VOL | 누적 거래량 | String | Y | 8 |  |
| ACML_TR_PBMN | 누적 거래 대금 | String | Y | 8 |  |
| SELN_CNTG_CSNU | 매도 체결 건수 | String | Y | 4 |  |
| SHNU_CNTG_CSNU | 매수 체결 건수 | String | Y | 4 |  |
| NTBY_CNTG_CSNU | 순매수 체결 건수 | String | Y | 4 |  |
| CTTR | 체결강도 | String | Y | 8 |  |
| SELN_CNTG_SMTN | 총 매도 수량 | String | Y | 8 |  |
| SHNU_CNTG_SMTN | 총 매수 수량 | String | Y | 8 |  |
| CNTG_CLS_CODE | 체결구분 | String | Y | 1 |  |
| SHNU_RATE | 매수비율 | String | Y | 8 |  |
| PRDY_VOL_VRSS_ACML_VOL_RATE | 전일 거래량 대비 등락율 | String | Y | 8 |  |
| OPRC_HOUR | 시가 시간 | String | Y | 6 |  |
| OPRC_VRSS_PRPR_SIGN | 시가대비구분 | String | Y | 1 |  |
| OPRC_VRSS_PRPR | 시가대비 | String | Y | 4 |  |
| HGPR_HOUR | 최고가 시간 | String | Y | 6 |  |
| HGPR_VRSS_PRPR_SIGN | 고가대비구분 | String | Y | 1 |  |
| HGPR_VRSS_PRPR | 고가대비 | String | Y | 4 |  |
| LWPR_HOUR | 최저가 시간 | String | Y | 6 |  |
| LWPR_VRSS_PRPR_SIGN | 저가대비구분 | String | Y | 1 |  |
| LWPR_VRSS_PRPR | 저가대비 | String | Y | 4 |  |
| BSOP_DATE | 영업 일자 | String | Y | 8 |  |
| NEW_MKOP_CLS_CODE | 신 장운영 구분 코드 | String | Y | 2 |  |
| TRHT_YN | 거래정지 여부 | String | Y | 1 |  |
| ASKP_RSQN1 | 매도호가 잔량1 | String | Y | 8 |  |
| BIDP_RSQN1 | 매수호가 잔량1 | String | Y | 8 |  |
| TOTAL_ASKP_RSQN | 총 매도호가 잔량 | String | Y | 8 |  |
| TOTAL_BIDP_RSQN | 총 매수호가 잔량 | String | Y | 8 |  |
| VOL_TNRT | 거래량 회전율 | String | Y | 8 |  |
| PRDY_SMNS_HOUR_ACML_VOL | 전일 동시간 누적 거래량 | String | Y | 8 |  |
| PRDY_SMNS_HOUR_ACML_VOL_RATE | 전일 동시간 누적 거래량 비율 | String | Y | 8 |  |
| HOUR_CLS_CODE | 시간 구분 코드 | String | Y | 1 |  |
| MRKT_TRTM_CLS_CODE | 임의종료구분코드 | String | Y | 1 |  |
| VI_STND_PRC | 정적VI발동기준가 | String | Y | 4 |  |
