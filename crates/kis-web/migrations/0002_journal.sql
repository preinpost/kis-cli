-- 매매일지 + 포트폴리오 관리 (수동 우선).

-- 매매일지(수동). 어느 증권사 거래든 1체결 1행.
CREATE TABLE IF NOT EXISTS trades (
    id         TEXT PRIMARY KEY,
    user_id    TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    traded_at  TEXT NOT NULL,                 -- 체결일시(RFC3339 또는 날짜)
    symbol     TEXT NOT NULL,                 -- 정규화 코드/티커
    name       TEXT,                          -- 표시명 스냅샷
    market     TEXT,                          -- domestic|overseas|other
    broker     TEXT,                          -- 'KIS'|'TOSS'|'기타'
    side       TEXT NOT NULL,                 -- 'buy'|'sell'
    quantity   REAL NOT NULL,
    price      REAL NOT NULL,
    fee        REAL NOT NULL DEFAULT 0,
    currency   TEXT NOT NULL DEFAULT 'KRW',
    reason     TEXT,                          -- 매매 사유
    tags       TEXT,                          -- 콤마구분 태그
    memo       TEXT,                          -- 회고/메모
    created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_trades_user ON trades(user_id, traded_at DESC);
CREATE INDEX IF NOT EXISTS idx_trades_symbol ON trades(user_id, symbol);

-- 종목별 관리 메타(KIS·수동 보유 공용).
CREATE TABLE IF NOT EXISTS holding_meta (
    user_id       TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    symbol        TEXT NOT NULL,
    memo          TEXT,
    target_price  REAL,
    stop_price    REAL,
    target_weight REAL,                       -- 목표 비중(%)
    updated_at    TEXT NOT NULL,
    PRIMARY KEY (user_id, symbol)
);
