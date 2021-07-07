# :octopus: akkorokamui

[![docs.rs](https://docs.rs/akkorokamui/badge.svg)](https://docs.rs/akkorokamui)
[![crates.io](https://img.shields.io/crates/v/akkorokamui.svg)](https://crates.io/crates/akkorokamui)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[akkorokamui](https://en.wikipedia.org/wiki/Akkorokamui) is a HTTP client
written in Rust that allows to query the
[Kraken REST APIs](https://docs.kraken.com/rest).

The main goal of this project is to provide a flexible interface and a safe
implementation.

Check out the [crate documentation](https://docs.rs/akkorokamui) to learn how to
use `akkorokamui`.


### Example: account balance (async version)

```rust
use akkorokamui::{api, Asset, Client, Credentials, Response};
use anyhow::{bail, Result};
use std::collections::HashMap;

type Amount = String;
type Balance<'a> = HashMap<Asset<'a>, Amount>;

#[tokio::main]
async fn main() -> Result<()> {
    let keys_path = "kraken.key";
    let credentials = Credentials::read(keys_path)?;

    let user_agent = "<product>/<product-version>";
    let client = Client::with_credentials(user_agent, credentials)?;

    let api = api::private::balance();
    let resp: Response<Balance> = client.send(api).await?;
    println!("{:?}", resp);

    if let Some(result) = resp.result {
        println!("GBP: {:?}", result.get(&Asset::new("ZGBP")));
    } else {
        bail!("Cannot get balance: {:?}", resp.error);
    }

    Ok(())
}
```
