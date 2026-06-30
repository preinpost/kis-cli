// 시세 표시 포맷 (REST Quote·실시간 LiveTick 공용).
// 둘 다 KIS 전일대비부호 코드(sign: 1상한·2상승·3보합·4하한·5하락)를 주므로
// 절대값에 부호를 재적용해 일관되게 표시한다(한국식 색상: 빨강=상승, 파랑=하락).

export function num(s?: string): number {
  return Number(String(s ?? '').trim()) || 0
}

export function fmtMoney(s?: string, currency?: string): string {
  const sym = currency === 'USD' ? '$' : '₩'
  return `${sym}${num(s).toLocaleString('ko-KR', { maximumFractionDigits: 2 })}`
}

/** 절대값 크기 + 부호코드 → 부호 있는 숫자 문자열. */
export function signed(mag?: string, sign?: string): string {
  const m = Math.abs(num(mag))
  const v = sign === '4' || sign === '5' ? -m : m
  return (v > 0 ? '+' : '') + v.toLocaleString('ko-KR', { maximumFractionDigits: 2 })
}

export function colorBySign(sign?: string): string {
  if (sign === '1' || sign === '2') return 'text-red-600'
  if (sign === '4' || sign === '5') return 'text-blue-600'
  return 'text-neutral-500'
}

export function trimNum(s?: string): string {
  return num(s).toLocaleString('ko-KR', { maximumFractionDigits: 4 })
}
