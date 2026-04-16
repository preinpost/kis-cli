<!-- endpoint: /tryitout/HDFSASP1 -->
<!-- category: [해외주식] 실시간시세 -->
<!-- korean_name: 해외주식 지연호가(아시아) -->

# 해외주식 지연호가(아시아)[실시간-008]

## Info
- **Method**: POST
- **URL**: /tryitout/HDFSASP1
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 제공 안함
- **실전TRID**: HDFSASP1
- **모의TRID**: 모의투자 미지원

## 개요
해외주식 지연호가(아시아)의 경우 아시아 무료시세(지연호가)가 제공됩니다.
HTS(efriend Plus) [7781] 시세신청(실시간) 화면에서 유료 서비스 신청 시,
"해외주식 실시간호가 HDFSASP0" 을 이용하여 아시아국가 유료시세(실시간호가)를 받아보실 수 있습니다. (24.11.29 반영)
※ 지연시세 지연시간 : 홍콩, 베트남, 중국, 일본 - 15분지연
[참고자료]
실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓 접속키 | String | Y | 286 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 / P : 개인 |
| tr_type | 등록/해제 | String | Y | 1 | "1: 등록, 2:해제" |
| content-type | 컨텐츠타입 | String | Y | 20 | utf-8 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 7 | HDFSASP1 |
| tr_key | D거래소명종목코드 | String | Y | 6 | <아시아국가 - 무료시세>D+시장구분(3자리)+종목코드예) DHKS00003 : D+HKS(홍콩)+00003(홍콩중화가스)[시장구분]TSE : 도쿄, HKS : 홍콩,SHS : 상해, SZS : 심천HSX : 호치민, HNX : 하노이 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| RSYM | 실시간종목코드 | String | Y | 16 | '각 항목사이에는 구분자로 ^ 사용,모든 데이터타입은 String으로 변환되어 push 처리됨' |
| SYMB | 종목코드 | String | Y | 16 |  |
| ZDIV | 소수점자리수 | String | Y | 1 |  |
| XYMD | 현지일자 | String | Y | 8 |  |
| XHMS | 현지시간 | String | Y | 6 |  |
| KYMD | 한국일자 | String | Y | 8 |  |
| KHMS | 한국시간 | String | Y | 6 |  |
| BVOL | 매수총잔량 | String | Y | 10 |  |
| AVOL | 매도총잔량 | String | Y | 10 |  |
| BDVL | 매수총잔량대비 | String | Y | 10 |  |
| ADVL | 매도총잔량대비 | String | Y | 10 |  |
| PBID1 | 매수호가1 | String | Y | 12 |  |
| PASK1 | 매도호가1 | String | Y | 12 |  |
| VBID1 | 매수잔량1 | String | Y | 10 |  |
| VASK1 | 매도잔량1 | String | Y | 10 |  |
| DBID1 | 매수잔량대비1 | String | Y | 10 |  |
| DASK1 | 매도잔량대비1 | String | Y | 10 |  |
