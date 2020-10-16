//! Kraken asynchronous HTTP client.
//!
//! [https://www.kraken.com/features/api](https://www.kraken.com/features/api)

pub use api::{Api, Response, ResponseValue};
pub use assets::Asset;
pub use auth::Credentials;
pub use client::Client;
pub use error::Error;

pub mod api;
pub mod client;

mod assets;
mod auth;
mod error;

/// Crate Result type.
type Result<T> = std::result::Result<T, Error>;

/// Kraken REST API domain.
const KRAKEN_DOMAIN: &str = "https://api.kraken.com";

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn server_time() -> Result<()> {
        let client = Client::default();

        let api: Api = api::public::time().into();
        println!("{}", api);

        let resp: ResponseValue = client.send(api).await?;
        println!("{:?}", resp);
        assert!(resp.error.is_empty());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn assets_info() -> Result<()> {
        let client = Client::default();
        let assets = [Asset::XBT, Asset::EUR, Asset::ETH];

        let asset = assets
            .iter()
            .map(|a| a.with_prefix())
            .collect::<Vec<String>>()
            .join(",");
        let api: Api = api::public::assets().with("asset", asset).into();
        println!("{}", api);

        let resp: ResponseValue = client.send(api).await?;
        println!("{:?}", resp);
        assert!(resp.error.is_empty());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn asset_pairs() -> Result<()> {
        let client = Client::default();

        let assets = api::public::asset_pairs();
        let asset_pair = Asset::XBT.pair(Asset::EUR);
        let api: Api = assets.with("pair", &asset_pair).into();
        println!("{}", api);

        let resp: ResponseValue = client.send(api).await?;
        println!("{:?}", resp);
        assert!(resp.error.is_empty());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn ticker_info() -> Result<()> {
        let client = Client::default();

        let ticker = api::public::ticker();
        let asset_pair = Asset::XBT.pair(Asset::EUR);
        let api: Api = ticker.with("pair", &asset_pair).into();
        println!("{}", api);

        let resp: ResponseValue = client.send(api).await?;
        println!("{:?}", resp);
        assert!(resp.error.is_empty());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn ohlc() -> Result<()> {
        let client = Client::default();

        let ohlc = api::public::ohlc();
        let asset_pair = Asset::XBT.pair(Asset::GBP);
        let api: Api = ohlc.with("pair", &asset_pair).into();
        println!("{}", api);

        let resp: ResponseValue = client.send(api).await?;
        println!("{:?}", resp);
        assert!(resp.error.is_empty());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn depth() -> Result<()> {
        let client = Client::default();

        let depth = api::public::depth();
        let asset_pair = Asset::XBT.pair(Asset::GBP);
        let api: Api = depth.with("pair", &asset_pair).with("count", 2).into();
        println!("{}", api);

        let resp: ResponseValue = client.send(api).await?;
        println!("{:?}", resp);
        assert!(resp.error.is_empty());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn trades() -> Result<()> {
        let client = Client::default();

        let trades = api::public::trades();
        let asset_pair = Asset::XBT.pair(Asset::USD);
        let api: Api = trades.with("pair", &asset_pair).into();
        println!("{}", api);

        let resp: ResponseValue = client.send(api).await?;
        println!("{:?}", resp);
        assert!(resp.error.is_empty());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn spread() -> Result<()> {
        let client = Client::default();

        let spread = api::public::spread();
        let asset_pair = Asset::XBT.pair(Asset::USD);
        let api: Api = spread.with("pair", &asset_pair).into();
        println!("{}", api);

        let resp: ResponseValue = client.send(api).await?;
        println!("{:?}", resp);
        assert!(resp.error.is_empty());
        assert!(resp.result.is_some());

        Ok(())
    }
}
