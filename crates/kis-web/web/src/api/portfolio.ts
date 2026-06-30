// 포트폴리오(잔고) 조회.
import { useQuery } from '@tanstack/react-query'
import { client } from './client'
import type { components } from './schema'

export type Portfolio = components['schemas']['Portfolio']
export type Holding = components['schemas']['Holding']
export type Summary = components['schemas']['Summary']

/** 잔고 조회. enabled=false 면 자격증명 미등록 등으로 호출 보류. */
export function usePortfolio(enabled: boolean) {
  return useQuery<Portfolio>({
    queryKey: ['portfolio', 'balance'],
    enabled,
    retry: false,
    refetchInterval: 30_000, // 30초 폴링 (P3에서 SSE로 교체 예정)
    queryFn: async () => {
      const res = await client.GET('/portfolio/balance')
      if (!res.response.ok || !res.data) {
        // 백엔드가 평문 메시지를 내려줌 (502: KIS 조회 실패, 409: 미등록 등)
        const body = (res.error ?? res.data) as unknown
        const msg =
          typeof body === 'string' && body
            ? body
            : `잔고 조회 실패 (HTTP ${res.response.status})`
        throw new Error(msg)
      }
      return res.data as Portfolio
    },
  })
}
