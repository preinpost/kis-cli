-- kis-web P1 스키마: 멀티유저 인증 + KIS 자격증명 + watchlist.
-- id 는 TEXT(uuid v4 하이픈) 로 저장해 SQLite 타입 매핑을 단순화한다.

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS users (
    id            TEXT PRIMARY KEY,
    username      TEXT NOT NULL UNIQUE,
    display_name  TEXT NOT NULL,
    password_hash TEXT NOT NULL,            -- argon2id PHC 문자열
    is_admin      INTEGER NOT NULL DEFAULT 0,
    created_at    TEXT NOT NULL
);

-- 서버측 세션. id = 세션토큰의 SHA-256 hex (원문 토큰은 쿠키에만, DB엔 해시만).
CREATE TABLE IF NOT EXISTS sessions (
    id         TEXT PRIMARY KEY,
    user_id    TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);

-- 사용자별 KIS 자격증명. app_key/app_secret 은 {key,secret} JSON 을 AES-256-GCM 으로 봉투암호화.
-- account_number 는 비밀이 아니라 평문(표시·계좌번호 분해에 필요).
CREATE TABLE IF NOT EXISTS kis_credentials (
    user_id        TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    secret_enc     BLOB NOT NULL,           -- AES-GCM ciphertext(+tag)
    nonce          BLOB NOT NULL,           -- 12-byte GCM nonce (행마다 고유)
    account_number TEXT NOT NULL,
    is_mock        INTEGER NOT NULL DEFAULT 1,
    updated_at     TEXT NOT NULL
);

-- WebAuthn 패스키 (webauthn-rs Passkey 직렬화 JSON).
CREATE TABLE IF NOT EXISTS passkeys (
    id           TEXT PRIMARY KEY,
    user_id      TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    cred_id      TEXT NOT NULL UNIQUE,      -- base64url credential id
    passkey_json TEXT NOT NULL,
    created_at   TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_passkeys_user ON passkeys(user_id);

-- 관심종목.
CREATE TABLE IF NOT EXISTS watchlist (
    user_id  TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    symbol   TEXT NOT NULL,
    market   TEXT,
    sort     INTEGER NOT NULL DEFAULT 0,
    added_at TEXT NOT NULL,
    PRIMARY KEY (user_id, symbol)
);
