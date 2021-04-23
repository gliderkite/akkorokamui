//! [Kraken](https://www.kraken.com) HTTP client.
//!
//!
//! ## How to use akkorokamui
//!
//! Add `akkorokamui` to the list of your dependencies:
//!
//! ```toml
//! akkorokamui = "0.4"
//! ```
//!
//! ## Features
//! By default `akkorokamui` will make use of the [rustls](https://github.com/ctz/rustls)
//! transport layer security to connect to HTTPs destinations. If you wish to use
//! the system-native TLS you can do so by specifying the `native-tls` optional
//! feature:
//!
//! ```toml
//! akkorokamui = { version = "0.4", features = ["native-tls"], default-features = false }
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
//! use akkorokamui::{api, Client, ResponseValue};
//! use anyhow::Result;
//!
//! fn main() -> Result<()> {
//!    let user_agent = "<product>/<product-version>";
//!    let client = Client::new(user_agent)?;
//!
//!    let api = api::public::time();
//!    let resp: ResponseValue = client.send(api)?;
//!    println!("{:?}", resp);
//!
//!    Ok(())
//! }
//! ```
//!
//! ### Extract fields from a generic response (server time `unixtime`)
//!
//! You can extract any field from the JSON response using the `serde_json::Value`
//! available methods.
//!
//! ```no_run
//! use akkorokamui::{api, Client, ResponseValue};
//! use anyhow::Result;
//!
//! fn main() -> Result<()> {
//!    let user_agent = "<product>/<product-version>";
//!    let client = Client::new(user_agent)?;
//!
//!    let api = api::public::time();
//!    let resp: ResponseValue = client.send(api)?;
//!    println!("{:?}", resp);
//!
//!    if let Some(result) = resp.result {
//!        let time = result.get("unixtime").and_then(|t| t.as_u64());
//!        println!("Time: {:?}", time);
//!    }
//!
//!    Ok(())
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
//! use akkorokamui::{api, Client, Response};
//! use anyhow::{bail, Result};
//! use serde::Deserialize;
//!
//! fn main() -> Result<()> {
//!    let user_agent = "<product>/<product-version>";
//!    let client = Client::new(user_agent)?;
//!
//!    #[derive(Debug, Deserialize)]
//!    struct Time {
//!        unixtime: u64,
//!    }
//!
//!    let api = api::public::time();
//!    let resp: Response<Time> = client.send(api)?;
//!    println!("{:?}", resp);
//!
//!    if let Some(result) = resp.result {
//!        println!("Time: {}", result.unixtime);
//!    } else {
//!       bail!("Cannot get server time: {:?}", resp.error);
//!    }
//!
//!    Ok(())
//! }
//! ```
//!
//! ### Specify API parameters (recent trades)
//!
//! The API builder allows to specify any key-value pair as new parameter via the
//! method `.with(key, value)`, which can be chained for as many parameters are
//! needed:
//!
//! ```no_run
//! use akkorokamui::{api, Asset, Client, Response};
//! use anyhow::{bail, Result};
//! use serde::Deserialize;
//! use std::{
//!     collections::HashMap,
//!     time::{Duration, SystemTime, UNIX_EPOCH},
//! };
//!
//! fn main() -> Result<()> {
//!     let user_agent = "<product>/<product-version>";
//!     let client = Client::new(user_agent)?;
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
//!     let since = Duration::from_secs(30 * 60);
//!     let since = match SystemTime::now().checked_sub(since) {
//!         Some(since) => since.duration_since(UNIX_EPOCH)?.as_secs(),
//!         _ => bail!("invalid duration"),
//!     };
//!
//!     let asset_pair = Asset::new("XXBT").pair("ZEUR");
//!     let api = api::public::trades()
//!         .with("pair", &asset_pair)
//!         .with("since", since);
//!
//!     let resp: Response<Trades> = client.send(api)?;
//!     println!("{:?}", resp);
//!
//!     if let Some(result) = resp.result {
//!         // note: check GET public/AssetPairs for the actual asset pair name
//!         let asset_pair_name = asset_pair.to_string();
//!         if let Some(trades) = result.asset_pair_trades.get(&asset_pair_name) {
//!             for trade in trades {
//!                 println!("price at {}: {}", trade.time, trade.price);
//!             }
//!         }
//!     } else {
//!         bail!("Cannot get trades: {:?}", resp.error);
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
//! use akkorokamui::{api, Asset, Client, Credentials, Response};
//! use anyhow::{bail, Result};
//! use std::collections::HashMap;
//!
//! type Amount = String;
//! type Balance<'a> = HashMap<Asset<'a>, Amount>;
//!
//! fn main() -> Result<()> {
//!     let keys_path = "kraken.key";
//!     let credentials = Credentials::read(keys_path)?;
//!
//!     let user_agent = "<product>/<product-version>";
//!     let client = Client::with_credentials(user_agent, credentials)?;
//!
//!     let api = api::private::balance();
//!     let resp: Response<Balance> = client.send(api)?;
//!     println!("{:?}", resp);
//!
//!     if let Some(result) = resp.result {
//!         println!("GBP: {:?}", result.get(&Asset::new("ZGBP")));
//!     } else {
//!         bail!("Cannot get balance: {:?}", resp.error);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Add a new order
//!
//! ```no_run
//! use akkorokamui::{
//!     api, Asset, AssetPair, Client, Credentials, Order, OrderType, Response,
//!     ResponseValue,
//! };
//! use anyhow::{bail, Result};
//! use serde::Deserialize;
//! use std::collections::HashMap;
//!
//! fn main() -> Result<()> {
//!     let keys_path = "kraken.key";
//!     let credentials = Credentials::read(keys_path)?;
//!
//!     let user_agent = "<product>/<product-version>";
//!     let client = Client::with_credentials(user_agent, credentials)?;
//!
//!     let asset_pairs = get_asset_pairs(&client)?;
//!     let pair = Asset::new("XXRP").pair("ZGBP");
//!     let xrp_gbp = if let Some(name) = asset_pairs.get(&pair) {
//!         name
//!     } else {
//!         bail!("{} asset pair name not found", pair)
//!     };
//!
//!     let api = api::private::add_order()
//!         // validate only, do not actually place any order
//!         .with("validate", true)
//!         .with("pair", xrp_gbp)
//!         .with("type", Order::Buy)
//!         .with("ordertype", OrderType::TakeProfitLimit)
//!         // take profit price trigger
//!         .with("price", 0.19)
//!         // limit price
//!         .with("price2", 0.191)
//!         .with("volume", 30)
//!         // prefer fee in quote currency
//!         .with("oflags", "fciq");
//!
//!     let resp: ResponseValue = client.send(api)?;
//!     println!("{:?}", resp);
//!
//!     Ok(())
//! }
//!
//! fn get_asset_pairs<'a>(client: &Client) -> Result<HashMap<AssetPair<'a>, String>> {
//!     type AssetPairs<'a> = HashMap<String, AssetPair<'a>>;
//!
//!     let api = api::public::asset_pairs();
//!     let resp: Response<AssetPairs> = client.send(api)?;
//!
//!     if let Some(result) = resp.result {
//!         Ok(result
//!             .into_iter()
//!             .map(|(k, v)| (v.base.pair(v.quote), k))
//!             .collect())
//!     } else {
//!         bail!("Cannot get asset pairs: {:?}", resp.error);
//!     }
//! }
//! ```

