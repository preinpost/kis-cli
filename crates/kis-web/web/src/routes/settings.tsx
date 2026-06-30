import { useEffect, useState } from 'react'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { Field } from '@base-ui-components/react/field'
import { Switch } from '@base-ui-components/react/switch'
import { useMe } from '../api/auth'
import {
  useKisStatus,
  useSaveKisCredentials,
  useDeleteKisCredentials,
} from '../api/account'

export const Route = createFileRoute('/settings')({
  component: Settings,
})

function Settings() {
  const navigate = useNavigate()
  const me = useMe()
  const status = useKisStatus()
  const save = useSaveKisCredentials()
  const del = useDeleteKisCredentials()
  const [isMock, setIsMock] = useState(true)

  useEffect(() => {
    if (!me.isPending && !me.data) navigate({ to: '/login' })
  }, [me.isPending, me.data, navigate])

  // 기존 모의 여부 반영
  useEffect(() => {
    if (status.data?.is_mock != null) setIsMock(status.data.is_mock)
  }, [status.data?.is_mock])

  const inputClass =
    'w-full rounded-lg border border-neutral-300 bg-white px-3 py-2 text-sm outline-none focus:border-neutral-900 focus:ring-1 focus:ring-neutral-900'

  function onSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault()
    const fd = new FormData(e.currentTarget)
    save.mutate(
      {
        app_key: String(fd.get('app_key') ?? '').trim(),
        app_secret: String(fd.get('app_secret') ?? '').trim(),
        account_number: String(fd.get('account_number') ?? '').trim(),
        is_mock: isMock,
      },
      { onSuccess: () => navigate({ to: '/' }) },
    )
  }

  const configured = status.data?.configured ?? false

  return (
    <div className="mx-auto max-w-lg space-y-6">
      <div>
        <h1 className="text-2xl font-semibold tracking-tight">설정</h1>
        <p className="mt-1 text-sm text-neutral-500">
          KIS Open API 자격증명. 시크릿은 암호화되어 저장되며 다시 표시되지 않습니다.
        </p>
      </div>

      {configured && (
        <div className="flex items-center justify-between rounded-lg border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm">
          <span className="text-emerald-700">
            등록됨 · 계좌 {status.data?.account_number} ·{' '}
            {status.data?.is_mock ? '모의투자' : '실거래'}
          </span>
          <button
            type="button"
            onClick={() => del.mutate()}
            disabled={del.isPending}
            className="rounded-md border border-red-300 px-2.5 py-1 font-medium text-red-600 hover:bg-red-50 disabled:opacity-50"
          >
            삭제
          </button>
        </div>
      )}

      <form
        className="space-y-4 rounded-xl border border-neutral-200 bg-white p-6 shadow-sm"
        onSubmit={onSubmit}
      >
        <Field.Root className="space-y-1.5">
          <Field.Label className="block text-sm font-medium text-neutral-700">
            App Key
          </Field.Label>
          <Field.Control
            type="text"
            name="app_key"
            required
            autoComplete="off"
            placeholder="PSxxxxxxxx..."
            className={inputClass}
          />
        </Field.Root>

        <Field.Root className="space-y-1.5">
          <Field.Label className="block text-sm font-medium text-neutral-700">
            App Secret
          </Field.Label>
          <Field.Control
            type="password"
            name="app_secret"
            required
            autoComplete="off"
            placeholder="••••••••"
            className={inputClass}
          />
        </Field.Root>

        <Field.Root className="space-y-1.5">
          <Field.Label className="block text-sm font-medium text-neutral-700">
            계좌번호
          </Field.Label>
          <Field.Control
            type="text"
            name="account_number"
            required
            autoComplete="off"
            placeholder="12345678-01"
            defaultValue={status.data?.account_number ?? ''}
            className={inputClass}
          />
          <p className="text-xs text-neutral-400">예: 12345678-01 (계좌 8자리-상품 2자리)</p>
        </Field.Root>

        <label className="flex items-center justify-between rounded-lg bg-neutral-50 px-3 py-2.5">
          <span className="text-sm font-medium text-neutral-700">
            모의투자 계좌
            <span className="ml-2 text-xs font-normal text-neutral-400">
              {isMock ? '(openapivts)' : '(실거래 openapi)'}
            </span>
          </span>
          <Switch.Root
            checked={isMock}
            onCheckedChange={setIsMock}
            className="relative h-6 w-11 rounded-full bg-neutral-300 transition data-[checked]:bg-neutral-900"
          >
            <Switch.Thumb className="block h-5 w-5 translate-x-0.5 rounded-full bg-white transition-transform data-[checked]:translate-x-[22px]" />
          </Switch.Root>
        </label>

        {(save.error || del.error) && (
          <p className="rounded-lg bg-red-50 px-3 py-2 text-sm text-red-600">
            {(save.error as Error)?.message ?? (del.error as Error)?.message}
          </p>
        )}

        <button
          type="submit"
          disabled={save.isPending}
          className="w-full rounded-lg bg-neutral-900 px-3 py-2 text-sm font-medium text-white hover:bg-neutral-800 disabled:opacity-50"
        >
          {save.isPending ? '저장 중…' : configured ? '자격증명 갱신' : '자격증명 저장'}
        </button>
      </form>
    </div>
  )
}
