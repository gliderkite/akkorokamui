use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::{Error, Result};

/// List of crypto and fiat currencies.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
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

impl FromStr for Asset {
    type Err = Error;

    fn from_str(asset: &str) -> Result<Self> {
        let asset = match asset.to_ascii_uppercase().as_str() {
            // Crypto currencies
            "ADA" => Self::ADA,
            "ALGO" => Self::ALGO,
            "ATOM" => Self::ATOM,
            "BAL" => Self::BAL,
            "BAT" => Self::BAT,
            "BCH" => Self::BCH,
            "COMP" => Self::COMP,
            "CRV" => Self::CRV,
            "DAI" => Self::DAI,
            "DASH" => Self::DASH,
            "DOT" => Self::DOT,
            "EOS" => Self::EOS,
            "ETC" => Self::ETC,
            "ETH" => Self::ETH,
            "FIL" => Self::FIL,
            "GNO" => Self::GNO,
            "ICX" => Self::ICX,
            "KAVA" => Self::KAVA,
            "KNC" => Self::KNC,
            "KSM" => Self::KSM,
            "LINK" => Self::LINK,
            "LSK" => Self::LSK,
            "LTC" => Self::LTC,
            "MLN" => Self::MLN,
            "NANO" => Self::NANO,
            "OMG" => Self::OMG,
            "OXT" => Self::OXT,
            "PAXG" => Self::PAXG,
            "QTUM" => Self::QTUM,
            "REP" => Self::REP,
            "REPV2" => Self::REPV2,
            "SC" => Self::SC,
            "SNX" => Self::SNX,
            "STORJ" => Self::STORJ,
            "TRX" => Self::TRX,
            "UNI" => Self::UNI,
            "USDC" => Self::USDC,
            "USDT" => Self::USDT,
            "WAVES" => Self::WAVES,
            "XBT" => Self::XBT,
            "XDG" => Self::XDG,
            "XLM" => Self::XLM,
            "XMR" => Self::XMR,
            "XRP" => Self::XRP,
            "XTZ" => Self::XTZ,
            "YFI" => Self::YFI,
            "ZEC" => Self::ZEC,
            // Fiat currencies
            "AUD" => Self::AUD,
            "EUR" => Self::EUR,
            "GBP" => Self::GBP,
            "USD" => Self::USD,
            _ => return Err(Error::unknown_asset(asset)),
        };
        Ok(asset)
    }
}

impl Asset {
    /// Gets the alternate pair name.
    ///
    /// # NOTE
    /// Sometimes, the asset pair name may need to use the X and Z prefix depending
    /// on the Kraken classification system, where X stands for cryptocurrency
    /// based assets while Z is for fiat based assets. You can build a map of
    /// pair alternative names to asset pair effective names by querying all the
    /// AssetPairs from the homonymous API.
    pub fn pair(self, other: Self) -> String {
        format!("{}{}", self, other)
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
