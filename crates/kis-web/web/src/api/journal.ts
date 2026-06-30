// 매매일지 훅.
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { client } from './client'
import type { components } from './schema'

export type Trade = components['schemas']['TradeDto']
export type TradeInput = components['schemas']['TradeInput']
export type TradeStats = components['schemas']['TradeStatsDto']

export type TradeFilter = {
  symbol?: string
  side?: string
  broker?: string
  from?: string
  to?: string
}

function errMessage(error: unknown, fallback: string): string {
  if (typeof error === 'string') return error
  if (error && typeof error === 'object' && 'message' in error) {
    return String((error as { message: unknown }).message)
  }
  return fallback
}

export function useTrades(filter: TradeFilter) {
  const query = Object.fromEntries(
    Object.entries(filter).filter(([, v]) => v != null && v !== ''),
  )
  return useQuery<Trade[]>({
    queryKey: ['trades', query],
    queryFn: async () => {
      const { data, error } = await client.GET('/trades', { params: { query } })
      if (error) throw new Error(errMessage(error, '조회 실패'))
      return (data as Trade[]) ?? []
    },
  })
}

export function useTradeStats(filter: { from?: string; to?: string } = {}) {
  const query = Object.fromEntries(
    Object.entries(filter).filter(([, v]) => v != null && v !== ''),
  )
  return useQuery<TradeStats>({
    queryKey: ['trades', 'stats', query],
    queryFn: async () => {
      const { data, error } = await client.GET('/trades/stats', { params: { query } })
      if (error) throw new Error(errMessage(error, '통계 조회 실패'))
      return data as TradeStats
    },
  })
}

export function useCreateTrade() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async (body: TradeInput) => {
      const { data, error } = await client.POST('/trades', { body })
      if (error) throw new Error(errMessage(error, '저장 실패'))
      return data as Trade
    },
    onSuccess: () => invalidate(qc),
  })
}

export function useUpdateTrade() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async ({ id, body }: { id: string; body: TradeInput }) => {
      const { data, error } = await client.PUT('/trades/{id}', {
        params: { path: { id } },
        body,
      })
      if (error) throw new Error(errMessage(error, '수정 실패'))
      return data as Trade
    },
    onSuccess: () => invalidate(qc),
  })
}

export function useDeleteTrade() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async (id: string) => {
      const { error } = await client.DELETE('/trades/{id}', { params: { path: { id } } })
      if (error) throw new Error(errMessage(error, '삭제 실패'))
    },
    onSuccess: () => invalidate(qc),
  })
}

function invalidate(qc: ReturnType<typeof useQueryClient>) {
  qc.invalidateQueries({ queryKey: ['trades'] })
  qc.invalidateQueries({ queryKey: ['portfolio'] }) // 수동 포지션도 영향
}
