// KIS 자격증명 상태 + 등록/삭제.
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { client } from './client'
import type { components } from './schema'

export type KisStatus = components['schemas']['KisCredentialsStatus']

const STATUS_KEY = ['account', 'kis-status'] as const

function errMessage(error: unknown, fallback: string): string {
  if (typeof error === 'string') return error
  if (error && typeof error === 'object' && 'message' in error) {
    return String((error as { message: unknown }).message)
  }
  return fallback
}

export function useKisStatus() {
  return useQuery<KisStatus>({
    queryKey: STATUS_KEY,
    queryFn: async () => {
      const { data, error } = await client.GET('/account/kis-credentials/status')
      if (error) throw new Error(errMessage(error, '상태 조회 실패'))
      return data as KisStatus
    },
  })
}

export function useSaveKisCredentials() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async (body: {
      app_key: string
      app_secret: string
      account_number: string
      is_mock: boolean
    }) => {
      const { data, error } = await client.PUT('/account/kis-credentials', { body })
      if (error) throw new Error(errMessage(error, '자격증명 저장 실패'))
      return data as KisStatus
    },
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: STATUS_KEY })
      qc.invalidateQueries({ queryKey: ['portfolio'] })
    },
  })
}

export function useDeleteKisCredentials() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async () => {
      const { error } = await client.DELETE('/account/kis-credentials')
      if (error) throw new Error(errMessage(error, '자격증명 삭제 실패'))
    },
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: STATUS_KEY })
      qc.invalidateQueries({ queryKey: ['portfolio'] })
    },
  })
}
