use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

/// Kraken tradable asset name, such a cryptocurrencies or FIAT.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash,
)]
pub struct Asset<'a>(Cow<'a, str>);

impl<'a> fmt::Display for Asset<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<&'a str> for Asset<'a> {
    fn from(asset: &'a str) -> Self {
        Self(asset.into())
    }
}

impl<'a> From<String> for Asset<'a> {
    fn from(asset: String) -> Self {
        Self(asset.into())
    }
}

impl<'a> Asset<'a> {
    /// Constructs a new Asset.
    pub fn new(asset: impl Into<Cow<'a, str>>) -> Self {
        Self(asset.into())
    }

    /// Constructs a new asset pair using self as base and the given Asset as
    /// quote.
    pub fn pair(self, quote: impl Into<Self>) -> AssetPair<'a> {
        AssetPair {
            base: self,
            quote: quote.into(),
        }
    }
}

/// An asset pair with base and quote assets.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash,
)]
pub struct AssetPair<'a> {
    /// The base component of the asset pair.
    pub base: Asset<'a>,
    /// The quote component of the asset pair.
    pub quote: Asset<'a>,
}

impl<'a> fmt::Display for AssetPair<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.base, self.quote)
    }
}

impl<'a> From<(Asset<'a>, Asset<'a>)> for AssetPair<'a> {
    fn from((base, quote): (Asset<'a>, Asset<'a>)) -> Self {
        Self { base, quote }
    }
}

impl<'a> From<AssetPair<'a>> for (Asset<'a>, Asset<'a>) {
    fn from(asset_pair: AssetPair<'a>) -> Self {
        (asset_pair.base, asset_pair.quote)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{api, Client, Response};
    use anyhow::Result;
    use std::collections::HashMap;

    #[test]
    fn asset_pairs() -> Result<()> {
        let client = Client::default();

        type AssetPairs<'a> = HashMap<String, AssetPair<'a>>;

        let api = api::public::asset_pairs();
        let resp: Response<AssetPairs> = client.send(api)?;
        assert!(resp.is_success());
        println!("{:#?}", resp.result);

        let asset_pairs = resp.result.expect("No asset pairs in response");
        for (_, asset_pair) in asset_pairs {
            let (base, quote): (Asset, Asset) = asset_pair.clone().into();
            assert_eq!(AssetPair::from((base, quote)), asset_pair);
        }

        Ok(())
    }

    #[test]
    fn asset_deserialize() -> Result<()> {
        let xbt: Asset = serde_json::from_str(r#""XXBT""#)?;
        assert_eq!(Asset::new("XXBT"), xbt);
        assert_eq!(xbt.to_string(), "XXBT".to_string());

        let eur: Asset = serde_json::from_str(r#""ZEUR""#)?;
        assert_eq!(Asset::new("ZEUR"), eur);
        assert_eq!(eur.to_string(), "ZEUR".to_string());

        let kfee: Asset = serde_json::from_str(r#""KFEE""#)?;
        assert_eq!(Asset::new("KFEE"), kfee);
        assert_eq!(kfee.to_string(), "KFEE".to_string());

        Ok(())
    }
}
