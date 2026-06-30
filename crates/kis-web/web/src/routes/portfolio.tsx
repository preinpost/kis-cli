import { useEffect, useState } from 'react'
import { createFileRoute, Link, useNavigate } from '@tanstack/react-router'
import { useMe } from '../api/auth'
import { usePositions, useSaveMeta, type Position } from '../api/positions'
import { fmtMoney } from '../lib/quote'

export const Route = createFileRoute('/portfolio')({
  component: PortfolioPage,
})

function PortfolioPage() {
  const navigate = useNavigate()
  const me = useMe()
  const pos = usePositions()

  useEffect(() => {
    if (!me.isPending && !me.data) navigate({ to: '/login' })
  }, [me.isPending, me.data, navigate])

  if (me.isPending) return <p className="text-sm text-neutral-500">로딩 중…</p>
  if (!me.data) return null

  const positions = pos.data?.positions ?? []
  // 통화별 그룹 + 합계
  const byCurrency = groupBy(positions, (p) => p.currency)

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-semibold tracking-tight">포트폴리오 관리</h1>
        <p className="mt-1 text-sm text-neutral-500">
          KIS 자동 보유 + 매매일지 기반 수동 보유. 종목별 목표·손절·메모 관리.
        </p>
      </div>

      {pos.isPending && <p className="text-sm text-neutral-500">불러오는 중…</p>}
      {pos.isError && (
        <p className="rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-600">
          {(pos.error as Error).message}
        </p>
      )}

      {positions.length === 0 && !pos.isPending && (
        <section className="rounded-xl border border-dashed border-neutral-300 bg-white p-6 text-center text-sm text-neutral-600">
          보유 종목이 없습니다.{' '}
          <Link to="/journal" search={{}} className="font-medium text-neutral-900 underline">
            매매일지
          </Link>
          에 매수 기록을 추가하거나{' '}
          <Link to="/settings" className="font-medium text-neutral-900 underline">
            KIS 자격증명
          </Link>
          을 등록하세요.
        </section>
      )}

      {Object.entries(byCurrency).map(([currency, items]) => {
        const totalEval = items.reduce((s, p) => s + p.eval_amount, 0)
        const totalUnreal = items.reduce((s, p) => s + (p.unrealized_pnl ?? 0), 0)
        const totalReal = items.reduce((s, p) => s + p.realized_pnl, 0)
        return (
          <section key={currency} className="space-y-2">
            <div className="flex flex-wrap items-baseline justify-between gap-2">
              <h2 className="text-sm font-medium text-neutral-700">{currency}</h2>
              <div className="flex gap-4 text-sm">
                <span className="text-neutral-500">
                  평가 <b className="font-mono text-neutral-900">{fmtMoney(String(totalEval), currency)}</b>
                </span>
                <span className="text-neutral-500">
                  미실현 <b className={`font-mono ${pnlColor(totalUnreal)}`}>{signedMoney(totalUnreal, currency)}</b>
                </span>
                <span className="text-neutral-500">
                  실현 <b className={`font-mono ${pnlColor(totalReal)}`}>{signedMoney(totalReal, currency)}</b>
                </span>
              </div>
            </div>
            <div className="overflow-hidden rounded-xl border border-neutral-200 bg-white shadow-sm">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-neutral-200 text-left text-xs text-neutral-500">
                    <th className="px-3 py-2.5 font-medium">종목</th>
                    <th className="px-3 py-2.5 text-right font-medium">수량</th>
                    <th className="px-3 py-2.5 text-right font-medium">평단</th>
                    <th className="px-3 py-2.5 text-right font-medium">현재가</th>
                    <th className="px-3 py-2.5 text-right font-medium">평가</th>
                    <th className="px-3 py-2.5 text-right font-medium">손익</th>
                    <th className="px-3 py-2.5 text-right font-medium">비중/목표</th>
                    <th className="px-3 py-2.5"></th>
                  </tr>
                </thead>
                <tbody>
                  {items.map((p) => (
                    <PositionRow key={`${p.source}-${p.symbol}`} p={p} totalEval={totalEval} />
                  ))}
                </tbody>
              </table>
            </div>
          </section>
        )
      })}
    </div>
  )
}

