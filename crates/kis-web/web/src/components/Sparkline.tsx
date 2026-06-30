// 토스 스타일 미니 차트. 종가 시계열 → SVG polyline + 끝점 dot.
// 색상은 한국식(상승=빨강, 하락=파랑). points 가 비면 점선 placeholder.

type Props = {
  points: number[]
  up?: boolean
  width?: number
  height?: number
  loading?: boolean
}

export function Sparkline({ points, up = true, width = 64, height = 32, loading }: Props) {
  const pad = 3
  const w = width
  const h = height

  if (loading) {
    // 로딩: 아주 옅은 펄스 막대 (점선 X — '깨진 것처럼' 보이지 않게)
    return (
      <div
        className="animate-pulse rounded bg-neutral-100"
        style={{ width: w, height: h }}
        aria-hidden
      />
    )
  }
  if (!points || points.length < 2) {
    // 데이터 없음/오류: 빈 칸 (placeholder 없음)
    return <div style={{ width: w, height: h }} aria-hidden />
  }

  const min = Math.min(...points)
  const max = Math.max(...points)
  const range = max - min || 1
  const stepX = (w - pad * 2) / (points.length - 1)
  const x = (i: number) => pad + i * stepX
  const y = (v: number) => h - pad - ((v - min) / range) * (h - pad * 2)

  const d = points.map((v, i) => `${i === 0 ? 'M' : 'L'}${x(i).toFixed(1)},${y(v).toFixed(1)}`).join(' ')
  const color = up ? '#ef4444' : '#3b82f6' // red-500 / blue-500
  const lastX = x(points.length - 1)
  const lastY = y(points[points.length - 1])

  return (
    <svg width={w} height={h} className="overflow-visible">
      <path d={d} fill="none" stroke={color} strokeWidth={1.5} strokeLinejoin="round" strokeLinecap="round" />
      <circle cx={lastX} cy={lastY} r={4} fill={color} opacity={0.18} />
      <circle cx={lastX} cy={lastY} r={2} fill={color} />
    </svg>
  )
}
