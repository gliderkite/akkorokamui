use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::{Error, Result};

/// Enumeration of assets.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Hash,
)]
pub enum Asset {
    // Crypto currencies
    ADA,   // Cardano
    ALGO,  // Algorand
    ATOM,  // Cosmos
    BAL,   // Balancer
    BAT,   // Basic Attention Token
    BCH,   // Bitcoin Cash
    COMP,  // Compound
    CRV,   // Curve
    DAI,   // Dai
    DASH,  // DASH
    DOT,   // DOT
    EOS,   // EOS
    FIL,   // Filecoin
    FLOW,  // Flow
    GNO,   // Gnosis
    ICX,   // ICON
    KAVA,  // Kava
    KNC,   // Kyber Network
    KSM,   // Kusama
    LINK,  // Chainlink
    LSK,   // Lisk
    OMG,   // OmiseGO
    OXT,   // Orchid
    PAXG,  // PAX Gold
    QTUM,  // QTUM
    SC,    // Siacoin
    SNX,   // Synthetix
    STORJ, // Storj
    TRX,   // Tron
    UNI,   // Uniswap
    USDC,  // USD Coin
    USDT,  // Tether
    WAVE,  // Waves
    XETC,  // Ethereum Classic
    XETH,  // Ethereum
    XLTC,  // Litecoin
    XMLN,  // Melon
    XREP,  // Augur
    REPV2, // Augur v2
    XXBT,  // Bitcoin
    XXDG,  // Dogecoin
    XXLM,  // Stellar Lumens
    XXMR,  // Monero
    XXRP,  // Ripple
    XTZ,   // Tezos
    XZEC,  // Zcash
    YFI,   // Yearn Finance
    ZEC,   // Zcash
    NANO,  // Nano
    WAVES, // Waves
    // Fiat currencies
    CHF,  // Swiss Franc
    ZAUD, // Australian dollar
    ZCAD, // Canadian dollar
    ZEUR, // Euro
    ZGBP, // Great British Pound
    ZJPY, // Japanese Yen
    ZUSD, // US Dollar
    // Kraken Fee Credits
    KFEE, // Promotional Credit
    // Unknown asset
    #[serde(other)]
    Unknown,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "<unknown asset>"),
            _ => write!(f, "{:?}", self),
        }
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
            "FIL" => Self::FIL,
            "FLOW" => Self::FLOW,
            "GNO" => Self::GNO,
            "ICX" => Self::ICX,
            "KAVA" => Self::KAVA,
            "KNC" => Self::KNC,
            "KSM" => Self::KSM,
            "LINK" => Self::LINK,
            "LSK" => Self::LSK,
            "NANO" => Self::NANO,
            "OMG" => Self::OMG,
            "OXT" => Self::OXT,
            "PAXG" => Self::PAXG,
            "QTUM" => Self::QTUM,
            "REPV2" => Self::REPV2,
            "SC" => Self::SC,
            "SNX" => Self::SNX,
            "STORJ" => Self::STORJ,
            "TRX" => Self::TRX,
            "UNI" => Self::UNI,
            "USDC" => Self::USDC,
            "USDT" => Self::USDT,
            "WAVE" => Self::WAVE,
            "WAVES" => Self::WAVES,
            "XETC" => Self::XETC,
            "XETH" => Self::XETH,
            "XLTC" => Self::XLTC,
            "XMLN" => Self::XMLN,
            "XREP" => Self::XREP,
            "XTZ" => Self::XTZ,
            "XXBT" => Self::XXBT,
            "XXDG" => Self::XXDG,
            "XXLM" => Self::XXLM,
            "XXMR" => Self::XXMR,
            "XXRP" => Self::XXRP,
            "XZEC" => Self::XZEC,
            "YFI" => Self::YFI,
            "ZEC" => Self::ZEC,
            // Fiat currencies
            "CHF" => Self::CHF,
            "ZAUD" => Self::ZAUD,
            "ZCAD" => Self::ZCAD,
            "ZEUR" => Self::ZEUR,
            "ZGBP" => Self::ZGBP,
            "ZJPY" => Self::ZJPY,
            "ZUSD" => Self::ZUSD,
            // Kraken Fee Credits
            "KFEE" => Self::KFEE,
            _ => return Err(Error::unknown_asset(asset)),
        };
        Ok(asset)
    }
}

