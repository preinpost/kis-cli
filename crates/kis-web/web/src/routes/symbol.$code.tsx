import { useEffect, useRef, useState } from 'react'
import { createFileRoute, Link, useNavigate } from '@tanstack/react-router'
import { useMe } from '../api/auth'
import { useKisStatus } from '../api/account'
import { useQuote } from '../api/quotes'
import { useLiveQuote, type LiveTick } from '../api/stream'
import { fmtMoney, signed, colorBySign, trimNum } from '../lib/quote'

export const Route = createFileRoute('/symbol/$code')({
  component: SymbolDetail,
})

function SymbolDetail() {
  const { code } = Route.useParams()
  const navigate = useNavigate()
  const me = useMe()
  const kis = useKisStatus()
  const configured = kis.data?.configured ?? false

  const quote = useQuote(code, configured)
  const live = useLiveQuote(code, configured)

  useEffect(() => {
    if (!me.isPending && !me.data) navigate({ to: '/login' })
  }, [me.isPending, me.data, navigate])

  // 최근 체결 누적 (최대 25건)
  const [recent, setRecent] = useState<LiveTick[]>([])
  const lastKey = useRef('')
  useEffect(() => {
    if (!live) return
    const key = `${live.time}-${live.price}-${live.volume}`
    if (key === lastKey.current) return
    lastKey.current = key
    setRecent((r) => [live, ...r].slice(0, 25))
  }, [live])

  if (me.isPending) return <p className="text-sm text-neutral-500">로딩 중…</p>
  if (!me.data) return null

  const q = quote.data
  const currency = q?.currency ?? (live?.market === 'overseas' ? 'USD' : 'KRW')
  const name = q?.name || code
  const price = live?.price ?? q?.price
  const diff = live?.diff ?? q?.change
  const rate = live?.rate ?? q?.change_rate
  const sign = live?.sign ?? q?.sign

  return (
    <div className="space-y-6">
      <Link to="/watchlist" className="text-sm text-neutral-500 hover:text-neutral-900">
        ← 관심종목
      </Link>

      {!kis.isPending && !configured && (
        <div className="rounded-xl border border-dashed border-neutral-300 bg-white p-5 text-sm text-neutral-600">
          현재가를 보려면{' '}
          <Link to="/settings" className="font-medium text-neutral-900 underline">
            KIS 자격증명
          </Link>
          을 먼저 등록하세요.
        </div>
      )}

      {/* 헤더: 종목명 + 라이브 가격 */}
      <div>
        <div className="flex items-center gap-2">
          <h1 className="text-2xl font-semibold tracking-tight">{name}</h1>
          {live && (
            <span className="flex items-center gap-1 text-xs font-medium text-emerald-600">
              <span className="inline-block h-1.5 w-1.5 animate-pulse rounded-full bg-emerald-500" />
              LIVE
            </span>
          )}
        </div>
        <p className="text-sm text-neutral-400">
          {code} · {currency}
        </p>
      </div>

      {configured && (
        <section className="rounded-xl border border-neutral-200 bg-white p-6 shadow-sm">
          {price == null ? (
            <p className="text-sm text-neutral-500">
              {quote.isError
                ? (quote.error as Error).message
                : '시세 불러오는 중…'}
            </p>
          ) : (
            <>
              <div className={`text-4xl font-semibold ${colorBySign(sign)}`}>
                {fmtMoney(price, currency)}
              </div>
              <div className={`mt-1 text-sm font-mono ${colorBySign(sign)}`}>
                {signed(diff, sign)} ({signed(rate, sign)}%)
              </div>
              <dl className="mt-4 grid grid-cols-2 gap-y-2 border-t border-neutral-100 pt-4 text-sm">
                {q?.open && (
                  <>
                    <dt className="text-neutral-500">시가</dt>
                    <dd className="text-right font-mono">{fmtMoney(q.open, currency)}</dd>
                  </>
                )}
                {q?.high && (
                  <>
                    <dt className="text-neutral-500">고가</dt>
                    <dd className="text-right font-mono">{fmtMoney(q.high, currency)}</dd>
                  </>
                )}
                {q?.low && (
                  <>
                    <dt className="text-neutral-500">저가</dt>
                    <dd className="text-right font-mono">{fmtMoney(q.low, currency)}</dd>
                  </>
                )}
                <dt className="text-neutral-500">누적거래량</dt>
                <dd className="text-right font-mono">
                  {trimNum(live?.volume ?? q?.volume)}
                </dd>
              </dl>
            </>
          )}
        </section>
      )}

      {/* 최근 체결 */}
      {recent.length > 0 && (
        <section className="overflow-hidden rounded-xl border border-neutral-200 bg-white shadow-sm">
          <h2 className="border-b border-neutral-200 px-4 py-2.5 text-sm font-medium text-neutral-500">
            실시간 체결
          </h2>
          <table className="w-full text-sm">
            <tbody>
              {recent.map((t, i) => (
                <tr key={`${t.time}-${i}`} className="border-b border-neutral-100 last:border-0">
                  <td className="px-4 py-1.5 font-mono text-xs text-neutral-400">
                    {fmtTime(t.time)}
                  </td>
                  <td className={`px-4 py-1.5 text-right font-mono ${colorBySign(t.sign)}`}>
                    {fmtMoney(t.price, currency)}
                  </td>
                  <td className={`px-4 py-1.5 text-right font-mono text-xs ${colorBySign(t.sign)}`}>
                    {signed(t.rate, t.sign)}%
                  </td>
                  <td className="px-4 py-1.5 text-right font-mono text-xs text-neutral-400">
                    {trimNum(t.volume)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </section>
      )}

      {configured && recent.length === 0 && price != null && (
        <p className="text-xs text-neutral-400">
          장 운영시간이 아니면 실시간 체결이 들어오지 않습니다 (위 시세는 최근값).
        </p>
      )}
    </div>
  )
}

// HHMMSS → HH:MM:SS
function fmtTime(t: string): string {
  if (t.length >= 6) return `${t.slice(0, 2)}:${t.slice(2, 4)}:${t.slice(4, 6)}`
  return t
}
