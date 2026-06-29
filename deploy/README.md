# 다른 서버 배포 (GHCR 이미지)

소스/빌드 없이 GHCR 발행 이미지를 pull 해서 데몬을 띄운다. **데몬마다 전용 이미지**라
한 데몬만 고쳐도 그 이미지만 재빌드·재배포된다. 이미지는
`.github/workflows/docker-publish.yml` 이 **main 푸시마다** 자동 발행한다(amd64 + arm64).
`:latest` = 최신 main 빌드, `:1.0.24` 처럼 버전 고정도 가능.

| 데몬 | 이미지 | compose 파일 | 추가 필요 env | 기동 |
|---|---|---|---|---|
| 시황 브리프 | `ghcr.io/<owner>/kisd-brief` | `brief.yaml` | `KIS_TELEGRAM_BOT_TOKEN`, `KIS_TELEGRAM_CHAT_ID` | `docker compose -f brief.yaml up -d` |
| 데이트레이드 | `ghcr.io/<owner>/kisd-daytrade` | `daytrade.yaml` | (공통만) `KIS_IS_MOCK=false`면 실주문 | `docker compose -f daytrade.yaml up -d` |
| 자동 손절 | `ghcr.io/<owner>/kisd-stop-loss` | `stop-loss.yaml` | (공통만) | `docker compose -f stop-loss.yaml up -d` (기본 dry-run) |
| (일회성 CLI) | `ghcr.io/<owner>/kis-cli` | — | — | `docker run --rm … kis-cli <subcommand>` |

공통 env(모든 데몬 필수): `KIS_APP_KEY`, `KIS_APP_SECRET`, `KIS_ACCOUNT_NUMBER`, `KIS_IS_MOCK`.

> **데몬 이미지엔 CLI 가 없다.** `symbols sync`·`daytrade add`·관심종목 시드 같은 일회성/설정
> 작업은 **`kis-cli` 이미지**로 실행한다(같은 `kis-data` 볼륨을 공유하므로 결과는 데몬이 그대로 읽는다).

상태(토큰캐시·`symbols.db`·`*.toml`)는 **external 볼륨 `kis-data`** 로 세 데몬이 공유한다.
→ 토큰은 한 번만 발급(KIS 발급 1분 1회 제한 회피), `symbols sync` 도 한 번만 하면 된다.

## 준비

```bash
# 0) deploy/ 디렉터리(*.yaml + .env.example)만 서버로 복사
scp -r deploy/  user@server:/opt/kis-cli/    # 예시

# 1) (패키지가 private 면) GHCR 로그인 — PAT 는 read:packages 권한
echo $GHCR_PAT | docker login ghcr.io -u <github-username> --password-stdin

# 2) 공유 볼륨 생성 (세 데몬이 토큰캐시·symbols.db 공유 — 최초 1회)
docker volume create kis-data

# 3) 환경변수 채우기 (플레이스홀더 → 실제 값)
cp .env.example .env
$EDITOR .env

# 4) 종목 마스터 동기화 (최초 1회 — kis-cli 이미지로, 볼륨에 symbols.db 생성)
docker run --rm -v kis-data:/data --env-file .env -e TZ=Asia/Seoul \
  ghcr.io/<owner>/kis-cli:latest symbols sync
```

> 일회성 작업의 공통 형태(이하 반복):
> `docker run --rm -v kis-data:/data --env-file .env -e TZ=Asia/Seoul ghcr.io/<owner>/kis-cli:latest <args>`

## 데몬별 기동

```bash
# 시황 브리프: 관심종목 1회 시드(kis-cli 이미지) 후 데몬 상주 (국내·미국 주식 모두 지원)
docker run --rm -v kis-data:/data --env-file .env -e TZ=Asia/Seoul \
  ghcr.io/<owner>/kis-cli:latest brief stream 005930 000660 TSLA --once
docker compose -f brief.yaml up -d

# 데이트레이드: 전략 등록(kis-cli 이미지) 후 데몬 상주 (daytrade.toml hot-reload)
docker run --rm -v kis-data:/data --env-file .env -e TZ=Asia/Seoul \
  ghcr.io/<owner>/kis-cli:latest daytrade add paper ma-cross 005930 --qty 1 --budget 1000000
docker compose -f daytrade.yaml up -d

# 자동 손절: 기본 dry-run. 실매도는 stop-loss.yaml 의 command 끝에 "--execute" 추가.
docker compose -f stop-loss.yaml up -d
```

