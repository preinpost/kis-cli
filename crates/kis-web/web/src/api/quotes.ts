// 현재가 + 종목검색 + 관심종목.
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { client } from './client'
import type { components } from './schema'

export type Quote = components['schemas']['Quote']
export type SymbolDto = components['schemas']['SymbolDto']
export type WatchlistItem = components['schemas']['WatchlistItem']

function errMessage(error: unknown, fallback: string): string {
  if (typeof error === 'string') return error
  if (error && typeof error === 'object' && 'message' in error) {
    return String((error as { message: unknown }).message)
  }
  return fallback
}

/** 단일 종목 현재가. 30초 폴링. */
export function useQuote(symbol: string, enabled = true) {
  return useQuery<Quote>({
    queryKey: ['quote', symbol],
    enabled: enabled && !!symbol,
    retry: false,
    refetchInterval: 30_000,
    queryFn: async () => {
      const res = await client.GET('/quotes/{symbol}', {
        params: { path: { symbol } },
      })
      if (!res.response.ok || !res.data) {
        const body = (res.error ?? res.data) as unknown
        throw new Error(
          typeof body === 'string' && body
            ? body
            : `시세 조회 실패 (HTTP ${res.response.status})`,
        )
      }
      return res.data as Quote
    },
  })
}

export type Spark = components['schemas']['Spark']

/** 미니 차트용 일봉 종가 시계열. 일봉이라 staleTime 길게(10분). */
export function useSpark(symbol: string, enabled = true) {
  return useQuery<Spark>({
    queryKey: ['spark', symbol],
    enabled: enabled && !!symbol,
    retry: false,
    staleTime: 10 * 60_000,
    queryFn: async () => {
      const { data, error } = await client.GET('/quotes/{symbol}/spark', {
        params: { path: { symbol } },
      })
      if (error) throw new Error(errMessage(error, '차트 조회 실패'))
      return data as Spark
    },
  })
}

/** 종목 검색 (디바운스는 호출부에서). */
export function useSymbolSearch(q: string) {
  return useQuery<SymbolDto[]>({
    queryKey: ['symbols', 'search', q],
    enabled: q.trim().length > 0,
    queryFn: async () => {
      const { data, error } = await client.GET('/symbols/search', {
        params: { query: { q, limit: 10 } },
      })
      if (error) throw new Error(errMessage(error, '검색 실패'))
      return (data as SymbolDto[]) ?? []
    },
  })
}

export function useWatchlist() {
  return useQuery<WatchlistItem[]>({
    queryKey: ['watchlist'],
    queryFn: async () => {
      const { data, error } = await client.GET('/watchlist')
      if (error) throw new Error(errMessage(error, '관심종목 조회 실패'))
      return (data as WatchlistItem[]) ?? []
    },
  })
}

export function useAddWatch() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async (symbol: string) => {
      const { data, error } = await client.POST('/watchlist', {
        body: { symbol },
      })
      if (error) throw new Error(errMessage(error, '추가 실패'))
      return data as WatchlistItem
    },
    onSuccess: () => qc.invalidateQueries({ queryKey: ['watchlist'] }),
  })
}

export function useRemoveWatch() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async (symbol: string) => {
      const { error } = await client.DELETE('/watchlist/{symbol}', {
        params: { path: { symbol } },
      })
      if (error) throw new Error(errMessage(error, '삭제 실패'))
    },
    onSuccess: () => qc.invalidateQueries({ queryKey: ['watchlist'] }),
  })
}

/** 종목 마스터 수동 동기화. */
export function useSyncSymbols() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async () => {
      const { data, error } = await client.POST('/symbols/sync')
      if (error) throw new Error(errMessage(error, '동기화 실패'))
      return data as { synced: number }
    },
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['symbols'] })
      qc.invalidateQueries({ queryKey: ['watchlist'] })
    },
  })
}