function PositionRow({ p, totalEval }: { p: Position; totalEval: number }) {
  const [open, setOpen] = useState(false)
  const save = useSaveMeta()

  const cur = p.current_price ?? undefined
  const targetHit = cur != null && p.target_price != null && cur >= p.target_price
  const stopHit = cur != null && p.stop_price != null && cur <= p.stop_price

  // 리밸런싱: 목표비중 대비 가감 주수
  let rebal: string | null = null
  if (p.target_weight != null && cur && cur > 0) {
    const targetValue = (p.target_weight / 100) * totalEval
    const diffShares = (targetValue - p.eval_amount) / cur
    if (Math.abs(diffShares) >= 0.5) {
      rebal = `${diffShares > 0 ? '+' : ''}${Math.round(diffShares)}주`
    }
  }

  function onSave(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault()
    const fd = new FormData(e.currentTarget)
    save.mutate(
      {
        symbol: p.symbol,
        body: {
          memo: strOrNull(fd.get('memo')),
          target_price: numOrNull(fd.get('target_price')),
          stop_price: numOrNull(fd.get('stop_price')),
          target_weight: numOrNull(fd.get('target_weight')),
        },
      },
      { onSuccess: () => setOpen(false) },
    )
  }

  const inputClass =
    'w-full rounded border border-neutral-300 px-2 py-1 text-sm outline-none focus:border-neutral-900'

  return (
    <>
      <tr className="border-b border-neutral-100 align-top">
        <td className="px-3 py-2.5">
          <Link
            to="/symbol/$code"
            params={{ code: p.symbol }}
            className="font-medium text-neutral-900 hover:underline"
          >
            {p.name}
          </Link>
          <div className="flex items-center gap-1 text-xs text-neutral-400">
            {p.symbol} · {p.source}
            {targetHit && <span className="rounded bg-red-50 px-1 text-red-600">목표</span>}
            {stopHit && <span className="rounded bg-blue-50 px-1 text-blue-600">손절</span>}
          </div>
          {p.memo && <div className="mt-0.5 text-xs text-neutral-400">{p.memo}</div>}
        </td>
        <td className="px-3 py-2.5 text-right font-mono">{fmtQty(p.quantity)}</td>
        <td className="px-3 py-2.5 text-right font-mono">{fmtMoney(String(p.avg_cost), p.currency)}</td>
        <td className="px-3 py-2.5 text-right font-mono">
          {cur != null ? fmtMoney(String(cur), p.currency) : '—'}
        </td>
        <td className="px-3 py-2.5 text-right font-mono">{fmtMoney(String(p.eval_amount), p.currency)}</td>
        <td className="px-3 py-2.5 text-right font-mono">
          {p.unrealized_pnl != null ? (
            <span className={pnlColor(p.unrealized_pnl)}>{signedMoney(p.unrealized_pnl, p.currency)}</span>
          ) : (
            <span className="text-neutral-300">—</span>
          )}
          {p.realized_pnl !== 0 && (
            <div className="text-xs text-neutral-400">
              실현 <span className={pnlColor(p.realized_pnl)}>{signedMoney(p.realized_pnl, p.currency)}</span>
            </div>
          )}
        </td>
        <td className="px-3 py-2.5 text-right">
          <div className="font-mono">{p.weight.toFixed(1)}%</div>
          {p.target_weight != null && (
            <div className="text-xs text-neutral-400">
              목표 {p.target_weight}%{rebal && <span className="ml-1 text-neutral-600">{rebal}</span>}
            </div>
          )}
        </td>
        <td className="px-3 py-2.5 text-right">
          <button
            type="button"
            onClick={() => setOpen((v) => !v)}
            className="rounded px-2 py-1 text-xs text-neutral-500 hover:bg-neutral-100"
          >
            관리
          </button>
        </td>
      </tr>
      {open && (
        <tr className="border-b border-neutral-100 bg-neutral-50">
          <td colSpan={8} className="px-3 py-3">
            <form onSubmit={onSave} className="flex flex-wrap items-end gap-3">
              <label className="text-xs text-neutral-500">
                메모
                <input name="memo" defaultValue={p.memo ?? ''} className={`${inputClass} w-48`} />
              </label>
              <label className="text-xs text-neutral-500">
                목표가
                <input name="target_price" type="number" step="any" defaultValue={p.target_price ?? ''} className={`${inputClass} w-28`} />
              </label>
              <label className="text-xs text-neutral-500">
                손절가
                <input name="stop_price" type="number" step="any" defaultValue={p.stop_price ?? ''} className={`${inputClass} w-28`} />
              </label>
              <label className="text-xs text-neutral-500">
                목표비중(%)
                <input name="target_weight" type="number" step="any" defaultValue={p.target_weight ?? ''} className={`${inputClass} w-24`} />
              </label>
              <button
                type="submit"
                disabled={save.isPending}
                className="rounded-lg bg-neutral-900 px-3 py-1.5 text-sm font-medium text-white hover:bg-neutral-800 disabled:opacity-50"
              >
                저장
              </button>
              <Link
                to="/journal"
                search={{ symbol: p.symbol }}
                className="text-xs text-neutral-500 underline hover:text-neutral-900"
              >
                이 종목 매매일지 →
              </Link>
            </form>
          </td>
        </tr>
      )}
    </>
  )
}

function groupBy<T>(arr: T[], key: (t: T) => string): Record<string, T[]> {
  const out: Record<string, T[]> = {}
  for (const item of arr) {
    const k = key(item)
    ;(out[k] ??= []).push(item)
  }
  return out
}
function fmtQty(n: number): string {
  return n.toLocaleString('ko-KR', { maximumFractionDigits: 4 })
}
function signedMoney(n: number, currency: string): string {
  return (n > 0 ? '+' : '') + fmtMoney(String(n), currency)
}
function pnlColor(n: number): string {
  if (n > 0) return 'text-red-600'
  if (n < 0) return 'text-blue-600'
  return 'text-neutral-500'
}
function strOrNull(v: FormDataEntryValue | null): string | undefined {
  const s = String(v ?? '').trim()
  return s === '' ? undefined : s
}
function numOrNull(v: FormDataEntryValue | null): number | undefined {
  const s = String(v ?? '').trim()
  if (s === '') return undefined
  const n = Number(s)
  return Number.isFinite(n) ? n : undefined
}
