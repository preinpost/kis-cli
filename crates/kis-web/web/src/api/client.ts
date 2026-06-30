import createClient from 'openapi-fetch'
import type { paths } from './schema'

// The OpenAPI spec declares its server as `/api`, so the generated `paths`
// keys are server-relative (e.g. `/health`, NOT `/api/health`). openapi-fetch
// ignores the spec `servers` entry and only uses `baseUrl`, so we set it to
// `/api` here — that way `client.GET('/health')` resolves to `/api/health`,
// which the Vite dev proxy forwards to the poem backend.
export const client = createClient<paths>({ baseUrl: '/api' })
