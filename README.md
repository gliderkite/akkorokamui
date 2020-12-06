# :octopus: akkorokamui

[![docs.rs](https://docs.rs/akkorokamui/badge.svg)](https://docs.rs/akkorokamui)
[![crates.io](https://img.shields.io/crates/v/akkorokamui.svg)](https://crates.io/crates/akkorokamui)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[akkorokamui](https://en.wikipedia.org/wiki/Akkorokamui) is a HTTP client
written in Rust that allows to query the
[Kraken REST APIs](https://www.kraken.com/features/api).

The main goal of this project is to provide a flexible interface and a safe
implementation.

Check out the [crate documentation](https://docs.rs/akkorokamui) to learn how to
use `akkorokamui`.


### Example: account balance

```rust
use akkorokamui::{api, client, Asset, Client, Credentials, Response};
use anyhow::{bail, Result};
use std::collections::HashMap;

type Amount = String;
type Balance = HashMap<Asset, Amount>;

fn main() -> Result<()> {
    let keys_path = "kraken.key";
    let credentials = Credentials::read(keys_path)?;
    let user_agent = "<product>/<product-version>";

    let client: Client = client::with_user_agent(user_agent)
        .with_credentials(credentials)
        .build()?;

    let api = api::private::balance();
    let resp: Response<Balance> = client.send(api)?;
    println!("{:?}", resp);

    if let Some(result) = resp.result {
        println!("GBP: {:?}", result.get(&Asset::ZGBP));
    } else {
        bail!("Cannot get balance: {:?}", resp.error);
    }

    Ok(())
}
```
