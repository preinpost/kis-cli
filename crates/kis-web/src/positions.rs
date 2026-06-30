//! 매매일지(trades)에서 포지션·실현손익을 파생 (평균단가법). 순수 로직 — 단위테스트 대상.
//!
//! 통화가 다르면(원/달러) 손익을 섞지 않도록 (symbol, currency) 단위로 집계한다.
//! v1 롱 전용 가정: 과매도(보유보다 많이 매도) 시 잔여수량/원가를 0으로 클램프한다.

use std::collections::HashMap;

/// 계산 입력용 체결 1건 (DB trades 행의 부분집합).
#[derive(Clone, Debug)]
pub struct TradeRow {
    pub traded_at: String,
    pub symbol: String,
    pub name: Option<String>,
    pub market: Option<String>,
    pub broker: Option<String>,
    pub side: String, // "buy" | "sell"
    pub quantity: f64,
    pub price: f64,
    pub fee: f64,
    pub currency: String,
}

/// 파생된 종목 포지션.
#[derive(Clone, Debug)]
pub struct Position {
    pub symbol: String,
    pub name: Option<String>,
    pub market: Option<String>,
    pub broker: Option<String>,
    pub currency: String,
    pub quantity: f64,     // 잔여 수량
    pub avg_cost: f64,     // 평균 매입단가
    pub realized_pnl: f64, // 실현 손익
    pub buy_count: usize,
    pub sell_count: usize,
    pub win_count: usize, // 이익 실현 매도 횟수
}

/// (symbol, currency) 별 평균단가법 집계. traded_at 오름차순 처리.
pub fn positions_from_trades(trades: &[TradeRow]) -> Vec<Position> {
    // 그룹 키 보존을 위해 입력 순서 기반 그룹핑
    let mut groups: HashMap<(String, String), Vec<&TradeRow>> = HashMap::new();
    let mut order: Vec<(String, String)> = Vec::new();
    for t in trades {
        let key = (t.symbol.clone(), t.currency.clone());
        groups.entry(key.clone()).or_insert_with(|| {
            order.push(key.clone());
            Vec::new()
        });
        groups.get_mut(&key).unwrap().push(t);
    }

    let mut out = Vec::new();
    for key in &order {
        let mut rows = groups.remove(key).unwrap();
        rows.sort_by(|a, b| a.traded_at.cmp(&b.traded_at));

        let mut qty = 0.0f64;
        let mut cost_total = 0.0f64; // 잔여수량 * 평단
        let mut realized = 0.0f64;
        let (mut buy_count, mut sell_count, mut win_count) = (0usize, 0usize, 0usize);
        let (mut name, mut market, mut broker) = (None, None, None);

        for t in &rows {
            if t.name.is_some() {
                name = t.name.clone();
            }
            if t.market.is_some() {
                market = t.market.clone();
            }
            if t.broker.is_some() {
                broker = t.broker.clone();
            }

            if t.side == "sell" {
                let avg = if qty > 0.0 { cost_total / qty } else { 0.0 };
                let pnl = (t.price - avg) * t.quantity - t.fee;
                realized += pnl;
                sell_count += 1;
                if pnl > 0.0 {
                    win_count += 1;
                }
                let reduce = t.quantity.min(qty);
                cost_total -= avg * reduce;
                qty -= t.quantity;
                if qty <= 0.0 {
                    qty = 0.0;
                    cost_total = 0.0;
                }
            } else {
                // buy (기본)
                cost_total += t.quantity * t.price + t.fee;
                qty += t.quantity;
                buy_count += 1;
            }
        }

        out.push(Position {
            symbol: key.0.clone(),
            name,
            market,
            broker,
            currency: key.1.clone(),
            quantity: qty,
            avg_cost: if qty > 0.0 { cost_total / qty } else { 0.0 },
            realized_pnl: realized,
            buy_count,
            sell_count,
            win_count,
        });
    }
    out
}

/// 매매 통계 (통화별 실현손익 + 전체 카운트·승률).
#[derive(Clone, Debug)]
pub struct TradeStats {
    pub trade_count: usize,
    pub buy_count: usize,
    pub sell_count: usize,
    pub win_count: usize,
    pub symbol_count: usize,
    /// 통화별 실현손익 (currency, amount)
    pub realized_by_currency: Vec<(String, f64)>,
}

