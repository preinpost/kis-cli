// 통합 포지션 + 종목 메타 훅.
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { client } from './client'
import type { components } from './schema'

export type Position = components['schemas']['PositionDto']
export type Positions = components['schemas']['Positions']
export type MetaInput = components['schemas']['MetaInput']

function errMessage(error: unknown, fallback: string): string {
  if (typeof error === 'string') return error
  if (error && typeof error === 'object' && 'message' in error) {
    return String((error as { message: unknown }).message)
  }
  return fallback
}

export function usePositions(enabled = true) {
  return useQuery<Positions>({
    queryKey: ['portfolio', 'positions'],
    enabled,
    refetchInterval: 30_000,
    queryFn: async () => {
      const { data, error } = await client.GET('/portfolio/positions')
      if (error) throw new Error(errMessage(error, '포지션 조회 실패'))
      return data as Positions
    },
  })
}

export function useSaveMeta() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async ({ symbol, body }: { symbol: string; body: MetaInput }) => {
      const { error } = await client.PUT('/portfolio/meta/{symbol}', {
        params: { path: { symbol } },
        body,
      })
      if (error) throw new Error(errMessage(error, '메타 저장 실패'))
    },
    onSuccess: () => qc.invalidateQueries({ queryKey: ['portfolio', 'positions'] }),
  })
}
