import { createRootRoute, Link, Outlet, useNavigate } from '@tanstack/react-router'
import { Separator } from '@base-ui-components/react/separator'
import { useMe, useLogout } from '../api/auth'

export const Route = createRootRoute({
  component: RootLayout,
})

function RootLayout() {
  const me = useMe()
  const logout = useLogout()
  const navigate = useNavigate()
  const user = me.data

  return (
    <div className="min-h-screen bg-neutral-50 text-neutral-900">
      <header className="border-b border-neutral-200 bg-white">
        <nav className="mx-auto flex max-w-5xl items-center gap-4 px-6 py-3">
          <span className="text-base font-semibold tracking-tight">
            kis-web — 포트폴리오
          </span>
          <Separator orientation="vertical" className="h-4 w-px bg-neutral-300" />
          <div className="flex items-center gap-4 text-sm">
            <Link
              to="/"
              className="text-neutral-600 hover:text-neutral-900 [&.active]:font-semibold [&.active]:text-neutral-900"
            >
              대시보드
            </Link>
            {user && (
              <Link
                to="/portfolio"
                className="text-neutral-600 hover:text-neutral-900 [&.active]:font-semibold [&.active]:text-neutral-900"
              >
                포트폴리오
              </Link>
            )}
            {user && (
              <Link
                to="/watchlist"
                className="text-neutral-600 hover:text-neutral-900 [&.active]:font-semibold [&.active]:text-neutral-900"
              >
                관심종목
              </Link>
            )}
            {user && (
              <Link
                to="/journal"
                className="text-neutral-600 hover:text-neutral-900 [&.active]:font-semibold [&.active]:text-neutral-900"
              >
                매매일지
              </Link>
            )}
            {user && (
              <Link
                to="/settings"
                className="text-neutral-600 hover:text-neutral-900 [&.active]:font-semibold [&.active]:text-neutral-900"
              >
                설정
              </Link>
            )}
          </div>

          {/* 우측: 로그인 상태 */}
          <div className="ml-auto flex items-center gap-3 text-sm">
            {user ? (
              <>
                <span className="text-neutral-500">
                  {user.display_name}
                  {user.is_admin && (
                    <span className="ml-1.5 rounded bg-neutral-900 px-1.5 py-0.5 text-[10px] font-medium text-white">
                      admin
                    </span>
                  )}
                </span>
                <button
                  type="button"
                  onClick={() =>
                    logout.mutate(undefined, {
                      onSuccess: () => navigate({ to: '/login' }),
                    })
                  }
                  className="rounded-lg border border-neutral-300 px-2.5 py-1 font-medium text-neutral-700 hover:bg-neutral-50"
                >
                  로그아웃
                </button>
              </>
            ) : (
              <Link
                to="/login"
                className="rounded-lg bg-neutral-900 px-3 py-1 font-medium text-white hover:bg-neutral-800"
              >
                로그인
              </Link>
            )}
          </div>
        </nav>
      </header>
      <main className="mx-auto max-w-5xl px-6 py-8">
        <Outlet />
      </main>
    </div>
  )
}
