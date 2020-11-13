# akkorokamui

[![docs.rs](https://docs.rs/akkorokamui/badge.svg)](https://docs.rs/akkorokamui)
[![crates.io](https://img.shields.io/crates/v/akkorokamui.svg)](https://crates.io/crates/akkorokamui)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[akkorokamui](https://en.wikipedia.org/wiki/Akkorokamui) :octopus: is a HTTP
client written in Rust that allows to query the [Kraken](https://www.kraken.com/)
REST APIs.

The main goal of this project is to provide a flexible interface and a safe
implementation.

Check out the [crate documentation](https://docs.rs/akkorokamui) to learn how to
use `akkorokamui`.


### Example: account balance

```rust
use akkorokamui::{api, client, Asset, Client, Credentials, Response};
use anyhow::{bail, Result};
use std::{collections::HashMap, convert::TryInto};

type Currency = String;
type Amount = String;
type Balance = HashMap<Currency, Amount>;

fn main() -> Result<()> {
    let keys_path = "kraken.key";
    let credentials = Credentials::read(keys_path)?;
    let user_agent = "<product>/<product-version>";

    let client: Client = client::with_user_agent(user_agent)
        .with_credentials(credentials)
        .try_into()?;

    let api = api::private::balance();
    let resp: Response<Balance> = client.send(api)?;
    println!("{:?}", resp);

    if let Some(result) = resp.result {
        println!("USD: {:?}", result.get(&Asset::USD.with_prefix()));
    } else {
        bail!("Cannot get balance: {:?}", resp.error);
    }

    Ok(())
}
```
