// 실시간 시세 EventSource 매니저 + useLiveQuote 훅.
// 마운트된 훅들이 요청하는 심볼 합집합을 ref-count로 관리 → /api/quotes/stream 하나만 연결.
// 합집합이 바뀌면 재연결. 수신 틱은 Map에 저장하고 useSyncExternalStore로 구독자에 반영.
import { useEffect } from 'react'
import { useSyncExternalStore } from 'react'

export type LiveTick = {
  market: string
  symbol: string
  time: string
  price: string
  sign: string
  diff: string
  rate: string
  volume: string
}

const ticks = new Map<string, LiveTick>()
const refs = new Map<string, number>()
const listeners = new Set<() => void>()

let es: EventSource | null = null
let currentUrl = ''
let rebuildTimer: ReturnType<typeof setTimeout> | null = null

function notify() {
  listeners.forEach((l) => l())
}

function rebuild() {
  const syms = [...refs.keys()].sort()
  const url = syms.length
    ? `/api/quotes/stream?symbols=${encodeURIComponent(syms.join(','))}`
    : ''
  if (url === currentUrl) return
  currentUrl = url

  if (es) {
    es.close()
    es = null
  }
  if (!url) return

  es = new EventSource(url) // 동일 오리진 → 세션 쿠키 자동 전송
  es.onmessage = (e) => {
    try {
      const tick = JSON.parse(e.data) as LiveTick
      ticks.set(tick.symbol, tick)
      notify()
    } catch {
      /* keep-alive 코멘트 등 무시 */
    }
  }
  // 오류 시 EventSource가 자동 재연결. (401/409면 계속 재시도하나 무해)
}

// 마운트/언마운트가 몰릴 때 재연결을 한 번으로 묶음
function scheduleRebuild() {
  if (rebuildTimer) clearTimeout(rebuildTimer)
  rebuildTimer = setTimeout(rebuild, 120)
}

function addSymbols(syms: string[]) {
  syms.forEach((s) => refs.set(s, (refs.get(s) ?? 0) + 1))
  scheduleRebuild()
}
function removeSymbols(syms: string[]) {
  syms.forEach((s) => {
    const n = (refs.get(s) ?? 1) - 1
    if (n <= 0) refs.delete(s)
    else refs.set(s, n)
  })
  scheduleRebuild()
}

function subscribe(cb: () => void) {
  listeners.add(cb)
  return () => {
    listeners.delete(cb)
  }
}

/** 실시간 틱 구독. enabled=false면 구독 안 함(자격증명 미등록 등). */
export function useLiveQuote(symbol: string, enabled = true): LiveTick | undefined {
  useEffect(() => {
    if (!enabled || !symbol) return
    addSymbols([symbol])
    return () => removeSymbols([symbol])
  }, [symbol, enabled])

  return useSyncExternalStore(
    subscribe,
    () => (enabled ? ticks.get(symbol) : undefined),
  )
}
