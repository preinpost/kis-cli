<!-- endpoint: /tryitout/H0STCNT0 -->
<!-- category: [국내주식] 실시간시세 -->
<!-- korean_name: 국내주식 실시간체결가 (KRX) -->

# 국내주식 실시간체결가 (KRX) [실시간-003]

## Info
- **Method**: POST
- **URL**: /tryitout/H0STCNT0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: ws://ops.koreainvestment.com:31000
- **실전TRID**: H0STCNT0
- **모의TRID**: H0STCNT0
- **Format**: JSON
- **Content-Type**: text/plain

## 개요
[참고자료]
실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
[호출 데이터]
헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
[응답 데이터]
1. 정상 등록 여부 (JSON)
- JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
- JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
- JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
2. 실시간 결과 응답 ( | 로 구분되는 값)
ex) 0|H0STCNT0|004|005930^123929^73100^5^...
- 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
- TR_ID : 등록한 tr_id (ex. H0STCNT0)
- 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
- 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
※ 데이터가 많은 경우 여러 건을 페이징 처리해서 데이터를 보내는 점 참고 부탁드립니다.
ex) 0|H0STCNT0|004|... 인 경우 004가 데이터 개수를 의미하여, 뒤에 체결데이터가 4건 들어옴
→ 0|H0STCNT0|004|005930^123929...(체결데이터1)...^005930^123929...(체결데이터2)...^005930^123929...(체결데이터3)...^005930^123929...(체결데이터4)...

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
| tr_id | 거래ID | String | Y | 1 | [실전/모의투자]H0STCNT0 : 실시간 주식 체결가 |
| tr_key | 구분값 | String | Y | 1 | 종목번호 (6자리)ETN의 경우, Q로 시작 (EX. Q500001) |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| MKSC_SHRN_ISCD | 유가증권 단축 종목코드 | String | Y | 9 |  |
| STCK_CNTG_HOUR | 주식 체결 시간 | String | Y | 6 |  |
| STCK_PRPR | 주식 현재가 | Number | Y | 4 | 체결가격 |
| PRDY_VRSS_SIGN | 전일 대비 부호 | String | Y | 1 | 1 : 상한2 : 상승3 : 보합4 : 하한5 : 하락 |
| PRDY_VRSS | 전일 대비 | Number | Y | 4 |  |
| PRDY_CTRT | 전일 대비율 | Number | Y | 8 |  |
| WGHN_AVRG_STCK_PRC | 가중 평균 주식 가격 | Number | Y | 8 |  |
| STCK_OPRC | 주식 시가 | Number | Y | 4 |  |
| STCK_HGPR | 주식 최고가 | Number | Y | 4 |  |
| STCK_LWPR | 주식 최저가 | Number | Y | 4 |  |
| ASKP1 | 매도호가1 | Number | Y | 4 |  |
| BIDP1 | 매수호가1 | Number | Y | 4 |  |
| CNTG_VOL | 체결 거래량 | Number | Y | 8 |  |
| ACML_VOL | 누적 거래량 | Number | Y | 8 |  |
| ACML_TR_PBMN | 누적 거래 대금 | Number | Y | 8 |  |
| SELN_CNTG_CSNU | 매도 체결 건수 | Number | Y | 4 |  |
| SHNU_CNTG_CSNU | 매수 체결 건수 | Number | Y | 4 |  |
| NTBY_CNTG_CSNU | 순매수 체결 건수 | Number | Y | 4 |  |
| CTTR | 체결강도 | Number | Y | 8 |  |
| SELN_CNTG_SMTN | 총 매도 수량 | Number | Y | 8 |  |
| SHNU_CNTG_SMTN | 총 매수 수량 | Number | Y | 8 |  |
| CCLD_DVSN | 체결구분 | String | Y | 1 | 1:매수(+) 3:장전 5:매도(-) |
| SHNU_RATE | 매수비율 | Number | Y | 8 |  |
| PRDY_VOL_VRSS_ACML_VOL_RATE | 전일 거래량 대비 등락율 | Number | Y | 8 |  |
| OPRC_HOUR | 시가 시간 | String | Y | 6 |  |
| OPRC_VRSS_PRPR_SIGN | 시가대비구분 | String | Y | 1 | 1 : 상한2 : 상승3 : 보합4 : 하한5 : 하락 |
| OPRC_VRSS_PRPR | 시가대비 | Number | Y | 4 |  |
| HGPR_HOUR | 최고가 시간 | String | Y | 6 |  |
| HGPR_VRSS_PRPR_SIGN | 고가대비구분 | String | Y | 1 | 1 : 상한2 : 상승3 : 보합4 : 하한5 : 하락 |
| HGPR_VRSS_PRPR | 고가대비 | Number | Y | 4 |  |
| LWPR_HOUR | 최저가 시간 | String | Y | 6 |  |
| LWPR_VRSS_PRPR_SIGN | 저가대비구분 | String | Y | 1 | 1 : 상한2 : 상승3 : 보합4 : 하한5 : 하락 |
| LWPR_VRSS_PRPR | 저가대비 | Number | Y | 4 |  |
| BSOP_DATE | 영업 일자 | String | Y | 8 |  |
| NEW_MKOP_CLS_CODE | 신 장운영 구분 코드 | String | Y | 2 | (1) 첫 번째 비트1 : 장개시전2 : 장중3 : 장종료후4 : 시간외단일가7 : 일반Buy-in8 : 당일Buy-in(2) 두 번째 비트0 : 보통1 : 종가2 : 대량3 : 바스켓7 : 정리매매8 : Buy-in |
| TRHT_YN | 거래정지 여부 | String | Y | 1 | Y : 정지N : 정상거래 |
| ASKP_RSQN1 | 매도호가 잔량1 | Number | Y | 8 |  |
| BIDP_RSQN1 | 매수호가 잔량1 | Number | Y | 8 |  |
| TOTAL_ASKP_RSQN | 총 매도호가 잔량 | Number | Y | 8 |  |
| TOTAL_BIDP_RSQN | 총 매수호가 잔량 | Number | Y | 8 |  |
| VOL_TNRT | 거래량 회전율 | Number | Y | 8 |  |
| PRDY_SMNS_HOUR_ACML_VOL | 전일 동시간 누적 거래량 | Number | Y | 8 |  |
| PRDY_SMNS_HOUR_ACML_VOL_RATE | 전일 동시간 누적 거래량 비율 | Number | Y | 8 |  |
| HOUR_CLS_CODE | 시간 구분 코드 | String | Y | 1 | 0 : 장중A : 장후예상B : 장전예상C : 9시이후의 예상가, VI발동D : 시간외 단일가 예상 |
| MRKT_TRTM_CLS_CODE | 임의종료구분코드 | String | Y | 1 |  |
| VI_STND_PRC | 정적VI발동기준가 | Number | Y | 4 |  |
