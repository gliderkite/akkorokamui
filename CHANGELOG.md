# Changelog
All notable changes to this project will be documented in this file.


## [0.4.0] - 2021-04-25
### Changed
- The `Asset` enum has been replaced by a
    [clone-on-write](https://doc.rust-lang.org/std/borrow/enum.Cow.html) smart
    pointer that allows to store the string representation of the asset, and
    does not require breaking changes if the upstream Kraken assets change.
- The `Error::UnknownAsset` variant has been removed.

### Added
- Add two new private APIs to cancel all orders after a specified timeout, and
    to request a new token used to connect to and authenticate with the Kraken
    Websockets API.
- Add a new `AssetPair` struct to represent base and quote `Asset`s.


## [0.3.0] - 2020-12-06
### Changed
- Update the variants of the `Asset` enum, according to the values returned by
    the `GET public/AssetPairs` Kraken API, which can now be used directly to
    deserialize the API JSON response.
- The dependency `reqwest` will use by default `rustls` instead of `native-tls`
    for easier cross builds. It is still possible to use the system-native TLS
    by specifying the `native-tls` optional Cargo feature.
- Add `Client::new` and `Client::with_credentials` methods to construct HTTP
    clients that can be used to send requests to the Kraken servers, which
    replace the previous methods of the `ClientBuilder` type.

### Added
- `ClientBuilder::build` method that consumes the builder to construct a new
    `Client`, equivalent to the already existing `TryInto<Client>`.
- Derive more common traits for the `Order`, `OrderType` and `Asset` types.
- Add the Kraken Fee Credits (`KFEE`) as new `Asset` and add the `Asset::Unknown`
    variant that will be used when deserializing any unrecognized asset name.
- Add a new public API to get the Kraken system status, as well as a new private
    API to cancel all open orders.


## [0.2.0] - 2020-11-13
### Changed
- The method `Client::send` accepts now two generic parameters provided by the
    caller, where the user can specify the response result type (as it happened
    in the previous library version) and the request type (not bound anymore
    exclusively to the `Api` type).


## [0.1.0] - 2020-11-07
### Added
- Synchronous HTTP `Client` that can be used to send public and private requests
    via the [Kraken REST APIs](https://www.kraken.com/en-gb/features/api),
    allowing the user to define its own response type.
- `ApiBuilder` for all the existing public and private APIs, with methods to set
    any parameter where both key and value implement `fmt::Display`.
- API `Response` wrapper that contains the list of errors, the optional value for
    the user's defined result type and the response status code.
- `ResponseValue` type alias for generic JSON responses with utility methods to
    get the value of specific response fields.
- `Credentials` reader that can be used to load the Kraken public API key and the
    user's private key, which will be used by the HTTP client when constructed
    by the `ClientBuilder`.
- `Asset`s enumeration with utility methods for combining them into asset pairs.
- Crate `Error` type for error reporting.
- `Order` and `OrderType` enumerations to enhance queries type safety for the
    most commonly used parameters.
