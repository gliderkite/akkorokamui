use std::fmt;

use crate::api::ApiBuilder;

/// List of public methods.
#[derive(Debug, Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum PublicMethod {
    // Public Market Data
    AssetPairs,
    Assets,
    Depth,
    OHLC,
    Spread,
    SystemStatus,
    Ticker,
    Time,
    Trades,
}

impl fmt::Display for PublicMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Get server time.
pub fn time() -> ApiBuilder {
    ApiBuilder::public(PublicMethod::Time)
}

/// Get asset info.
pub fn assets() -> ApiBuilder {
    ApiBuilder::public(PublicMethod::Assets)
}

/// Get tradable asset pair.
pub fn asset_pairs() -> ApiBuilder {
    ApiBuilder::public(PublicMethod::AssetPairs)
}

/// Get ticker info.
pub fn ticker() -> ApiBuilder {
    ApiBuilder::public(PublicMethod::Ticker)
}

/// Get OHLC info.
pub fn ohlc() -> ApiBuilder {
    ApiBuilder::public(PublicMethod::OHLC)
}

/// Get order book.
pub fn depth() -> ApiBuilder {
    ApiBuilder::public(PublicMethod::Depth)
}

/// Get recent trades.
pub fn trades() -> ApiBuilder {
    ApiBuilder::public(PublicMethod::Trades)
}

/// Get recent spread data.
pub fn spread() -> ApiBuilder {
    ApiBuilder::public(PublicMethod::Spread)
}

/// Get system status.
pub fn system_status() -> ApiBuilder {
    ApiBuilder::public(PublicMethod::SystemStatus)
}