impl Asset {
    /// Gets the asset pair name.
    pub fn pair(self, other: Self) -> String {
        format!("{}{}", self, other)
    }

    /// Returns true only if this asset is a crypto currency.
    pub fn is_crypto(self) -> bool {
        !(self.is_fiat() || self.is_kraken_credit() || self.is_unknown())
    }

    /// Returns true only if this asset is a fiat currency.
    pub fn is_fiat(self) -> bool {
        matches!(
            self,
            Self::ZAUD
                | Self::ZCAD
                | Self::ZEUR
                | Self::ZGBP
                | Self::ZJPY
                | Self::ZUSD
                | Self::CHF
        )
    }

    /// Returns true only if this asset is a Kraken fee credit.
    pub fn is_kraken_credit(self) -> bool {
        self == Self::KFEE
    }

    /// Returns true only if this asset is unknown.
    pub fn is_unknown(self) -> bool {
        self == Self::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{api, Client, Response};
    use anyhow::{bail, Result};
    use rand::{distributions::Alphanumeric, thread_rng, Rng};
    use std::{collections::HashMap, iter};

    #[derive(Debug, Deserialize)]
    struct AssetPair<T> {
        base: T,
        quote: T,
    }

    #[test]
    fn asset_pairs() -> Result<()> {
        let client = Client::default();

        type AssetPairs = HashMap<String, AssetPair<Asset>>;

        let api = api::public::asset_pairs();
        let resp: Response<AssetPairs> = client.send(api)?;
        assert!(resp.is_success());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[test]
    fn asset_from_str() -> Result<()> {
        let client = Client::default();

        type AssetPairs = HashMap<String, AssetPair<String>>;

        let api = api::public::asset_pairs();
        let resp: Response<AssetPairs> = client.send(api)?;
        assert!(resp.is_success());

        if let Some(asset_pairs) = resp.result {
            for asset_pair in asset_pairs.values() {
                let base = asset_pair.base.parse::<Asset>()?;
                assert!(!base.is_unknown());
                let quote = asset_pair.quote.parse::<Asset>()?;
                assert!(!quote.is_unknown());
            }
        } else {
            bail!("No asset pairs in response result");
        }

        Ok(())
    }

    #[test]
    fn asset_deserialize() -> Result<()> {
        let xbt: Asset = serde_json::from_str(r#""XXBT""#)?;
        assert_eq!(Asset::XXBT, xbt);
        assert!(xbt.is_crypto());
        assert!(!(xbt.is_fiat() || xbt.is_kraken_credit() || xbt.is_unknown()));

        let eur: Asset = serde_json::from_str(r#""ZEUR""#)?;
        assert_eq!(Asset::ZEUR, eur);
        assert!(eur.is_fiat());
        assert!(
            !(eur.is_crypto() || eur.is_kraken_credit() || eur.is_unknown())
        );

        let kfee: Asset = serde_json::from_str(r#""KFEE""#)?;
        assert_eq!(Asset::KFEE, kfee);
        assert!(kfee.is_kraken_credit());
        assert!(!(kfee.is_crypto() || kfee.is_fiat() || kfee.is_unknown()));

        let mut rng = thread_rng();
        let unknown = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(4)
            .collect::<String>()
            .to_ascii_uppercase();
        println!("Deserializing unknown asset: {}", unknown);
        let unknown: Asset =
            serde_json::from_str(&format!(r#""{}""#, unknown))?;
        assert_eq!(Asset::Unknown, unknown);
        assert!(unknown.is_unknown());
        assert!(
            !(unknown.is_crypto()
                || unknown.is_kraken_credit()
                || unknown.is_fiat())
        );

        Ok(())
    }
}