## 운영

각 명령은 대상 데몬 파일(`-f <데몬>.yaml`)에 건다.

```bash
docker compose -f daytrade.yaml ps                  # 상태
docker compose -f daytrade.yaml logs -f             # 로그 (KST 타임스탬프)
docker compose -f daytrade.yaml stop                # 그레이스풀 종료 (SIGTERM → 현재 작업 마무리)
docker compose -f daytrade.yaml pull && docker compose -f daytrade.yaml up -d   # 새 이미지로 갱신
docker compose -f daytrade.yaml down                # 이 데몬만 정리 (공유 볼륨 kis-data 는 보존)
```

> 기존 단일 `kis-cli` 이미지 + `command:` 로 띄우던 compose 도 그대로 동작한다(kis-cli 이미지에
> 모든 subcommand 가 남아 있다). 데몬 전용 이미지로 옮기려면 `image:` 만 `kisd-*` 로 바꾸고
> `pull && up -d` 하면 된다 — 데이터 마이그레이션 없음(같은 `kis-data` 볼륨).

## 주의
- **실주문**: `KIS_IS_MOCK=false` + (stop-loss는) `--execute` 일 때만 실제 매매. 처음엔 `KIS_IS_MOCK=true`(모의) 또는 dry-run 으로 검증.
- 시크릿은 이미지에 안 들어간다 — 런타임에 `.env` 로 주입. `deploy/.env` 는 `.gitignore` 처리됨(커밋 금지).
- 상태(토큰캐시·DB·toml)는 external 볼륨 `kis-data` 에 영속. `external` 이라 `docker compose down -v` 로도 안 지워진다. 서버 이전 시 이 볼륨을 함께 옮기면 관심종목/전략/토큰 유지.

## 문제 해결

### `/add` 또는 종목 조회 시 "일치하는 종목 없음"

```
⚠️ 실패: 005930: '005930' 일치하는 종목 없음. `kis symbols sync`로 마스터 갱신 필요할 수 있음.
```

005930(삼성전자)처럼 멀쩡한 코드인데도 뜬다면 **종목 마스터(`symbols.db`)가 비어 있다**는 뜻이다 —
준비 단계의 `symbols sync` 를 건너뛴 경우다. 마스터 동기화는 정적 마스터 파일을 받는 거라 **장 마감(세션 밖)에도** 된다.

```bash
# 1) 마스터 동기화 (kis-cli 이미지로 — 볼륨 kis-data 에 symbols.db 생성). 한 번만 하면 영속됨.
docker run --rm -v kis-data:/data --env-file .env -e TZ=Asia/Seoul \
  ghcr.io/<owner>/kis-cli:latest symbols sync
#    → "KOSPI 960 건 / KOSDAQ 1700 건 ..." 처럼 시장별 건수가 찍히면 정상.

# 2) 떠 있는 데몬이 새 DB 를 잡도록 재기동
docker compose -f brief.yaml restart

# 3) 확인 — 삼성전자 005930 이 나오면 OK
docker run --rm -v kis-data:/data ghcr.io/<owner>/kis-cli:latest symbols find 삼성
```

> 셋 데몬이 external 볼륨 `kis-data` 를 공유하므로 **`symbols sync` 는 한 번만** 하면 brief·daytrade·stop-loss 모두 같은 마스터를 쓴다.
> 단, 볼륨을 새로 만든 직후(`docker volume create kis-data` 후)거나 `docker volume rm kis-data` 로 지운 뒤에는 다시 한 번 동기화해야 한다.
