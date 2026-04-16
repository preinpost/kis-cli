<!-- endpoint: /tryitout/H0IOASP0 -->
<!-- category: [국내선물옵션] 실시간시세 -->
<!-- korean_name: 지수옵션 실시간호가 -->

# 지수옵션 실시간호가[실시간-015]

## Info
- **Method**: POST
- **URL**: /tryitout/H0IOASP0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: 모의투자 미지원
- **실전TRID**: H0IOASP0
- **모의TRID**: 모의투자 미지원

## 개요
[참고자료]
실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓접속키 | String | Y | 36 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| tr_type | 등록/해제 | String | Y | 1 | "1: 등록, 2:해제" |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 / P : 개인 |
| content-type | 컨텐츠타입 | String | Y | 20 | utf-8 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 7 | H0IOASP0 |
| tr_key | 코드 | String | Y | 6 | 예:201S11305 |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| OPTN_SHRN_ISCD | 옵션 단축 종목코드 | Object | Y | 16 | '각 항목사이에는 구분자로 ^ 사용,모든 데이터타입은 String으로 변환되어 push 처리됨' |
| BSOP_HOUR | 영업 시간 | String | Y | 16 |  |
| OPTN_ASKP1 | 옵션 매도호가1 | String | Y | 1 |  |
| OPTN_ASKP2 | 옵션 매도호가2 | String | Y | 8 |  |
| OPTN_ASKP3 | 옵션 매도호가3 | String | Y | 8 |  |
| OPTN_ASKP4 | 옵션 매도호가4 | String | Y | 8 |  |
| OPTN_ASKP5 | 옵션 매도호가5 | String | Y | 8 |  |
| OPTN_BIDP1 | 옵션 매수호가1 | String | Y | 8 |  |
| OPTN_BIDP2 | 옵션 매수호가2 | String | Y | 8 |  |
| OPTN_BIDP3 | 옵션 매수호가3 | String | Y | 8 |  |
| OPTN_BIDP4 | 옵션 매수호가4 | String | Y | 8 |  |
| OPTN_BIDP5 | 옵션 매수호가5 | String | Y | 8 |  |
| ASKP_CSNU1 | 매도호가 건수1 | String | Y | 8 |  |
| ASKP_CSNU2 | 매도호가 건수2 | String | Y | 8 |  |
| ASKP_CSNU3 | 매도호가 건수3 | String | Y | 8 |  |
| ASKP_CSNU4 | 매도호가 건수4 | String | Y | 8 |  |
| ASKP_CSNU5 | 매도호가 건수5 | String | Y | 8 |  |
| BIDP_CSNU1 | 매수호가 건수1 | String | Y | 8 |  |
| BIDP_CSNU2 | 매수호가 건수2 | String | Y | 8 |  |
| BIDP_CSNU3 | 매수호가 건수3 | String | Y | 8 |  |
| BIDP_CSNU4 | 매수호가 건수4 | String | Y | 8 |  |
| BIDP_CSNU5 | 매수호가 건수5 | String | Y | 8 |  |
| ASKP_RSQN1 | 매도호가 잔량1 | String | Y | 8 |  |
| ASKP_RSQN2 | 매도호가 잔량2 | String | Y | 8 |  |
| ASKP_RSQN3 | 매도호가 잔량3 | String | Y | 8 |  |
| ASKP_RSQN4 | 매도호가 잔량4 | String | Y | 8 |  |
| ASKP_RSQN5 | 매도호가 잔량5 | String | Y | 8 |  |
| BIDP_RSQN1 | 매수호가 잔량1 | String | Y | 8 |  |
| BIDP_RSQN2 | 매수호가 잔량2 | String | Y | 8 |  |
| BIDP_RSQN3 | 매수호가 잔량3 | String | Y | 8 |  |
| BIDP_RSQN4 | 매수호가 잔량4 | String | Y | 8 |  |
| BIDP_RSQN5 | 매수호가 잔량5 | String | Y | 8 |  |
| TOTAL_ASKP_CSNU | 총 매도호가 건수 | String | Y | 8 |  |
| TOTAL_BIDP_CSNU | 총 매수호가 건수 | String | Y | 8 |  |
| TOTAL_ASKP_RSQN | 총 매도호가 잔량 | String | Y | 8 |  |
| TOTAL_BIDP_RSQN | 총 매수호가 잔량 | String | Y | 8 |  |
| TOTAL_ASKP_RSQN_ICDC | 총 매도호가 잔량 증감 | String | Y | 8 |  |
| TOTAL_BIDP_RSQN_ICDC | 총 매수호가 잔량 증감 | String | Y | 8 |  |
