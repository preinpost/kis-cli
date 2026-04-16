<!-- endpoint: /tryitout/H0NXMKO0 -->
<!-- category: [국내주식] 실시간시세 -->
<!-- korean_name: 국내주식 장운영정보 (NXT) -->

# 국내주식 장운영정보 (NXT)

## Info
- **Method**: POST
- **URL**: /tryitout/H0NXMKO0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 모의투자 미지원
- **실전TRID**: H0NXMKO0
- **모의TRID**: 모의투자 미지원

## 개요
요청

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓 접속키 | String | Y | 286 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 P : 개인 |
| tr_type | 거래타입 | String | Y | 1 | 1 : 등록2 : 해제 |
| content-type | 컨텐츠타입 | String | Y | 1 | utf-8 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 2 | H0NXMKO0 : 국내주식 장운영정보 (NXT) |
| tr_key | 구분값 | String | Y | 12 | 종목코드 (ex 005930 삼성전자) |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| MKSC_SHRN_ISCD | 종목코드 | String | Y | 9 |  |
| TRHT_YN | 거래정지 여부 | String | Y | 1 |  |
| TR_SUSP_REAS_CNTT | 거래 정지 사유 내용 | String | Y | 100 |  |
| MKOP_CLS_CODE | 장운영 구분 코드 | String | Y | 3 |  |
| ANTC_MKOP_CLS_CODE | 예상 장운영 구분 코드 | String | Y | 3 |  |
| MRKT_TRTM_CLS_CODE | 임의연장구분코드 | String | Y | 1 |  |
| DIVI_APP_CLS_CODE | 동시호가배분처리구분코드 | String | Y | 2 |  |
| ISCD_STAT_CLS_CODE | 종목상태구분코드 | String | Y | 2 |  |
| VI_CLS_CODE | VI적용구분코드 | String | Y | 1 |  |
| OVTM_VI_CLS_CODE | 시간외단일가VI적용구분코드 | String | Y | 1 |  |
| EXCH_CLS_CODE | 거래소 구분코드 | String | Y | 1 |  |
