//! Kraken asynchronous HTTP client.
//!
//!
//! ## How to use akkorokamui
//!
//! Add `akkorokamui` to the list of your dependencies:
//!
//! ```toml
//! akkorokamui = { git = "https://github.com/gliderkite/akkorokamui.git" }
//! ```
//!
//! The HTTP client is based on [reqwest](https://github.com/seanmonstar/reqwest),
//! therefore you'll be able to use [tokio](https://github.com/tokio-rs/tokio) as
//! your asynchronous runtime:
//!
//! ```toml
//! tokio = { version = "0.2", features = ["full"] }
//! ```
//!
//! ## Examples
//!
//! ### Create a client without credentials (server time)
//!
//! There are two possible ways to construct a client: with or without credentials,
//! but be aware that without credentials you will only have access to the public
//! APIs.
//!
//! According to the [Kraken APIs documentation](https://www.kraken.com/features/api#general-usage),
//! all the HTTP responses will contain two fields:
//!
//! ```json
//! error = array of error messages
//! result = result of API call (may not be present if errors occur)
//! ```
//!
//! The `ResponseValue` type is the most general type of response that can be
//! returned by the client and it mirrors the above description, where the `result`,
//! if present, will be encoded in a
//! [serde_json::Value](https://docs.serde.rs/serde_json/value/enum.Value.html) enum.
//!
//! ```no_run
//! use akkorokamui::{api, Api, client, Client, ResponseValue};
//! use anyhow::Result;
//! use std::convert::TryInto;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let user_agent = "<product>/<product-version>";
//!     let client: Client = client::with_user_agent(user_agent).try_into()?;
//!
//!     let api: Api = api::public::time().into();
//!     let resp: ResponseValue = client.send(api).await?;
//!     println!("{:?}", resp);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Extract fields from a generic response (server time `unixtime`)
//!
//! You can extract any field from the JSON response using the `serde_json::Value`
//! available methods.
//!
//! ```no_run
//! use akkorokamui::{api, client, Api, Client, ResponseValue};
//! use anyhow::Result;
//! use std::convert::TryInto;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let user_agent = "<product>/<product-version>";
//!     let client: Client = client::with_user_agent(user_agent).try_into()?;
//!
//!     let api: Api = api::public::time().into();
//!     let resp: ResponseValue = client.send(api).await?;
//!     println!("{:?}", resp);
//!
//!     if let Some(result) = resp.result {
//!         let time = result.get("unixtime").and_then(|t| t.as_u64());
//!         println!("Time: {:?}", time);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Deserialize a response into a user defined type (server time `unixtime`)
//!
//! The `ResponseValue` allows not to worry too much on the JSON structure returned
//! by the Kraken APIs, while not being tied too much on the types defined in this
//! library. But if you want to exploit the type safety of your user defined types
//! you can do so by defining your own `Response<T>`.
//!
//! ```no_run
//!use akkorokamui::{api, client, Api, Client, Response};
//!use anyhow::Result;
//!use serde::Deserialize;
//!use std::convert::TryInto;
//!
//!#[tokio::main]
//!async fn main() -> Result<()> {
//!    let user_agent = "<product>/<product-version>";
//!    let client: Client = client::with_user_agent(user_agent).try_into()?;
//!
//!    #[derive(Debug, Deserialize)]
//!    struct Time {
//!        unixtime: u64,
//!    }
//!
//!    let api: Api = api::public::time().into();
//!    let resp: Response<Time> = client.send(api).await?;
//!    println!("{:?}", resp);
//!
//!    if let Some(result) = resp.result {
//!        println!("Time: {}", result.unixtime);
//!    }
//!
//!    Ok(())
//!}
//! ```
//!
//! ### Specify API parameters (recent trades)
//!
//! The API builder allows to specify any key-value pair as new parameter via the
//! method `.with(key, value)`, which can be chained for as many parameters are
//! needed:
//!
//! ```no_run
//! use akkorokamui::{api, client, Api, Asset, Client, Response};
//! use anyhow::Result;
//! use serde::Deserialize;
//! use std::collections::HashMap;
//! use std::convert::TryInto;
//! use std::time::{Duration, SystemTime};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let user_agent = "<product>/<product-version>";
//!     let client: Client = client::with_user_agent(user_agent).try_into()?;
//!
//!     #[derive(Debug, Deserialize)]
//!     struct Trade {
//!         price: String,
//!         volume: String,
//!         time: f32,
//!         buy_sell: String,
//!         market_limit: String,
//!         miscellaneous: String,
//!     }
//!
//!     #[derive(Debug, Deserialize)]
//!     struct Trades {
//!         #[serde(flatten)]
//!         asset_pair_trades: HashMap<String, Vec<Trade>>,
//!         last: String,
//!     }
//!
//!     let now = SystemTime::now();
//!     let since = now.checked_sub(Duration::from_secs(10)).unwrap();
//!     let since = since.elapsed()?.as_secs();
//!
//!     // NOTE: the asset pair name may need to use the X and Z prefix depending
//!     // on the Kraken classification system, where X stands for cryptocurrency
//!     // based assets while Z is for fiat based assets. You can build a map of
//!     // pair alternative name to asset pair effective name by querying all the
//!     // AssetPairs from the homonymous API.
//!     let asset_pair = Asset::XBT.pair(Asset::EUR);
//!     let api: Api = api::public::trades()
//!         .with("pair", &asset_pair)
//!         .with("since", since)
//!         .into();
//!
//!     let resp: Response<Trades> = client.send(api).await?;
//!     println!("{:?}", resp);
//!
//!     if let Some(result) = resp.result {
//!         if let Some(trades) = result.asset_pair_trades.get(&asset_pair) {
//!             for trade in trades {
//!                 println!("price at {}: {}", trade.time, trade.price);
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Client credentials and private APIs (account balance)
//!
//! In order to use private APIs you need to own a pair of keys: the public API key
//! as well as your private key (refer to the
//! [Kraken support page](https://support.kraken.com/hc/en-us/articles/360000919966-How-to-generate-an-API-key-pair)
//! to learn how to generate these keys).
//!
//! The keys must be stored in a single file, where the first line contains the
//! public API key and the second line contains the private key.
//!
//! ```no_run
//! use akkorokamui::{api, client, Api, Asset, Client, Credentials, Response};
//! use anyhow::Result;
//! use std::collections::HashMap;
//! use std::convert::TryInto;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let keys_path = "kraken.key";
//!     let credentials = Credentials::read(keys_path)?;
//!
//!     let user_agent = "<product>/<product-version>";
//!     let client: Client = client::with_user_agent(user_agent)
//!         .with_credentials(credentials)
//!         .try_into()?;
//!
//!     let api: Api = api::private::balance().into();
//!     let resp: Response<HashMap<String, String>> =
//!         client.send(api).await?;
//!     println!("{:?}", resp);
//!
//!     if let Some(result) = resp.result {
//!         println!("USD: {:?}", result.get(&Asset::USD.with_prefix()));
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Add a new order (validate)
//!
//! ```no_run
//! use akkorokamui::{
//!     api, client, Api, Asset, Client, Credentials, Order, OrderType, Response,
//!     ResponseValue,
//! };
//! use anyhow::{bail, Result};
//! use serde::Deserialize;
//! use std::collections::HashMap;
//! use std::convert::TryInto;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let keys_path = "kraken.key";
//!     let credentials = Credentials::read(keys_path)?;
//!
//!     let user_agent = "<product>/<product-version>";
//!     let client: Client = client::with_user_agent(user_agent)
//!         .with_credentials(credentials)
//!         .try_into()?;
//!
//!     let asset_pairs = get_asset_pairs(&client).await?;
//!     let pair = Asset::XRP.pair(Asset::GBP);
//!     let xrp_gbp = if let Some(name) = asset_pairs.get(&pair) {
//!         name
//!     } else {
//!         bail!("{} asset pair name not found", pair)
//!     };
//!
//!     let api: Api = api::private::add_order()
//!         // validate only, do not actually place any order
//!         .with("validate", true)
//!         .with("pair", &xrp_gbp)
//!         .with("type", Order::Buy)
//!         .with("ordertype", OrderType::TakeProfitLimit)
//!         // take profit price trigger
//!         .with("price", 0.19)
//!         // limit price
//!         .with("price2", 0.191)
//!         .with("volume", 30)
//!         // prefer fee in quote currency
//!         .with("oflags", "fciq")
//!         .into();
//!
//!     let resp: ResponseValue = client.send(api).await?;
//!     println!("{:?}", resp);
//!
//!     Ok(())
//! }
//!
//! async fn get_asset_pairs(client: &Client) -> Result<HashMap<String, String>> {
//!     #[derive(Debug, Deserialize)]
//!     struct AssetPair {
//!         altname: String,
//!     }
//!
//!     type AssetPairs = HashMap<String, AssetPair>;
//!
//!     let api: Api = api::public::asset_pairs().into();
//!     let resp: Response<AssetPairs> = client.send(api).await?;
//!
//!     if let Some(result) = resp.result {
//!         Ok(result.into_iter().map(|(k, v)| (v.altname, k)).collect())
//!     } else {
//!         Ok(HashMap::new())
//!     }
//! }
//! ```

pub use api::{Api, Response, ResponseValue};
pub use assets::Asset;
pub use auth::Credentials;
pub use client::Client;
pub use error::Error;
pub use order::{Order, OrderType};

pub mod api;
pub mod client;

mod assets;
mod auth;
mod error;
mod order;

/// Crate Result type.
type Result<T> = std::result::Result<T, Error>;

/// Kraken REST API domain.
const KRAKEN_DOMAIN: &str = "https://api.kraken.com";

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use client::Client;

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
