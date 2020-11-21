use serde::{Deserialize, Serialize};
use std::fmt;

/// Order to buy or sell the asset.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    Ord,
    PartialOrd,
    Hash,
)]
pub enum Order {
    Buy,
    Sell,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let order_type = match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        };
        write!(f, "{}", order_type)
    }
}

/// Enumeration of order types.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    Ord,
    PartialOrd,
    Hash,
)]
pub enum OrderType {
    /// Buy/Sell asset at the best market price.
    Market,
    /// Buy/Sell at a fixed price per asset.
    Limit,
    /// Settle position(s) at the original order price.
    SettlePosition,
    /// Buy at market once last price is >= stop price.
    /// Sell at market once last price is <= stop price.
    StopLoss,
    /// Buy at a fixed price once last price >= stop price.
    /// Sell at a fixed price once last price <= stop price.
    StopLossLimit,
    /// Buy at market once last price <= take profit price.
    /// Sell at market once last price >= take profit price.
    TakeProfit,
    /// Buy at a fixed price once market price <= take profit price.
    /// Sell at a fixed price once market price >= take profit price.
    TakeProfitLimit,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let order_type = match self {
            Self::Market => "market",
            Self::Limit => "limit",
            Self::SettlePosition => "settle-position",
            Self::StopLoss => "stop-loss",
            Self::TakeProfit => "take-profit",
            Self::StopLossLimit => "stop-loss-limit",
            Self::TakeProfitLimit => "take-profit-limit",
        };
        write!(f, "{}", order_type)
    }
}
