use serde::{Deserialize, Serialize};
use std::fmt;

/// List of crypto and fiat currencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Asset {
    // Crypto currencies
    ADA,
    ALGO,
    ATOM,
    BAL,
    BAT,
    BCH,
    COMP,
    CRV,
    DAI,
    DASH,
    DOT,
    EOS,
    ETC,
    ETH,
    FIL,
    GNO,
    ICX,
    KAVA,
    KNC,
    KSM,
    LINK,
    LSK,
    LTC,
    MLN,
    NANO,
    OMG,
    OXT,
    PAXG,
    QTUM,
    REP,
    REPV2,
    SC,
    SNX,
    STORJ,
    TRX,
    UNI,
    USDC,
    USDT,
    WAVES,
    XBT,
    XDG,
    XLM,
    XMR,
    XRP,
    XTZ,
    YFI,
    ZEC,
    // Fiat currencies
    AUD,
    EUR,
    GBP,
    USD,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Asset {
    /// Gets the string representation of the asset pair.
    pub fn pair(self, other: Self) -> String {
        if self.is_fiat() != other.is_fiat() {
            // crypto with fiat (or vice-versa)
            format!("{}{}", self.with_prefix(), other.with_prefix())
        } else {
            // either both crypto or both fiat
            format!("{}{}", self, other)
        }
    }

    /// Gets a new string representing the asset with the crypto/fiat prefix.
    pub fn with_prefix(self) -> String {
        if self.is_fiat() {
            format!("Z{}", self)
        } else {
            debug_assert!(self.is_crypto());
            format!("X{}", self)
        }
    }

    /// Returns true only if this asset is a crypto currency.
    pub fn is_crypto(self) -> bool {
        !self.is_fiat()
    }

    /// Returns true only if this asset is a fiat currency.
    pub fn is_fiat(self) -> bool {
        matches!(self, Self::USD | Self::EUR | Self::GBP | Self::AUD)
    }
}
