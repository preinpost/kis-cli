import { useEffect, useState } from 'react'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { Field } from '@base-ui-components/react/field'
import { useLogin, useRegister, useMe } from '../api/auth'

export const Route = createFileRoute('/login')({
  component: AuthPage,
})

type Mode = 'login' | 'register'

function AuthPage() {
  const navigate = useNavigate()
  const me = useMe()
  const [mode, setMode] = useState<Mode>('login')

  const login = useLogin()
  const register = useRegister()

  // 이미 로그인 상태면 대시보드로.
  useEffect(() => {
    if (me.data) navigate({ to: '/' })
  }, [me.data, navigate])

  const pending = login.isPending || register.isPending
  const error =
    (login.error as Error | null)?.message ??
    (register.error as Error | null)?.message ??
    null

  function onSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault()
    const fd = new FormData(e.currentTarget)
    const username = String(fd.get('username') ?? '').trim()
    const password = String(fd.get('password') ?? '')
    if (mode === 'login') {
      login.mutate({ username, password }, { onSuccess: () => navigate({ to: '/' }) })
    } else {
      const display_name = String(fd.get('display_name') ?? '').trim()
      register.mutate(
        { username, display_name, password },
        { onSuccess: () => navigate({ to: '/' }) },
      )
    }
  }

  const inputClass =
    'w-full rounded-lg border border-neutral-300 bg-white px-3 py-2 text-sm outline-none focus:border-neutral-900 focus:ring-1 focus:ring-neutral-900'

  return (
    <div className="mx-auto max-w-sm">
      {/* 로그인 / 회원가입 토글 */}
      <div className="mb-6 flex rounded-lg border border-neutral-200 bg-neutral-100 p-1 text-sm">
        <button
          type="button"
          onClick={() => setMode('login')}
          className={`flex-1 rounded-md px-3 py-1.5 font-medium transition ${
            mode === 'login' ? 'bg-white shadow-sm' : 'text-neutral-500'
          }`}
        >
          로그인
        </button>
        <button
          type="button"
          onClick={() => setMode('register')}
          className={`flex-1 rounded-md px-3 py-1.5 font-medium transition ${
            mode === 'register' ? 'bg-white shadow-sm' : 'text-neutral-500'
          }`}
        >
          회원가입
        </button>
      </div>

      <form
        className="space-y-4 rounded-xl border border-neutral-200 bg-white p-6 shadow-sm"
        onSubmit={onSubmit}
      >
        <Field.Root className="space-y-1.5">
          <Field.Label className="block text-sm font-medium text-neutral-700">
            사용자 이름
          </Field.Label>
          <Field.Control
            type="text"
            name="username"
            required
            autoComplete="username"
            placeholder="username"
            className={inputClass}
          />
        </Field.Root>

        {mode === 'register' && (
          <Field.Root className="space-y-1.5">
            <Field.Label className="block text-sm font-medium text-neutral-700">
              표시 이름 <span className="text-neutral-400">(선택)</span>
            </Field.Label>
            <Field.Control
              type="text"
              name="display_name"
              autoComplete="name"
              placeholder="민수"
              className={inputClass}
            />
          </Field.Root>
        )}

        <Field.Root className="space-y-1.5">
          <Field.Label className="block text-sm font-medium text-neutral-700">
            비밀번호
          </Field.Label>
          <Field.Control
            type="password"
            name="password"
            required
            minLength={mode === 'register' ? 8 : undefined}
            autoComplete={mode === 'register' ? 'new-password' : 'current-password'}
            placeholder="••••••••"
            className={inputClass}
          />
          {mode === 'register' && (
            <p className="text-xs text-neutral-400">8자 이상</p>
          )}
        </Field.Root>

        {error && (
          <p className="rounded-lg bg-red-50 px-3 py-2 text-sm text-red-600">{error}</p>
        )}

        <button
          type="submit"
          disabled={pending}
          className="w-full rounded-lg bg-neutral-900 px-3 py-2 text-sm font-medium text-white hover:bg-neutral-800 disabled:opacity-50"
        >
          {pending ? '처리 중…' : mode === 'login' ? '로그인' : '회원가입'}
        </button>

        <div className="flex items-center gap-3 text-xs text-neutral-400">
          <span className="h-px flex-1 bg-neutral-200" />
          또는
          <span className="h-px flex-1 bg-neutral-200" />
        </div>

        {/* 패스키는 다음 단계(P1 후속)에서 활성화 */}
        <button
          type="button"
          disabled
          title="곧 지원 예정"
          className="w-full cursor-not-allowed rounded-lg border border-neutral-200 bg-white px-3 py-2 text-sm font-medium text-neutral-400"
        >
          패스키로 로그인 (준비 중)
        </button>
      </form>
    </div>
  )
}
