# kis-web — 포트폴리오/시세 웹앱 구현 todo

플랜: `~/.claude/plans/glittery-enchanting-quill.md`

## P0 — 스캐폴드 ✅ 완료
- [x] **(선행) kis-core 토큰 캐시 pluggable** — `TokenStore` 트레잇 + `FileTokenStore`/`NullTokenStore`, `with_in_memory_cache` ✅ 테스트 22/22 통과
  - [x] token.rs 수정 (트레잇 + 구현 + TokenManager 주입)
  - [x] client.rs 수정 (`with_in_memory_cache` / `with_store`)
  - [x] `cargo build -p kis-core` + `cargo test --workspace` 회귀
- [x] `crates/kis-web` 크레이트 생성 (Cargo.toml + main.rs poem hello) ✅
- [x] poem-openapi 셸 + `/openapi.json` + `/docs` ✅ 구동 확인(curl)
- [x] Vite SPA 셸 (`web/`) — TanStack Router/Query + Base UI + Tailwind ✅ build/tsc 통과
- [x] dev 프록시(`/api` → poem) + `gen:api` 파이프라인 ✅ e2e 프록시 통과
- [x] dist 임베드(rust-embed, mime-guess) + SPA 폴백 라우트 ✅ 릴리스 바이너리 self-contained 검증(/tmp 실행)
- [x] Dockerfile.web (멀티스테이지 node→rust→slim) + .dockerignore ✅ 작성(docker build 자체는 미실행 — 임베드는 로컬 검증됨)

### P0 검증 요약
- `/` → index.html(200), `/login` → SPA 폴백(200), `/assets/*.js`(200, 정확 MIME), `/api/health` JSON, `/openapi.json`(200)
- 릴리스 바이너리를 web/dist 없는 cwd에서 실행해도 임베드 SPA 서빙 → 단일 바이너리 자립 확인
- **남은 검증**: `docker build -f Dockerfile.web -t kis-web .` 실제 빌드(느려서 보류)

## P1 — 인증/멀티유저 기반 (진행 중)
- [x] sqlx + SQLite, 마이그레이션 (users, kis_credentials, passkeys, sessions, watchlist) ✅ 0001_init.sql
- [x] 비번 register/login (argon2id) + 서버측 세션 + SecurityScheme(쿠키) ✅ **e2e 8케이스 통과**
  - register/login/logout/me, 첫 사용자=admin, allow_register 게이트, 중복=409, 짧은비번=400, 오류=401
  - 세션: 토큰 SHA-256 해시만 DB저장, 쿠키 HttpOnly+SameSite=Strict+Path=/+Max-Age
  - config: KIS_WEB_MASTER_KEY(64hex 필수), secure_cookie/allow_register env
- [x] 프론트 인증 배선 ✅ tsc/build 통과, 통합 단일오리진 검증
  - api/auth.ts(useMe/useLogin/useRegister/useLogout), login.tsx(로그인↔회원가입 토글),
    __root.tsx(상태별 nav+로그아웃), index.tsx(미로그인 리다이렉트)
- [x] crypto.rs (AES-256-GCM 봉투암호화) ✅ 테스트 3/3 (roundtrip/wrong-key/nonce-unique)
- [x] KIS 자격증명 암호화 CRUD (PUT/GET status/DELETE /account/kis-credentials) ✅ e2e 검증, DB 평문 없음 확인
- [x] 사용자별 KisClient 매니저 (RwLock<HashMap>, with_in_memory_cache 주입) ✅
- [ ] 패스키 reg/login (webauthn-rs) — 후속 (UI 버튼 "준비 중" 비활성 상태)

## P2 — 읽기전용 대시보드 (핵심, 진행 중)
- [x] portfolio/balance (국내+해외 잔고/손익 합산) ✅ 배관 e2e(409/502/패닉없음), 국내 TTTC8434R + 해외 NASD/NYSE/AMEX 순회
- [x] 자격증명 등록 화면(settings.tsx) + 포트폴리오 표시(index.tsx, 요약카드+보유테이블, 한국식 색상) ✅ tsc/build 통과
- [x] 폴링 (30초 refetchInterval) + Base UI 화면 ✅
- [x] quotes (현재가, 국내/해외 자동판별) ✅ e2e(409/배관), GET /quotes/{symbol}
- [x] symbols 검색 (kis-data Store, spawn_blocking) ✅ GET /symbols/search (symbols.db 없으면 빈결과 graceful)
- [x] watchlist CRUD ✅ e2e(추가/목록/멱등/삭제/정규화), GET/POST/DELETE /watchlist
- [x] 프론트: watchlist 페이지(검색-추가 드롭다운 + 행별 라이브 시세 + 삭제), 관심종목 nav ✅ tsc/build 통과
- [x] **잔고 실데이터 검증** ✅ 사용자 실제 자격증명으로 잔고 표시 확인됨
- [x] **자동 심볼 싱크** ✅ 시작 시 백그라운드(sync_all, 무인증 공개마스터 19822건) + 수동 버튼(POST /symbols/sync)
- [x] 이름→코드 해석 (FTS + LIKE 폴백) ✅ "삼성전자"→005930, "하이닉스"→000660(SK하이닉스)
- [x] 검색 LIKE 폴백 (kis-data Store::search_like 추가) ✅ 부분어 "하이닉스" 매칭
- [x] 해외 전일대비 부호 수정 (apply_sign, KIS sign코드 4/5=하락) ✅
- [x] 매칭 실패 한글 입력 거부(404) — 해외 $0 쓰레기 행 방지 ✅
- [ ] quotes/watchlist 실데이터 검증 (현재가 실제 표시) — 사용자 확인 대기
- [ ] (선택) FTS 토크나이저 개선은 kis-data 영역 — LIKE 폴백으로 우회함

## P3 — 실시간 시세 (SSE) ✅ 구현 완료 (실데이터는 장중 확인 대기)
- [x] kis-core ws.rs 리팩터(비파괴): Tick/Sub 타입, connect_and_stream 다중구독+Sink(Print|Channel),
      run_stream(단일연결 국내+해외 멀티플렉싱 + CancellationToken). CLI watch 무영향 ✅ 25테스트 통과
- [x] kis-web SSE: GET /api/quotes/stream?symbols= (일반 poem 핸들러, 세션쿠키 인증, CookieJarManager),
      run_stream 틱 → SSE Event, 스트림 Drop 시 cancel→WS종료 ✅ 401/409/200(event-stream) 검증
- [x] 프론트: api/stream.ts(앱-레벨 EventSource 매니저 + useLiveQuote, 심볼 합집합 1연결),
      watchlist 행 라이브(LIVE 점, REST 폴백), routes/symbol.$code.tsx 상세(라이브+최근체결) ✅ build/tsc
- [ ] **실데이터 검증**: 장중(국내 09:00~15:30 KST / 미국 야간)에 실제 틱 흐르는지 — 사용자 확인 대기
- 한계(v2): 탭당 WS 1연결(다중탭 위험), 동적구독=재연결. 야간선물 제외.

## P4+ (장기)
- [ ] P4 퍼블릭 하드닝 / P5 주문 / P6 데몬제어

## 검증 노트
- CI는 build만 → 로컬 `cargo test --workspace` 필수
- kis-core 변경 후 CLI 회귀 확인
