# syntax=docker/dockerfile:1
#
# kis-cli 컨테이너 이미지 (헤드리스).
# 차트 뷰어(wry/tao, WebKitGTK)는 `--no-default-features` 로 제외 → 작은 이미지.
# 데몬(telegram stream / daytrade daemon / stop-loss)과 일회성 CLI 모두 이 이미지로 실행.

# ─────────────────────────── builder ───────────────────────────
FROM rust:1-bookworm AS builder

# reqwest/tokio-tungstenite 의 native-tls = OpenSSL. rusqlite 는 bundled(C 컴파일러는 이미지에 포함).
RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
# skill.rs 가 include_str!("../../skill/SKILL.md") 로 컴파일타임 임베드.
COPY skill ./skill

# BuildKit 캐시 마운트로 registry/target 캐시 → 반복 빌드 가속.
# target 은 캐시(레이어 비영속)이므로 바이너리를 레이어 경로로 복사해 둔다.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --no-default-features --locked \
    && cp target/release/kis /usr/local/bin/kis

# ─────────────────────────── runtime ───────────────────────────
FROM debian:bookworm-slim AS runtime

# native-tls 런타임(libssl3) + TLS 루트 인증서 + KST 타임존 데이터.
RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates libssl3 tzdata \
    && rm -rf /var/lib/apt/lists/*

# 비루트 유저 + 상태 볼륨(/data → XDG_CONFIG_HOME → ~/.config 대체).
RUN useradd --create-home --uid 10001 kis \
    && mkdir -p /data && chown kis:kis /data

COPY --from=builder /usr/local/bin/kis /usr/local/bin/kis

USER kis
# XDG_CONFIG_HOME=/data → dirs::config_dir() = /data/kis-cli/
#   (config.toml·토큰캐시·symbols.db·daytrade.db·daytrade.toml·telegram-stream.toml 전부 볼륨에 영속)
# TZ=Asia/Seoul → Local::now()/cron 이 KST 로 동작 (한국 장시간 판정).
ENV XDG_CONFIG_HOME=/data \
    TZ=Asia/Seoul \
    RUST_LOG=info
VOLUME ["/data"]

ENTRYPOINT ["kis"]
CMD ["--help"]