pub fn stats_from_trades(trades: &[TradeRow]) -> TradeStats {
    let positions = positions_from_trades(trades);
    let mut realized: HashMap<String, f64> = HashMap::new();
    let mut realized_order: Vec<String> = Vec::new();
    let (mut buy_count, mut sell_count, mut win_count) = (0usize, 0usize, 0usize);
    let mut symbols: std::collections::HashSet<String> = std::collections::HashSet::new();

    for p in &positions {
        buy_count += p.buy_count;
        sell_count += p.sell_count;
        win_count += p.win_count;
        symbols.insert(p.symbol.clone());
        realized
            .entry(p.currency.clone())
            .and_modify(|v| *v += p.realized_pnl)
            .or_insert_with(|| {
                realized_order.push(p.currency.clone());
                p.realized_pnl
            });
    }

    TradeStats {
        trade_count: trades.len(),
        buy_count,
        sell_count,
        win_count,
        symbol_count: symbols.len(),
        realized_by_currency: realized_order
            .into_iter()
            .map(|c| {
                let v = realized[&c];
                (c, v)
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(at: &str, side: &str, qty: f64, price: f64, fee: f64) -> TradeRow {
        TradeRow {
            traded_at: at.into(),
            symbol: "005930".into(),
            name: Some("삼성전자".into()),
            market: Some("domestic".into()),
            broker: Some("KIS".into()),
            side: side.into(),
            quantity: qty,
            price,
            fee,
            currency: "KRW".into(),
        }
    }

    #[test]
    fn avg_cost_and_realized() {
        // 10주 @100, 10주 @200 → 평단 150, 20주.  5주 @300 매도 → 실현 (300-150)*5 = 750.
        let trades = vec![
            t("2026-01-01", "buy", 10.0, 100.0, 0.0),
            t("2026-01-02", "buy", 10.0, 200.0, 0.0),
            t("2026-01-03", "sell", 5.0, 300.0, 0.0),
        ];
        let p = positions_from_trades(&trades);
        assert_eq!(p.len(), 1);
        assert!((p[0].avg_cost - 150.0).abs() < 1e-9);
        assert!((p[0].quantity - 15.0).abs() < 1e-9);
        assert!((p[0].realized_pnl - 750.0).abs() < 1e-9);
        assert_eq!(p[0].win_count, 1);
    }

    #[test]
    fn fee_reduces_realized_and_loss() {
        // 10주 @100 매수(수수료10) → 평단 (1000+10)/10=101.  10주 @90 매도(수수료10) → (90-101)*10-10 = -120.
        let trades = vec![
            t("2026-01-01", "buy", 10.0, 100.0, 10.0),
            t("2026-01-02", "sell", 10.0, 90.0, 10.0),
        ];
        let p = positions_from_trades(&trades);
        assert!((p[0].avg_cost - 0.0).abs() < 1e-9); // 전량 매도 → 잔여 0
        assert!((p[0].quantity).abs() < 1e-9);
        assert!((p[0].realized_pnl - (-120.0)).abs() < 1e-9);
        assert_eq!(p[0].win_count, 0);
        assert_eq!(p[0].sell_count, 1);
    }

    #[test]
    fn currency_separated() {
        let mut a = t("2026-01-01", "buy", 1.0, 100.0, 0.0);
        a.symbol = "TSLA".into();
        a.currency = "USD".into();
        let trades = vec![t("2026-01-01", "buy", 10.0, 100.0, 0.0), a];
        let s = stats_from_trades(&trades);
        assert_eq!(s.symbol_count, 2);
        assert_eq!(s.realized_by_currency.len(), 2); // KRW, USD 분리
    }

    #[test]
    fn win_rate_counts() {
        let trades = vec![
            t("2026-01-01", "buy", 10.0, 100.0, 0.0),
            t("2026-01-02", "sell", 5.0, 120.0, 0.0), // +이익
            t("2026-01-03", "sell", 5.0, 80.0, 0.0),  // -손실
        ];
        let s = stats_from_trades(&trades);
        assert_eq!(s.sell_count, 2);
        assert_eq!(s.win_count, 1);
    }
}
