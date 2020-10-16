# akkorokamui

[akkorokamui](https://en.wikipedia.org/wiki/Akkorokamui) is a
[Kraken](https://www.kraken.com/) asynchronous HTTP(s) client written in Rust.

It currently supports all (TODO!) `v0`
[Kraken REST APIs](https://www.kraken.com/en-gb/features/api).

The main goal of this project is to provide a flexible interface and a safe
implementation.


## How to use akkorokamui

Add `akkorokamui` to the list of your dependencies:

```toml
akkorokamui = { git = "https://github.com/gliderkite/akkorokamui.git" }
```

The HTTP client is based on [reqwest](https://github.com/seanmonstar/reqwest),
therefore you'll be able to use [tokio](https://github.com/tokio-rs/tokio) as
your asynchronous runtime:

```toml
tokio = { version = "0.2", features = ["full"] }
```


## Examples

### Create a client without credentials and get the server time

There are two possible ways to construct a client: with or without credentials,
but be aware that without credentials you will only have access to the public
APIs.

According to the [Kraken documentation](https://www.kraken.com/features/api#general-usage),
all the HTTP responses will contain two fields:

```
error = array of error messages
result = result of API call (may not be present if errors occur)
```

The `ResponseValue` type is the most general type of response that can be
returned by the client and it mirrors the above description, where the `result`,
if present, will be encoded in a
[serde_json::Value](https://docs.serde.rs/serde_json/value/enum.Value.html) enum.

```rust
use akkorokamui::{api, Api, Client, ResponseValue};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::default();

    let time: Api = api::public::time().into();
    let resp: ResponseValue = client.send(time).await?;
    println!("{:?}", resp);

    Ok(())
}
```


### Extract fields from a generic response (server time `unixtime`)

You can extract any field from the JSON response using the `serde_json::Value`
available methods.

```rust
use akkorokamui::{api, Api, Client, ResponseValue};

let client = Client::default();

let time: Api = api::public::time().into();
let resp: ResponseValue = client.send(time).await?;
println!("{:?}", resp);

if let Some(result) = resp.result {
    let time = result.get("unixtime").and_then(|t| t.as_u64());
    println!("Time: {:?}", time);
}
```


### Deserialize a response into a user defined type (server time `unixtime`)

The `ResponseValue` allows not to worry too much on the JSON structure returned
by the Kraken APIs, while not being tied too much on the types defined in this
library. But if you want to exploit the type safety of your user defined types
you can do so by defining your own `Response<T>`.

```rust
use akkorokamui::{api, Api, Client, Response};
use serde::Deserialize;

let client = Client::default();

#[derive(Debug, Deserialize)]
struct Time {
    unixtime: u64,
}

let time: Api = api::public::time().into();
let resp: Response<Time> = client.send(time).await?;
println!("{:?}", resp);

if let Some(result) = resp.result {
    println!("Time: {}", result.unixtime);
}
```


### Specify API parameters (recent trades)

The API builder allows to specify any key-value pair as new parameter via the
method `.with(key, value)`, which can be chained for as many parameters are
needed:

```rust
use akkorokamui::{api, Api, Asset, Client, Response};
use serde::Deserialize;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Deserialize)]
struct Trade {
    price: String,
    volume: String,
    time: f32,
    buy_sell: String,
    market_limit: String,
    miscellaneous: String,
}

#[derive(Debug, Deserialize)]
struct Trades {
    #[serde(flatten)]
    asset_pair_trades: HashMap<String, Vec<Trade>>,
    last: String,
}

let now = SystemTime::now();
let since = now.checked_sub(Duration::from_secs(10)).unwrap();
let since = since.elapsed().unwrap().as_secs();

let asset_pair = Asset::XBT.pair(Asset::EUR);
let time: Api = api::public::trades()
    .with("pair", &asset_pair)
    .with("since", since)
    .into();

let client = Client::default();
let resp: Response<Trades> = client.send(time).await?;
println!("{:?}", resp);

if let Some(result) = resp.result {
    if let Some(trades) = result.asset_pair_trades.get(&asset_pair) {
        for trade in trades {
            println!("price at {}: {}", trade.time, trade.price);
        }
    }
}
```


## Client credentials and private APIs (account balance)

In order to use private APIs you need to own a pair of keys: the public API key
as well as your private key (refer to the
[Kraken support page](https://support.kraken.com/hc/en-us/articles/360000919966-How-to-generate-an-API-key-pair)
to learn how to generate these keys).

The keys must be stored in a single file, where the first line contains the
public API key and the second line contains the private key.

```rust
use akkorokamui::{api, Api, Asset, Client, Credentials, Response};
use std::collections::HashMap;

let keys_path = "kraken.key";
let credentials = Credentials::read(keys_path)?;
let client = Client::new(credentials);

let balance: Api = api::private::balance().into();
let resp: Response<HashMap<String, String>> = client.send(balance).await?;
println!("{:?}", resp);

if let Some(result) = resp.result {
    println!("USD: {:?}", result.get(&Asset::USD.with_prefix()));
}
```
