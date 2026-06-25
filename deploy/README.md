# 다른 서버 배포 (GHCR 이미지)

소스/빌드 없이 GHCR 에 발행된 이미지(`ghcr.io/<owner>/kis-cli`)를 pull 해서 데몬을 띄운다.
이미지는 `.github/workflows/docker-publish.yml` 이 **버전 태그(`v*`) 푸시** 시 자동 발행한다
(amd64 + arm64). `:latest` = 최신 버전, `:1.0.22` 처럼 버전 고정도 가능.

## 준비

```bash
# 0) 이 디렉터리(compose.yaml + .env.example)만 서버로 복사
scp -r deploy/  user@server:/opt/kis-cli/    # 예시

# 1) (패키지가 private 면) GHCR 로그인 — PAT 는 read:packages 권한
echo $GHCR_PAT | docker login ghcr.io -u <github-username> --password-stdin

# 2) 환경변수 채우기 (플레이스홀더 → 실제 값)
cp .env.example .env
$EDITOR .env

# 3) 이미지 pull
docker compose pull

# 4) 최초 1회 — 종목 마스터 동기화 (볼륨에 symbols.db 생성)
docker compose run --rm telegram symbols sync
```

## 데몬별 기동

| 데몬 | 추가 필요 env | 기동 |
|---|---|---|
| 텔레그램 스트림 | `KIS_TELEGRAM_BOT_TOKEN`, `KIS_TELEGRAM_CHAT_ID` | `docker compose up -d telegram` |
| 데이트레이드 | (공통만) `KIS_IS_MOCK=false`면 실주문 | `docker compose up -d daytrade` |
| 자동 손절 | (공통만) | `docker compose --profile stop-loss up -d stop-loss` |

공통 env(모든 데몬 필수): `KIS_APP_KEY`, `KIS_APP_SECRET`, `KIS_ACCOUNT_NUMBER`, `KIS_IS_MOCK`.

```bash
# 텔레그램: 관심종목 1회 시드 후 상주
docker compose run --rm telegram telegram stream 005930 000660 TSLA
docker compose up -d telegram

# 데이트레이드: 전략 등록(daytrade.toml hot-reload) 후 상주
docker compose run --rm daytrade daytrade add paper ma-cross 005930 --qty 1 --budget 1000000
docker compose up -d daytrade

# 자동 손절: 기본 dry-run. 실매도는 compose.yaml 의 command 에 "--execute" 추가.
docker compose --profile stop-loss up -d stop-loss
```

## 운영

```bash
docker compose ps                  # 상태
docker compose logs -f daytrade    # 데몬별 로그
docker compose stop daytrade       # 그레이스풀 종료 (SIGTERM → 현재 작업 마무리 후 종료)
docker compose pull && docker compose up -d   # 새 이미지로 무중단에 가깝게 갱신
docker compose down                # 전부 정리 (볼륨 kis-data 는 보존)
```

## 주의
- **실주문**: `KIS_IS_MOCK=false` + (stop-loss는) `--execute` 일 때만 실제 매매. 처음엔 `KIS_IS_MOCK=true`(모의) 또는 dry-run 으로 검증.
- 시크릿은 이미지에 안 들어간다 — 런타임에 `.env` 로 주입. `deploy/.env` 는 `.gitignore` 처리됨(커밋 금지).
- 상태(토큰캐시·DB·toml)는 `kis-data` 볼륨에 영속. 서버 이전 시 볼륨도 함께 옮기면 관심종목/전략 유지.
