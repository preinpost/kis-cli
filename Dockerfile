# syntax=docker/dockerfile:1
#
# kis-cli 컨테이너 이미지 (헤드리스).
# 차트 뷰어(wry/tao, WebKitGTK)는 `--no-default-features` 로 제외 → 작은 이미지.
# 데몬(telegram stream / daytrade daemon / stop-loss)과 일회성 CLI 모두 이 이미지로 실행.
# TLS 는 rustls(순수 Rust) — OpenSSL 불필요.
#
# 멀티아치(amd64+arm64): cargo-zigbuild 로 BUILDPLATFORM(amd64) 한 곳에서 두 아키를
# 네이티브 속도로 크로스컴파일한다(QEMU 컴파일 없음). 런타임 스테이지만 per-arch.

# ─────────────────────────── builder ───────────────────────────
# --platform=$BUILDPLATFORM: 빌더는 항상 네이티브(amd64 러너) → arm64 도 에뮬 없이 컴파일.
FROM --platform=$BUILDPLATFORM ghcr.io/rust-cross/cargo-zigbuild:0.23 AS builder

WORKDIR /app

# docker TARGETARCH(amd64/arm64) → rust 타깃 트리플.
ARG TARGETARCH
RUN case "$TARGETARCH" in \
        amd64) echo "x86_64-unknown-linux-gnu"  > /tgt ;; \
        arm64) echo "aarch64-unknown-linux-gnu" > /tgt ;; \
        *) echo "지원하지 않는 TARGETARCH: $TARGETARCH" >&2; exit 1 ;; \
    esac \
    && rustup target add "$(cat /tgt)"

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
# skill.rs 가 include_str!("../../../../skill/SKILL.md") 로 컴파일타임 임베드.
COPY skill ./skill

# BuildKit 캐시 마운트로 registry/target 캐시 → 반복 빌드 가속.
# registry 는 아키 공통이라 sharing=locked (amd64·arm64 빌더 동시 실행 시 unpack 경쟁 방지).
# target 은 아키별 캐시(비영속 레이어)이므로 바이너리를 레이어 경로로 복사해 둔다.
# --target 에 .2.34: glibc 2.34(pthread 가 libc 로 병합된 버전) 심볼을 zig 가 제공하도록.
# (런타임 debian bookworm = glibc 2.36 ≥ 2.34 → 호환). rustup·출력경로는 베이스 트리플 사용.
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=cargo-target-${TARGETARCH},target=/app/target \
    cargo zigbuild -p kis-cli --release --no-default-features --locked --target "$(cat /tgt).2.34" \
    && cp "target/$(cat /tgt)/release/kis" /usr/local/bin/kis

# ─────────────────────────── runtime ───────────────────────────
FROM debian:bookworm-slim AS runtime

# rustls = OpenSSL 불필요. TLS 루트 인증서 + KST 타임존 데이터만.
RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates tzdata \
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
