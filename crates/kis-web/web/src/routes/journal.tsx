import { useEffect, useState } from 'react'
import { createFileRoute, Link, useNavigate } from '@tanstack/react-router'
import { useMe } from '../api/auth'
import { useSymbolSearch } from '../api/quotes'
import {
  useTrades,
  useTradeStats,
  useCreateTrade,
  useUpdateTrade,
  useDeleteTrade,
  type Trade,
  type TradeInput,
} from '../api/journal'
import { num, fmtMoney } from '../lib/quote'

export const Route = createFileRoute('/journal')({
  validateSearch: (s: Record<string, unknown>): { symbol?: string } => ({
    symbol: typeof s.symbol === 'string' ? s.symbol : undefined,
  }),
  component: Journal,
})

function Journal() {
  const navigate = useNavigate()
  const me = useMe()
  const { symbol: symbolFilter } = Route.useSearch()

  const [side, setSide] = useState('')
  const [editing, setEditing] = useState<Trade | null>(null)
  const [showForm, setShowForm] = useState(false)

  const trades = useTrades({ symbol: symbolFilter, side: side || undefined })
  const stats = useTradeStats()
  const del = useDeleteTrade()

  useEffect(() => {
    if (!me.isPending && !me.data) navigate({ to: '/login' })
  }, [me.isPending, me.data, navigate])

  if (me.isPending) return <p className="text-sm text-neutral-500">로딩 중…</p>
  if (!me.data) return null

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-semibold tracking-tight">매매일지</h1>
          {symbolFilter && (
            <p className="mt-1 text-sm text-neutral-500">
              종목 {symbolFilter} ·{' '}
              <Link to="/journal" search={{}} className="text-neutral-900 underline">
                전체 보기
              </Link>
            </p>
          )}
        </div>
        <button
          type="button"
          onClick={() => {
            setEditing(null)
            setShowForm((v) => !v)
          }}
          className="rounded-lg bg-neutral-900 px-4 py-2 text-sm font-medium text-white hover:bg-neutral-800"
        >
          {showForm && !editing ? '닫기' : '+ 매매 기록'}
        </button>
      </div>

      {/* 통계 */}
      {stats.data && (
        <div className="grid gap-3 sm:grid-cols-3">
          <StatCard label="실현손익">
            {stats.data.realized.length === 0 ? (
              <span className="text-neutral-400">—</span>
            ) : (
              stats.data.realized.map((r) => (
                <div
                  key={r.currency}
                  className={r.amount >= 0 ? 'text-red-600' : 'text-blue-600'}
                >
                  {r.amount >= 0 ? '+' : ''}
                  {fmtMoney(String(r.amount), r.currency)}
                </div>
              ))
            )}
          </StatCard>
          <StatCard label="승률 (매도 기준)">
            {Math.round(stats.data.win_rate * 100)}%
            <span className="ml-1 text-sm font-normal text-neutral-400">
              ({stats.data.win_count}/{stats.data.sell_count})
            </span>
          </StatCard>
          <StatCard label="기록">
            {stats.data.trade_count}건
            <span className="ml-1 text-sm font-normal text-neutral-400">
              · {stats.data.symbol_count}종목
            </span>
          </StatCard>
        </div>
      )}

      {/* 폼 */}
      {(showForm || editing) && (
        <TradeForm
          initial={editing}
          onClose={() => {
            setShowForm(false)
            setEditing(null)
          }}
        />
      )}

      {/* 필터 */}
      <div className="flex gap-2 text-sm">
        {['', 'buy', 'sell'].map((s) => (
          <button
            key={s || 'all'}
            type="button"
            onClick={() => setSide(s)}
            className={`rounded-lg border px-3 py-1.5 ${
              side === s
                ? 'border-neutral-900 bg-neutral-900 text-white'
                : 'border-neutral-300 text-neutral-600 hover:bg-neutral-50'
            }`}
          >
            {s === '' ? '전체' : s === 'buy' ? '매수' : '매도'}
          </button>
        ))}
      </div>

      {/* 목록 */}
      <section className="overflow-hidden rounded-xl border border-neutral-200 bg-white shadow-sm">
        {trades.isPending ? (
          <p className="p-5 text-sm text-neutral-500">불러오는 중…</p>
        ) : (trades.data?.length ?? 0) === 0 ? (
          <p className="p-5 text-sm text-neutral-500">매매 기록이 없습니다.</p>
        ) : (
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-neutral-200 text-left text-xs text-neutral-500">
                <th className="px-4 py-2.5 font-medium">일시</th>
                <th className="px-4 py-2.5 font-medium">종목</th>
                <th className="px-4 py-2.5 text-center font-medium">구분</th>
                <th className="px-4 py-2.5 text-right font-medium">수량</th>
                <th className="px-4 py-2.5 text-right font-medium">가격</th>
                <th className="px-4 py-2.5 font-medium">사유/메모</th>
                <th className="px-4 py-2.5"></th>
              </tr>
            </thead>
            <tbody>
              {trades.data!.map((tr) => (
                <tr key={tr.id} className="border-b border-neutral-100 last:border-0 align-top">
                  <td className="px-4 py-2.5 text-neutral-500">{tr.traded_at?.slice(0, 10)}</td>
                  <td className="px-4 py-2.5">
                    <div className="font-medium text-neutral-900">{tr.name || tr.symbol}</div>
                    <div className="text-xs text-neutral-400">
                      {tr.symbol}
                      {tr.broker ? ` · ${tr.broker}` : ''}
                    </div>
                  </td>
                  <td className="px-4 py-2.5 text-center">
                    <span
                      className={`rounded px-1.5 py-0.5 text-xs font-medium ${
                        tr.side === 'buy'
                          ? 'bg-red-50 text-red-600'
                          : 'bg-blue-50 text-blue-600'
                      }`}
                    >
                      {tr.side === 'buy' ? '매수' : '매도'}
                    </span>
                  </td>
                  <td className="px-4 py-2.5 text-right font-mono">{num(String(tr.quantity)).toLocaleString('ko-KR')}</td>
                  <td className="px-4 py-2.5 text-right font-mono">
                    {fmtMoney(String(tr.price), tr.currency)}
                  </td>
                  <td className="px-4 py-2.5 text-xs text-neutral-500">
                    {tr.reason && <div>{tr.reason}</div>}
                    {tr.memo && <div className="text-neutral-400">{tr.memo}</div>}
                    {tr.tags && <div className="text-neutral-400">#{tr.tags}</div>}
                  </td>
                  <td className="px-4 py-2.5 text-right whitespace-nowrap">
                    <button
                      type="button"
                      onClick={() => {
                        setEditing(tr)
                        setShowForm(false)
                      }}
                      className="rounded px-2 py-1 text-xs text-neutral-500 hover:bg-neutral-100"
                    >
                      수정
                    </button>
                    <button
                      type="button"
                      onClick={() => del.mutate(tr.id)}
                      className="rounded px-2 py-1 text-xs text-neutral-400 hover:bg-neutral-100 hover:text-red-600"
                    >
                      삭제
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </section>
    </div>
  )
}

function StatCard({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div className="rounded-xl border border-neutral-200 bg-white p-4 shadow-sm">
      <div className="text-xs text-neutral-500">{label}</div>
      <div className="mt-1 text-xl font-semibold">{children}</div>
    </div>
  )
}

function TradeForm({ initial, onClose }: { initial: Trade | null; onClose: () => void }) {
  const create = useCreateTrade()
  const update = useUpdateTrade()
  const [symbol, setSymbol] = useState(initial?.symbol ?? '')
  const [symbolQuery, setSymbolQuery] = useState('')
  const search = useSymbolSearch(symbolQuery)

  const inputClass =
    'w-full rounded-lg border border-neutral-300 bg-white px-3 py-2 text-sm outline-none focus:border-neutral-900'
  const err = (create.error as Error | null)?.message ?? (update.error as Error | null)?.message

  function onSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault()
    const fd = new FormData(e.currentTarget)
    const body: TradeInput = {
      traded_at: String(fd.get('traded_at') || ''),
      symbol: symbol.trim(),
      side: String(fd.get('side') || 'buy'),
      quantity: Number(fd.get('quantity') || 0),
      price: Number(fd.get('price') || 0),
      fee: Number(fd.get('fee') || 0),
      broker: str(fd.get('broker')),
      currency: str(fd.get('currency')),
      reason: str(fd.get('reason')),
      tags: str(fd.get('tags')),
      memo: str(fd.get('memo')),
    }
    const done = { onSuccess: onClose }
    if (initial) update.mutate({ id: initial.id, body }, done)
    else create.mutate(body, done)
  }

  return (
    <form
      onSubmit={onSubmit}
      className="space-y-3 rounded-xl border border-neutral-200 bg-white p-5 shadow-sm"
    >
      <h2 className="text-sm font-medium text-neutral-700">
        {initial ? '매매 기록 수정' : '새 매매 기록'}
      </h2>
      <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
        <label className="text-xs text-neutral-500">
          일시
          <input
            type="date"
            name="traded_at"
            required
            defaultValue={initial?.traded_at?.slice(0, 10)}
            className={inputClass}
          />
        </label>
        <label className="relative text-xs text-neutral-500">
          종목
          <input
            type="text"
            value={symbol}
            onChange={(e) => {
              setSymbol(e.target.value)
              setSymbolQuery(e.target.value)
            }}
            placeholder="005930 / TSLA / 종목명"
            required
            className={inputClass}
          />
          {symbolQuery && (search.data?.length ?? 0) > 0 && (
            <ul className="absolute z-10 mt-1 max-h-48 w-full overflow-auto rounded-lg border border-neutral-200 bg-white shadow-lg">
              {search.data!.map((s) => (
                <li key={`${s.kind}-${s.code}`}>
                  <button
                    type="button"
                    onClick={() => {
                      setSymbol(s.code)
                      setSymbolQuery('')
                    }}
                    className="flex w-full justify-between px-3 py-1.5 text-left text-sm hover:bg-neutral-50"
                  >
                    <span>{s.name}</span>
                    <span className="text-xs text-neutral-400">{s.code}</span>
                  </button>
                </li>
              ))}
            </ul>
          )}
        </label>
        <label className="text-xs text-neutral-500">
          구분
          <select name="side" defaultValue={initial?.side ?? 'buy'} className={inputClass}>
            <option value="buy">매수</option>
            <option value="sell">매도</option>
          </select>
        </label>
        <label className="text-xs text-neutral-500">
          증권사
          <input
            type="text"
            name="broker"
            list="brokers"
            defaultValue={initial?.broker ?? ''}
            placeholder="KIS / TOSS"
            className={inputClass}
          />
          <datalist id="brokers">
            <option value="KIS" />
            <option value="TOSS" />
            <option value="기타" />
          </datalist>
        </label>
        <label className="text-xs text-neutral-500">
          수량
          <input type="number" name="quantity" step="any" required defaultValue={initial?.quantity} className={inputClass} />
        </label>
        <label className="text-xs text-neutral-500">
          가격
          <input type="number" name="price" step="any" required defaultValue={initial?.price} className={inputClass} />
        </label>
        <label className="text-xs text-neutral-500">
          수수료
          <input type="number" name="fee" step="any" defaultValue={initial?.fee ?? 0} className={inputClass} />
        </label>
        <label className="text-xs text-neutral-500">
          통화
          <input type="text" name="currency" defaultValue={initial?.currency ?? ''} placeholder="KRW" className={inputClass} />
        </label>
      </div>
      <label className="block text-xs text-neutral-500">
        매매 사유
        <input type="text" name="reason" defaultValue={initial?.reason ?? ''} className={inputClass} />
      </label>
      <div className="grid grid-cols-2 gap-3">
        <label className="text-xs text-neutral-500">
          태그 (콤마)
          <input type="text" name="tags" defaultValue={initial?.tags ?? ''} placeholder="장기,배당" className={inputClass} />
        </label>
        <label className="text-xs text-neutral-500">
          회고/메모
          <input type="text" name="memo" defaultValue={initial?.memo ?? ''} className={inputClass} />
        </label>
      </div>
      {err && <p className="text-sm text-red-600">{err}</p>}
      <div className="flex gap-2">
        <button
          type="submit"
          disabled={create.isPending || update.isPending}
          className="rounded-lg bg-neutral-900 px-4 py-2 text-sm font-medium text-white hover:bg-neutral-800 disabled:opacity-50"
        >
          저장
        </button>
        <button
          type="button"
          onClick={onClose}
          className="rounded-lg border border-neutral-300 px-4 py-2 text-sm text-neutral-600 hover:bg-neutral-50"
        >
          취소
        </button>
      </div>
    </form>
  )
}

function str(v: FormDataEntryValue | null): string | undefined {
  const s = String(v ?? '').trim()
  return s === '' ? undefined : s
}
