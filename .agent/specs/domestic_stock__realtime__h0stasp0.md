<!-- endpoint: /tryitout/H0STASP0 -->
<!-- category: [국내주식] 실시간시세 -->
<!-- korean_name: 국내주식 실시간호가 (KRX) -->

# 국내주식 실시간호가 (KRX) [실시간-004]

## Info
- **Method**: POST
- **URL**: /tryitout/H0STASP0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: ws://ops.koreainvestment.com:31000
- **실전TRID**: H0STASP0
- **모의TRID**: H0STASP0
- **Content-Type**: text/plain

## 개요
[참고자료]
실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
[호출 데이터]
헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
[응답 데이터]
1. 정상 등록 여부 (JSON)
- JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
- JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
- JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
2. 실시간 결과 응답 ( | 로 구분되는 값)
- 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
- TR_ID : 등록한 tr_id
- 데이터 건수 : (ex. 001 데이터 건수를 참조하여 활용)
- 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓 접속키 | String | Y | 286 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| custtype | 고객타입 | String | Y | 1 | B : 법인P : 개인 |
| tr_type | 거래타입 | String | Y | 1 | 1 : 등록2 : 해제 |
| content-type | 컨텐츠타입 | String | Y | 1 | utf-8 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 1 | [실전/모의투자]H0STASP0 : 주식호가 |
| tr_key | 구분값 | String | Y | 1 | 종목번호 (6자리)ETN의 경우, Q로 시작 (EX. Q500001) |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| MKSC_SHRN_ISCD | 유가증권 단축 종목코드 | String | Y | 9 |  |
| BSOP_HOUR | 영업 시간 | String | Y | 6 |  |
| HOUR_CLS_CODE | 시간 구분 코드 | String | Y | 1 | 0 : 장중A : 장후예상B : 장전예상C : 9시이후의 예상가, VI발동D : 시간외 단일가 예상 |
| ASKP1 | 매도호가1 | Number | Y | 4 |  |
| ASKP2 | 매도호가2 | Number | Y | 4 |  |
| ASKP3 | 매도호가3 | Number | Y | 4 |  |
| ASKP4 | 매도호가4 | Number | Y | 4 |  |
| ASKP5 | 매도호가5 | Number | Y | 4 |  |
| ASKP6 | 매도호가6 | Number | Y | 4 |  |
| ASKP7 | 매도호가7 | Number | Y | 4 |  |
| ASKP8 | 매도호가8 | Number | Y | 4 |  |
| ASKP9 | 매도호가9 | Number | Y | 4 |  |
| ASKP10 | 매도호가10 | Number | Y | 4 |  |
| BIDP1 | 매수호가1 | Number | Y | 4 |  |
| BIDP2 | 매수호가2 | Number | Y | 4 |  |
| BIDP3 | 매수호가3 | Number | Y | 4 |  |
| BIDP4 | 매수호가4 | Number | Y | 4 |  |
| BIDP5 | 매수호가5 | Number | Y | 4 |  |
| BIDP6 | 매수호가6 | Number | Y | 4 |  |
| BIDP7 | 매수호가7 | Number | Y | 4 |  |
| BIDP8 | 매수호가8 | Number | Y | 4 |  |
| BIDP9 | 매수호가9 | Number | Y | 4 |  |
| BIDP10 | 매수호가10 | Number | Y | 4 |  |
| ASKP_RSQN1 | 매도호가 잔량1 | Number | Y | 8 |  |
| ASKP_RSQN2 | 매도호가 잔량2 | Number | Y | 8 |  |
| ASKP_RSQN3 | 매도호가 잔량3 | Number | Y | 8 |  |
| ASKP_RSQN4 | 매도호가 잔량4 | Number | Y | 8 |  |
| ASKP_RSQN5 | 매도호가 잔량5 | Number | Y | 8 |  |
| ASKP_RSQN6 | 매도호가 잔량6 | Number | Y | 8 |  |
| ASKP_RSQN7 | 매도호가 잔량7 | Number | Y | 8 |  |
| ASKP_RSQN8 | 매도호가 잔량8 | Number | Y | 8 |  |
| ASKP_RSQN9 | 매도호가 잔량9 | Number | Y | 8 |  |
| ASKP_RSQN10 | 매도호가 잔량10 | Number | Y | 8 |  |
| BIDP_RSQN1 | 매수호가 잔량1 | Number | Y | 8 |  |
| BIDP_RSQN2 | 매수호가 잔량2 | Number | Y | 8 |  |
| BIDP_RSQN3 | 매수호가 잔량3 | Number | Y | 8 |  |
| BIDP_RSQN4 | 매수호가 잔량4 | Number | Y | 8 |  |
| BIDP_RSQN5 | 매수호가 잔량5 | Number | Y | 8 |  |
| BIDP_RSQN6 | 매수호가 잔량6 | Number | Y | 8 |  |
| BIDP_RSQN7 | 매수호가 잔량7 | Number | Y | 8 |  |
| BIDP_RSQN8 | 매수호가 잔량8 | Number | Y | 8 |  |
| BIDP_RSQN9 | 매수호가 잔량9 | Number | Y | 8 |  |
| BIDP_RSQN10 | 매수호가 잔량10 | Number | Y | 8 |  |
| TOTAL_ASKP_RSQN | 총 매도호가 잔량 | Number | Y | 8 |  |
| TOTAL_BIDP_RSQN | 총 매수호가 잔량 | Number | Y | 8 |  |
| OVTM_TOTAL_ASKP_RSQN | 시간외 총 매도호가 잔량 | Number | Y | 8 |  |
| OVTM_TOTAL_BIDP_RSQN | 시간외 총 매수호가 잔량 | Number | Y | 8 |  |
| ANTC_CNPR | 예상 체결가 | Number | Y | 4 | 동시호가 등 특정 조건하에서만 발생 |
| ANTC_CNQN | 예상 체결량 | Number | Y | 8 | 동시호가 등 특정 조건하에서만 발생 |
| ANTC_VOL | 예상 거래량 | Number | Y | 8 | 동시호가 등 특정 조건하에서만 발생 |
| ANTC_CNTG_VRSS | 예상 체결 대비 | Number | Y | 4 | 동시호가 등 특정 조건하에서만 발생 |
| ANTC_CNTG_VRSS_SIGN | 예상 체결 대비 부호 | String | Y | 1 | 동시호가 등 특정 조건하에서만 발생1 : 상한2 : 상승3 : 보합4 : 하한5 : 하락 |
| ANTC_CNTG_PRDY_CTRT | 예상 체결 전일 대비율 | Number | Y | 8 |  |
| ACML_VOL | 누적 거래량 | Number | Y | 8 |  |
| TOTAL_ASKP_RSQN_ICDC | 총 매도호가 잔량 증감 | Number | Y | 4 |  |
| TOTAL_BIDP_RSQN_ICDC | 총 매수호가 잔량 증감 | Number | Y | 4 |  |
| OVTM_TOTAL_ASKP_ICDC | 시간외 총 매도호가 증감 | Number | Y | 4 |  |
| OVTM_TOTAL_BIDP_ICDC | 시간외 총 매수호가 증감 | Number | Y | 4 |  |
| STCK_DEAL_CLS_CODE | 주식 매매 구분 코드 | String | Y | 2 | 사용 X (삭제된 값) |
