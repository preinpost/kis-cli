import { useEffect, useState } from 'react'
import { createFileRoute, Link, useNavigate } from '@tanstack/react-router'
import { useMe } from '../api/auth'
import { useKisStatus } from '../api/account'
import {
  useWatchlist,
  useAddWatch,
  useRemoveWatch,
  useSymbolSearch,
  useSyncSymbols,
  useQuote,
  useSpark,
  type WatchlistItem,
} from '../api/quotes'
import { useLiveQuote } from '../api/stream'
import { fmtMoney, signed, colorBySign } from '../lib/quote'
import { Sparkline } from '../components/Sparkline'

export const Route = createFileRoute('/watchlist')({
  component: WatchlistPage,
})

function WatchlistPage() {
  const navigate = useNavigate()
  const me = useMe()
  const kis = useKisStatus()
  const configured = kis.data?.configured ?? false
  const list = useWatchlist()

  useEffect(() => {
    if (!me.isPending && !me.data) navigate({ to: '/login' })
  }, [me.isPending, me.data, navigate])

  if (me.isPending) return <p className="text-sm text-neutral-500">로딩 중…</p>
  if (!me.data) return null

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold tracking-tight">관심종목</h1>
        <SyncButton />
      </div>

      {!kis.isPending && !configured && (
        <div className="rounded-xl border border-dashed border-neutral-300 bg-white p-5 text-sm text-neutral-600">
          현재가를 보려면{' '}
          <Link to="/settings" className="font-medium text-neutral-900 underline">
            KIS 자격증명
          </Link>
          을 먼저 등록하세요. (등록 전에도 종목 추가는 가능합니다.)
        </div>
      )}

      <AddBox />

      <section className="overflow-hidden rounded-xl border border-neutral-200 bg-white shadow-sm">
        {list.isPending ? (
          <p className="p-5 text-sm text-neutral-500">불러오는 중…</p>
        ) : (list.data?.length ?? 0) === 0 ? (
          <p className="p-5 text-sm text-neutral-500">
            관심종목이 없습니다. 위에서 종목을 검색해 추가하세요.
          </p>
        ) : (
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-neutral-200 text-left text-xs text-neutral-500">
                <th className="px-4 py-2.5 font-medium">종목</th>
                <th className="px-4 py-2.5 text-right font-medium">현재가</th>
                <th className="px-4 py-2.5 text-right font-medium">전일대비</th>
                <th className="px-4 py-2.5 text-center font-medium">차트</th>
                <th className="px-4 py-2.5 text-right font-medium"></th>
              </tr>
            </thead>
            <tbody>
              {list.data!.map((item) => (
                <WatchRow key={item.symbol} item={item} enabled={configured} />
              ))}
            </tbody>
          </table>
        )}
      </section>
    </div>
  )
}

function SyncButton() {
  const sync = useSyncSymbols()
  return (
    <div className="flex items-center gap-2">
      {sync.isSuccess && (
        <span className="text-xs text-emerald-600">
          {sync.data.synced.toLocaleString('ko-KR')}건 동기화됨
        </span>
      )}
      {sync.isError && (
        <span className="text-xs text-red-500">{(sync.error as Error).message}</span>
      )}
      <button
        type="button"
        onClick={() => sync.mutate()}
        disabled={sync.isPending}
        className="rounded-lg border border-neutral-300 px-3 py-1.5 text-xs font-medium text-neutral-600 hover:bg-neutral-50 disabled:opacity-50"
        title="종목 마스터(이름·검색) 최신화"
      >
        {sync.isPending ? '동기화 중…' : '종목 동기화'}
      </button>
    </div>
  )
}

