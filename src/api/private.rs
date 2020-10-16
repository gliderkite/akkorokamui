use std::fmt;

use crate::api::ApiBuilder;

/// List of private methods.
#[derive(Debug, Copy, Clone)]
pub(crate) enum PrivateMethod {
    Balance,
    ClosedOrders,
    OpenOrders,
    QueryLedgers,
    TradeBalance,
}

impl fmt::Display for PrivateMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Get account balance.
pub fn balance() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::Balance)
}

/// Get trace balance.
pub fn trade_balance() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::TradeBalance)
}

/// Get open orders.
pub fn open_orders() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::OpenOrders)
}

/// Get closed orders.
pub fn closed_orders() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::ClosedOrders)
}

/// Query ledgers.
pub fn query_ledgers() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::QueryLedgers)
}
