<!-- endpoint: /tryitout/H0STCNI0 -->
<!-- category: [국내주식] 실시간시세 -->
<!-- korean_name: 국내주식 실시간체결통보 -->

# 국내주식 실시간체결통보 [실시간-005]

## Info
- **Method**: POST
- **URL**: /tryitout/H0STCNI0
- **실전Domain**: ws://ops.koreainvestment.com:21000
- **모의Domain**: ws://ops.koreainvestment.com:31000
- **실전TRID**: H0STCNI0
- **모의TRID**: H0STCNI9
- **Content-Type**: test/plain

## 개요
국내주식 실시간 체결통보 수신 시에 (1) 주문·정정·취소·거부 접수 통보 와 (2) 체결 통보 가 모두 수신됩니다.
(14번째 값(CNTG_YN;체결여부)가 2이면 체결통보, 1이면 주문·정정·취소·거부 접수 통보입니다.)
※ 모의투자는 H0STCNI9 로 변경하여 사용합니다.
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
체결 통보 응답 결과는 암호화되어 출력됩니다. AES256 KEY IV를 활용해 복호화하여 활용하세요. 자세한 예제는 [도구>wikidocs]에 준비되어 있습니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| approval_key | 웹소켓 접속키 | String | N | 286 | 실시간 (웹소켓) 접속키 발급 API(/oauth2/Approval)를 사용하여 발급받은 웹소켓 접속키 |
| custtype | 고객 타입 | String | Y | 1 | B : 법인 P : 개인 |
| tr_type | 거래타입 | String | N | 1 | 1: 등록 2 : 해제 |
| content-type | 컨텐츠타입 | String | N | 1 | utf-8 |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| tr_id | 거래ID | String | Y | 2 | '[실전/모의투자]H0STCNI0 : 국내주식 실시간체결통보H0STCNI9 : 모의투자 실시간 체결통보 |
| tr_key | 구분값 | String | Y | 12 | HTS ID |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| CUST_ID | 고객 ID | String | Y | 8 |  |
| ACNT_NO | 계좌번호 | String | Y | 10 |  |
| ODER_NO | 주문번호 | String | Y | 10 |  |
| OODER_NO | 원주문번호 | String | Y | 10 |  |
| SELN_BYOV_CLS | 매도매수구분 | String | Y | 2 | 01 : 매도 02 : 매수 |
| RCTF_CLS | 정정구분 | String | Y | 1 | 0:정상 1:정정 2:취소 |
| ODER_KIND | 주문종류 | String | Y | 2 | [KRX]00 : 지정가01 : 시장가02 : 조건부지정가03 : 최유리지정가04 : 최우선지정가05 : 장전 시간외06 : 장후 시간외07 : 시간외 단일가11 : IOC지정가 (즉시체결,잔량취소)12 : FOK지정가 (즉시체결,전량취소)13 : IOC시장가 (즉시체결,잔량취소)14 : FOK시장가 (즉시체결,전량취소)15 : IOC최유리 (즉시체결,잔량취소)16 : FOK최유리 (즉시체결,전량취소)21 : 중간가22 : 스톱지정가23 : 중간가IOC24 : 중간가FOK[NXT]00 : 지정가03 : 최유리지정가04 : 최우선지정가11 : IOC지정가 (즉시체결,잔량취소)12 : FOK지정가 (즉시체결,전량취소)13 : IOC시장가 (즉시체결,잔량취소)14 : FOK시장가 (즉시체결,전량취소)15 : IOC최유리 (즉시체결,잔량취소)16 : FOK최유리 (즉시체결,전량취소)21 : 중간가22 : 스톱지정가23 : 중간가IOC24 : 중간가FOK[SOR]00 : 지정가01 : 시장가03 : 최유리지정가04 : 최우선지정가11 : IOC지정가 (즉시체결,잔량취소)12 : FOK지정가 (즉시체결,전량취소)13 : IOC시장가 (즉시체결,잔량취소)14 : FOK시장가 (즉시체결,전량취소)15 : IOC최유리 (즉시체결,잔량취소)16 : FOK최유리 (즉시체결,전량취소) |
| ODER_COND | 주문조건 | String | Y | 1 | 0:없음1:IOC 2:FOK |
| STCK_SHRN_ISCD | 주식 단축 종목코드 | String | Y | 9 |  |
| CNTG_QTY | 체결 수량 | String | Y | 10 |  |
| CNTG_UNPR | 체결단가 | String | Y | 9 |  |
| STCK_CNTG_HOUR | 주식 체결 시간 | String | Y | 6 |  |
| RFUS_YN | 거부여부 | String | Y | 1 | 0 : 승인 1 : 거부 |
| CNTG_YN | 체결여부 | String | Y | 1 | 1 : 주문,정정,취소,거부2 : 체결 |
| ACPT_YN | 접수여부 | String | Y | 1 | 1 : 주문접수2 : 확인3 : 취소(FOK/IOC) |
| BRNC_NO | 지점번호 | String | Y | 5 |  |
| ODER_QTY | 주문수량 | String | Y | 9 |  |
| ACNT_NAME | 계좌명 | String | Y | 12 |  |
| ORD_COND_PRC | 호가조건가격 | String | Y | 9 | 스톱지정가 시 표시 |
| ORD_EXG_GB | 주문거래소 구분 | String | Y | 1 | 1:KRX, 2:NXT, 3:SOR-KRX, 4:SOR-NXT |
| POPUP_YN | 실시간체결창 표시여부 | String | Y | 1 | Y/N |
| FILLER | 필러 | String | Y | 3 |  |
| CRDT_CLS | 신용구분 | String | Y | 2 |  |
| CRDT_LOAN_DATE | 신용대출일자 | String | Y | 8 |  |
| CNTG_ISNM40 | 체결종목명 | String | Y | 40 |  |
| ODER_PRC | 주문가격 | String | Y | 9 |  |