pub use api::{Api, Response, ResponseValue};
pub use assets::{Asset, AssetPair};
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

    #[test]
    fn server_time() -> Result<()> {
        let client = Client::default();

        let api = api::public::time();
        println!("{}", api);

        let resp: ResponseValue = client.send(api)?;
        println!("{:?}", resp);
        assert!(resp.is_success());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[test]
    fn system_status() -> Result<()> {
        let client = Client::default();

        let api = api::public::system_status();
        println!("{}", api);

        let resp: ResponseValue = client.send(api)?;
        println!("{:?}", resp);
        assert!(resp.is_success());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[test]
    fn assets_info() -> Result<()> {
        let client = Client::default();
        let assets =
            [Asset::new("XXBT"), Asset::new("ZEUR"), Asset::new("XETH")];

        let asset = assets
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let api = api::public::assets().with("asset", asset);
        println!("{}", api);

        let resp: ResponseValue = client.send(api)?;
        println!("{:?}", resp);
        assert!(resp.is_success());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[test]
    fn asset_pairs() -> Result<()> {
        let client = Client::default();

        let asset_pair = Asset::new("XXBT").pair("ZEUR");
        let api = api::public::asset_pairs().with("pair", &asset_pair);
        println!("{}", api);

        let resp: ResponseValue = client.send(api)?;
        println!("{:?}", resp);
        assert!(resp.is_success());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[test]
    fn ticker_info() -> Result<()> {
        let client = Client::default();

        let asset_pair = Asset::new("XXBT").pair("ZEUR");
        let api = api::public::ticker().with("pair", &asset_pair);
        println!("{}", api);

        let resp: ResponseValue = client.send(api)?;
        println!("{:?}", resp);
        assert!(resp.is_success());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[test]
    fn ohlc() -> Result<()> {
        let client = Client::default();

        let asset_pair = Asset::new("XXBT").pair("ZGBP");
        let api = api::public::ohlc().with("pair", &asset_pair);
        println!("{}", api);

        let resp: ResponseValue = client.send(api)?;
        println!("{:?}", resp);
        assert!(resp.is_success());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[test]
    fn depth() -> Result<()> {
        let client = Client::default();

        let asset_pair = Asset::new("XXBT").pair("ZGBP");
        let api = api::public::depth()
            .with("pair", &asset_pair)
            .with("count", 2);
        println!("{}", api);

        let resp: ResponseValue = client.send(api)?;
        println!("{:?}", resp);
        assert!(resp.is_success());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[test]
    fn trades() -> Result<()> {
        let client = Client::default();

        let asset_pair = Asset::new("XXBT").pair("ZUSD");
        let api = api::public::trades().with("pair", &asset_pair);
        println!("{}", api);

        let resp: ResponseValue = client.send(api)?;
        println!("{:?}", resp);
        assert!(resp.is_success());
        assert!(resp.result.is_some());

        Ok(())
    }

    #[test]
    fn spread() -> Result<()> {
        let client = Client::default();

        let asset_pair = Asset::new("XXBT").pair("ZUSD");
        let api = api::public::spread().with("pair", &asset_pair);
        println!("{}", api);

        let resp: ResponseValue = client.send(api)?;
        println!("{:?}", resp);
        assert!(resp.is_success());
        assert!(resp.result.is_some());

        Ok(())
    }
}
