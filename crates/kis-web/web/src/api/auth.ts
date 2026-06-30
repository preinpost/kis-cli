// 인증 상태 + 액션 (TanStack Query 래핑).
// 세션은 HttpOnly 쿠키라 JS에서 토큰을 만지지 않는다 — /auth/me 로 로그인 여부를 판별.
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { client } from './client'
import type { components } from './schema'

export type User = components['schemas']['UserDto']

const ME_KEY = ['auth', 'me'] as const

// openapi-fetch 의 error 는 백엔드가 내려준 본문(평문 메시지). 사람이 읽을 문자열로 변환.
function errMessage(error: unknown, fallback: string): string {
  if (typeof error === 'string') return error
  if (error && typeof error === 'object' && 'message' in error) {
    return String((error as { message: unknown }).message)
  }
  return fallback
}

/** 현재 로그인 사용자. 비로그인(401)이면 null. */
export function useMe() {
  return useQuery<User | null>({
    queryKey: ME_KEY,
    queryFn: async () => {
      const { data, response } = await client.GET('/auth/me')
      if (response.status === 401) return null
      return (data as User) ?? null
    },
    retry: false,
    staleTime: 30_000,
  })
}

export function useLogin() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async (body: { username: string; password: string }) => {
      const { data, error } = await client.POST('/auth/login', { body })
      if (error) throw new Error(errMessage(error, '로그인에 실패했습니다'))
      return data as User
    },
    onSuccess: (user) => {
      qc.setQueryData(ME_KEY, user)
    },
  })
}

export function useRegister() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async (body: {
      username: string
      display_name: string
      password: string
    }) => {
      const { data, error } = await client.POST('/auth/register', { body })
      if (error) throw new Error(errMessage(error, '회원가입에 실패했습니다'))
      return data as User
    },
    onSuccess: (user) => {
      qc.setQueryData(ME_KEY, user)
    },
  })
}

export function useLogout() {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async () => {
      await client.POST('/auth/logout')
    },
    onSuccess: () => {
      qc.setQueryData(ME_KEY, null)
      qc.clear()
    },
  })
}
