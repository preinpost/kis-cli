<!-- endpoint: /uapi/hashkey -->
<!-- category: OAuth인증 -->
<!-- korean_name: Hashkey -->

# Hashkey

## Info
- **Method**: POST
- **URL**: /uapi/hashkey
- **실전Domain**: https://openapi.koreainvestment.com:9443
- **모의Domain**: https://openapivts.koreainvestment.com:29443
- **Format**: JSON
- **Content-Type**: application/json

## 개요
해쉬키(Hashkey)는 보안을 위한 요소로 사용자가 보낸 요청 값을 중간에 탈취하여 변조하지 못하도록 하는데 사용됩니다.
해쉬키를 사용하면 POST로 보내는 요청(주로 주문/정정/취소 API 해당)의 body 값을 사전에 암호화시킬 수 있습니다.
해쉬키는 비필수값으로 사용하지 않아도 POST API 호출은 가능합니다.

## Request Header
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| content-type | 컨텐츠타입 | String | N | 40 | application/json; charset=utf-8 |
| appkey | 앱키 | String | Y | 36 | 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.) |
| appsecret | 앱시크릿키 | String | Y | 180 | 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.) |

## Request Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| JsonBody | 요청값 | Object | Y | - | POST로 보낼 body값ex)datas = { "CANO": '00000000', "ACNT_PRDT_CD": "01", "OVRS_EXCG_CD": "SHAA" } |

## Response Body
| Element | 한글명 | Type | Required | Length | Description |
| --- | --- | --- | --- | --- | --- |
| JsonBody | 요청값 | Object | Y | - | 요청한 JsonBody |
| HASH | 해쉬키 | String | Y | 256 | [POST API 대상] Client가 요청하는 Request Body를 hashkey api로 생성한 Hash값* API문서 > hashkey 참조 |
