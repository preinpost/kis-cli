<!-- endpoint: /tryitout/H0NXASP0 -->
<!-- category: [국내주식] 실시간시세 -->
<!-- korean_name: 국내주식 실시간호가 (NXT) -->

# 국내주식 실시간호가 (NXT)

## Info
- **Method**: POST
- **URL**: /tryitout/H0NXASP0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 모의투자 미지원
- **실전TRID**: H0NXASP0
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
| tr_id | 거래ID | String | Y | 2 | H0NXASP0 : 실시간 주식 호가 (NXT) |
| tr_key | 구분값 | String | Y | 12 | 종목코드 (ex 005930 삼성전자) |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| MKSC_SHRN_ISCD | 유가증권 단축 종목코드 | String | Y | 9 |  |
| BSOP_HOUR | 영업 시간 | String | Y | 6 |  |
| HOUR_CLS_CODE | 시간 구분 코드 | String | Y | 1 |  |
| ASKP1 | 매도호가1 | String | Y | 4 |  |
| ASKP2 | 매도호가2 | String | Y | 4 |  |
| ASKP3 | 매도호가3 | String | Y | 4 |  |
| ASKP4 | 매도호가4 | String | Y | 4 |  |
| ASKP5 | 매도호가5 | String | Y | 4 |  |
| ASKP6 | 매도호가6 | String | Y | 4 |  |
| ASKP7 | 매도호가7 | String | Y | 4 |  |
| ASKP8 | 매도호가8 | String | Y | 4 |  |
| ASKP9 | 매도호가9 | String | Y | 4 |  |
| ASKP10 | 매도호가10 | String | Y | 4 |  |
| BIDP1 | 매수호가1 | String | Y | 4 |  |
| BIDP2 | 매수호가2 | String | Y | 4 |  |
| BIDP3 | 매수호가3 | String | Y | 4 |  |
| BIDP4 | 매수호가4 | String | Y | 4 |  |
| BIDP5 | 매수호가5 | String | Y | 4 |  |
| BIDP6 | 매수호가6 | String | Y | 4 |  |
| BIDP7 | 매수호가7 | String | Y | 4 |  |
| BIDP8 | 매수호가8 | String | Y | 4 |  |
| BIDP9 | 매수호가9 | String | Y | 4 |  |
| BIDP10 | 매수호가10 | String | Y | 4 |  |
| ASKP_RSQN1 | 매도호가 잔량1 | String | Y | 8 |  |
| ASKP_RSQN2 | 매도호가 잔량2 | String | Y | 8 |  |
| ASKP_RSQN3 | 매도호가 잔량3 | String | Y | 8 |  |
| ASKP_RSQN4 | 매도호가 잔량4 | String | Y | 8 |  |
| ASKP_RSQN5 | 매도호가 잔량5 | String | Y | 8 |  |
| ASKP_RSQN6 | 매도호가 잔량6 | String | Y | 8 |  |
| ASKP_RSQN7 | 매도호가 잔량7 | String | Y | 8 |  |
| ASKP_RSQN8 | 매도호가 잔량8 | String | Y | 8 |  |
| ASKP_RSQN9 | 매도호가 잔량9 | String | Y | 8 |  |
| ASKP_RSQN10 | 매도호가 잔량10 | String | Y | 8 |  |
| BIDP_RSQN1 | 매수호가 잔량1 | String | Y | 8 |  |
| BIDP_RSQN2 | 매수호가 잔량2 | String | Y | 8 |  |
| BIDP_RSQN3 | 매수호가 잔량3 | String | Y | 8 |  |
| BIDP_RSQN4 | 매수호가 잔량4 | String | Y | 8 |  |
| BIDP_RSQN5 | 매수호가 잔량5 | String | Y | 8 |  |
| BIDP_RSQN6 | 매수호가 잔량6 | String | Y | 8 |  |
| BIDP_RSQN7 | 매수호가 잔량7 | String | Y | 8 |  |
| BIDP_RSQN8 | 매수호가 잔량8 | String | Y | 8 |  |
| BIDP_RSQN9 | 매수호가 잔량9 | String | Y | 8 |  |
| BIDP_RSQN10 | 매수호가 잔량10 | String | Y | 8 |  |
| TOTAL_ASKP_RSQN | 총 매도호가 잔량 | String | Y | 8 |  |
| TOTAL_BIDP_RSQN | 총 매수호가 잔량 | String | Y | 8 |  |
| OVTM_TOTAL_ASKP_RSQN | 시간외 총 매도호가 잔량 | String | Y | 8 |  |
| OVTM_TOTAL_BIDP_RSQN | 시간외 총 매수호가 잔량 | String | Y | 8 |  |
| ANTC_CNPR | 예상 체결가 | String | Y | 4 |  |
| ANTC_CNQN | 예상 체결량 | String | Y | 8 |  |
| ANTC_VOL | 예상 거래량 | String | Y | 8 |  |
| ANTC_CNTG_VRSS | 예상 체결 대비 | String | Y | 4 |  |
| ANTC_CNTG_VRSS_SIGN | 예상 체결 대비 부호 | String | Y | 1 |  |
| ANTC_CNTG_PRDY_CTRT | 예상 체결 전일 대비율 | String | Y | 8 |  |
| ACML_VOL | 누적 거래량 | String | Y | 8 |  |
| TOTAL_ASKP_RSQN_ICDC | 총 매도호가 잔량 증감 | String | Y | 4 |  |
| TOTAL_BIDP_RSQN_ICDC | 총 매수호가 잔량 증감 | String | Y | 4 |  |
| OVTM_TOTAL_ASKP_ICDC | 시간외 총 매도호가 증감 | String | Y | 4 |  |
| OVTM_TOTAL_BIDP_ICDC | 시간외 총 매수호가 증감 | String | Y | 4 |  |
| STCK_DEAL_CLS_CODE | 주식 매매 구분 코드 | String | Y | 2 |  |
| KMID_PRC | KRX 중간가 | String | Y | 4 |  |
| KMID_TOTAL_RSQN | KRX 중간가잔량합계수량 | String | Y | 8 |  |
| KMID_CLS_CODE | KRX 중간가 매수매도 구분 | String | Y | 1 |  |
| NMID_PRC | NXT 중간가 | String | Y | 4 |  |
| NMID_TOTAL_RSQN | NXT 중간가잔량합계수량 | String | Y | 8 |  |
| NMID_CLS_CODE | NXT 중간가 매수매도 구분 | String | Y | 1 |  |