function AddBox() {
  const [q, setQ] = useState('')
  const [debounced, setDebounced] = useState('')
  const search = useSymbolSearch(debounced)
  const add = useAddWatch()

  useEffect(() => {
    const t = setTimeout(() => setDebounced(q), 250)
    return () => clearTimeout(t)
  }, [q])

  function addSymbol(symbol: string) {
    if (!symbol.trim()) return
    add.mutate(symbol.trim(), {
      onSuccess: () => {
        setQ('')
        setDebounced('')
      },
    })
  }

  return (
    <div className="relative">
      <form
        onSubmit={(e) => {
          e.preventDefault()
          addSymbol(q)
        }}
        className="flex gap-2"
      >
        <input
          value={q}
          onChange={(e) => setQ(e.target.value)}
          placeholder="종목명·코드·티커 검색 (예: 삼성전자, 005930, AAPL)"
          className="flex-1 rounded-lg border border-neutral-300 bg-white px-3 py-2 text-sm outline-none focus:border-neutral-900 focus:ring-1 focus:ring-neutral-900"
        />
        <button
          type="submit"
          disabled={add.isPending || !q.trim()}
          className="rounded-lg bg-neutral-900 px-4 py-2 text-sm font-medium text-white hover:bg-neutral-800 disabled:opacity-50"
        >
          추가
        </button>
      </form>

      {add.error && (
        <p className="mt-1 text-xs text-red-600">{(add.error as Error).message}</p>
      )}

      {/* 검색 결과 드롭다운 */}
      {debounced && (search.data?.length ?? 0) > 0 && (
        <ul className="absolute z-10 mt-1 w-full overflow-hidden rounded-lg border border-neutral-200 bg-white shadow-lg">
          {search.data!.map((s) => (
            <li key={`${s.kind}-${s.code}`}>
              <button
                type="button"
                onClick={() => addSymbol(s.code)}
                className="flex w-full items-center justify-between px-3 py-2 text-left text-sm hover:bg-neutral-50"
              >
                <span>
                  <span className="font-medium text-neutral-900">{s.name}</span>
                  <span className="ml-2 text-xs text-neutral-400">{s.code}</span>
                </span>
                <span className="text-xs text-neutral-400">{s.market_label}</span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}

function WatchRow({ item, enabled }: { item: WatchlistItem; enabled: boolean }) {
  const quote = useQuote(item.symbol, enabled)
  const live = useLiveQuote(item.symbol, enabled)
  const spark = useSpark(item.symbol, enabled)
  const remove = useRemoveWatch()
  const q = quote.data

  // 실시간 틱 우선, 없으면 REST 폴백.
  const currency = q?.currency ?? (item.market === 'overseas' ? 'USD' : 'KRW')
  const price = live?.price ?? q?.price
  const diff = live?.diff ?? q?.change
  const rate = live?.rate ?? q?.change_rate
  const sign = live?.sign ?? q?.sign

  return (
    <tr className="border-b border-neutral-100 last:border-0">
      <td className="px-4 py-2.5">
        <Link
          to="/symbol/$code"
          params={{ code: item.symbol }}
          className="font-medium text-neutral-900 hover:underline"
        >
          {q?.name || item.symbol}
        </Link>
        <div className="flex items-center gap-1.5 text-xs text-neutral-400">
          {item.symbol} · {currency}
          {live && (
            <span className="flex items-center gap-1 text-emerald-600">
              <span className="inline-block h-1.5 w-1.5 animate-pulse rounded-full bg-emerald-500" />
              LIVE
            </span>
          )}
        </div>
      </td>
      <td className="px-4 py-2.5 text-right font-mono">
        {!enabled ? (
          <span className="text-neutral-300">—</span>
        ) : price == null && quote.isPending ? (
          <span className="text-neutral-400">…</span>
        ) : price == null && quote.isError ? (
          <span className="text-xs text-red-500">오류</span>
        ) : (
          fmtMoney(price, currency)
        )}
      </td>
      <td className={`px-4 py-2.5 text-right font-mono ${colorBySign(sign)}`}>
        {enabled && price != null ? (
          <>
            {signed(diff, sign)}
            <div className="text-xs">{signed(rate, sign)}%</div>
          </>
        ) : (
          ''
        )}
      </td>
      <td className="px-4 py-2.5">
        {enabled && (
          <div className="flex justify-center">
            <Sparkline
              points={spark.data?.points ?? []}
              up={spark.data?.up ?? true}
              loading={spark.isPending}
            />
          </div>
        )}
      </td>
      <td className="px-4 py-2.5 text-right">
        <button
          type="button"
          onClick={() => remove.mutate(item.symbol)}
          className="rounded-md px-2 py-1 text-xs text-neutral-400 hover:bg-neutral-100 hover:text-red-600"
          title="삭제"
        >
          ✕
        </button>
      </td>
    </tr>
  )
}
