import { useEffect } from 'react'
import { createFileRoute, Link, useNavigate } from '@tanstack/react-router'
import { useMe } from '../api/auth'
import { useKisStatus } from '../api/account'
import { usePortfolio, type Holding, type Summary } from '../api/portfolio'

export const Route = createFileRoute('/')({
  component: Dashboard,
})

function Dashboard() {
  const navigate = useNavigate()
  const me = useMe()
  const kis = useKisStatus()
  const configured = kis.data?.configured ?? false
  const portfolio = usePortfolio(configured)

  useEffect(() => {
    if (!me.isPending && !me.data) navigate({ to: '/login' })
  }, [me.isPending, me.data, navigate])

  if (me.isPending) return <p className="text-sm text-neutral-500">로딩 중…</p>
  if (!me.data) return <p className="text-sm text-neutral-500">로그인이 필요합니다…</p>

  return (
    <div className="space-y-6">
      <div className="flex items-end justify-between">
        <div>
          <h1 className="text-2xl font-semibold tracking-tight">포트폴리오</h1>
          <p className="mt-1 text-sm text-neutral-500">{me.data.display_name}님</p>
        </div>
        {portfolio.data?.is_mock && (
          <span className="rounded-full bg-amber-100 px-2.5 py-1 text-xs font-medium text-amber-700">
            모의투자
          </span>
        )}
      </div>

      {/* 자격증명 미등록 → 설정 유도 */}
      {!kis.isPending && !configured && (
        <section className="rounded-xl border border-dashed border-neutral-300 bg-white p-6 text-center">
          <p className="text-sm text-neutral-600">
            KIS 자격증명을 등록하면 잔고·평가손익·현재가가 표시됩니다.
          </p>
          <Link
            to="/settings"
            className="mt-3 inline-block rounded-lg bg-neutral-900 px-4 py-2 text-sm font-medium text-white hover:bg-neutral-800"
          >
            자격증명 등록하러 가기
          </Link>
        </section>
      )}

      {/* 등록됨 → 잔고 */}
      {configured && (
        <>
          {portfolio.isPending && (
            <p className="text-sm text-neutral-500">잔고 조회 중…</p>
          )}
          {portfolio.isError && (
            <div className="rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-600">
              {(portfolio.error as Error).message}
              <div className="mt-1 text-xs text-red-400">
                자격증명이 올바른지 / 장 운영시간인지 확인하세요. 30초마다 자동 재시도합니다.
              </div>
            </div>
          )}
          {portfolio.data && (
            <>
              <div className="grid gap-4 sm:grid-cols-2">
                {portfolio.data.domestic && (
                  <SummaryCard title="국내 (KRW)" s={portfolio.data.domestic} />
                )}
                {portfolio.data.overseas && (
                  <SummaryCard title="해외 (USD)" s={portfolio.data.overseas} />
                )}
              </div>
              <HoldingsTable holdings={portfolio.data.holdings} />
            </>
          )}
        </>
      )}
    </div>
  )
}

function SummaryCard({ title, s }: { title: string; s: Summary }) {
  const pnl = num(s.total_pnl)
  return (
    <section className="rounded-xl border border-neutral-200 bg-white p-5 shadow-sm">
      <h2 className="text-sm font-medium text-neutral-500">{title}</h2>
      <p className={`mt-2 text-2xl font-semibold ${pnlColor(pnl)}`}>
        {fmtMoney(s.total_eval, s.currency)}
      </p>
      <dl className="mt-3 grid grid-cols-2 gap-y-1.5 text-sm">
        {s.deposit && (
          <>
            <dt className="text-neutral-500">예수금</dt>
            <dd className="text-right font-mono">{fmtMoney(s.deposit, s.currency)}</dd>
          </>
        )}
        <dt className="text-neutral-500">매입</dt>
        <dd className="text-right font-mono">{fmtMoney(s.total_purchase, s.currency)}</dd>
        <dt className="text-neutral-500">평가손익</dt>
        <dd className={`text-right font-mono ${pnlColor(pnl)}`}>
          {fmtSigned(s.total_pnl, s.currency)}
          {s.total_pnl_rate && ` (${signed(s.total_pnl_rate)}%)`}
        </dd>
      </dl>
    </section>
  )
}

function HoldingsTable({ holdings }: { holdings: Holding[] }) {
  if (holdings.length === 0) {
    return (
      <p className="rounded-xl border border-neutral-200 bg-white p-5 text-sm text-neutral-500">
        보유 종목이 없습니다.
      </p>
    )
  }
  return (
    <section className="overflow-hidden rounded-xl border border-neutral-200 bg-white shadow-sm">
      <table className="w-full text-sm">
        <thead>
          <tr className="border-b border-neutral-200 text-left text-xs text-neutral-500">
            <th className="px-4 py-2.5 font-medium">종목</th>
            <th className="px-4 py-2.5 text-right font-medium">수량</th>
            <th className="px-4 py-2.5 text-right font-medium">평균가</th>
            <th className="px-4 py-2.5 text-right font-medium">현재가</th>
            <th className="px-4 py-2.5 text-right font-medium">평가금액</th>
            <th className="px-4 py-2.5 text-right font-medium">손익</th>
          </tr>
        </thead>
        <tbody>
          {holdings.map((h) => {
            const pnl = num(h.pnl_amount)
            return (
              <tr
                key={`${h.market}-${h.symbol}`}
                className="border-b border-neutral-100 last:border-0"
              >
                <td className="px-4 py-2.5">
                  <div className="font-medium text-neutral-900">{h.name}</div>
                  <div className="text-xs text-neutral-400">
                    {h.symbol} · {h.currency}
                  </div>
                </td>
                <td className="px-4 py-2.5 text-right font-mono">{trimNum(h.quantity)}</td>
                <td className="px-4 py-2.5 text-right font-mono">{trimNum(h.avg_price)}</td>
                <td className="px-4 py-2.5 text-right font-mono">{trimNum(h.current_price)}</td>
                <td className="px-4 py-2.5 text-right font-mono">
                  {fmtMoney(h.eval_amount, h.currency)}
                </td>
                <td className={`px-4 py-2.5 text-right font-mono ${pnlColor(pnl)}`}>
                  {fmtSigned(h.pnl_amount, h.currency)}
                  <div className="text-xs">{signed(h.pnl_rate)}%</div>
                </td>
              </tr>
            )
          })}
        </tbody>
      </table>
    </section>
  )
}

// ── 포맷 헬퍼 ──
function num(s: string | undefined): number {
  return Number(String(s ?? '').trim()) || 0
}
function trimNum(s: string): string {
  const n = num(s)
  return n.toLocaleString('ko-KR', { maximumFractionDigits: 4 })
}
function fmtMoney(s: string, currency: string): string {
  const n = num(s)
  const sym = currency === 'USD' ? '$' : '₩'
  return `${sym}${n.toLocaleString('ko-KR', { maximumFractionDigits: 2 })}`
}
function fmtSigned(s: string, currency: string): string {
  const n = num(s)
  const sign = n > 0 ? '+' : ''
  return `${sign}${fmtMoney(s, currency)}`
}
function signed(s: string): string {
  const n = num(s)
  return (n > 0 ? '+' : '') + n.toLocaleString('ko-KR', { maximumFractionDigits: 2 })
}
function pnlColor(n: number): string {
  if (n > 0) return 'text-red-600'
  if (n < 0) return 'text-blue-600'
  return 'text-neutral-900'
}
