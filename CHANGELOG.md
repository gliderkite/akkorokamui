# Changelog
All notable changes to this project will be documented in this file.


## [Unreleased]
### Changed
- Update the variants of the `Asset` enum, according to the values returned by
    the `GET public/AssetPairs` Kraken API, which can now be used directly to
    deserialize the API JSON response.
- The dependency `reqwest` will use by default `rustls` instead of `native-tls`
    for easier cross builds. It is still possible to use the system-native TLS
    by specifying the `native-tls` optional Cargo feature.

### Added
- `ClientBuilder::build` method that consumes the builder to construct a new
    `Client`, equivalent to the already existing `TryInto<Client>`.
- Derive more common traits for the `Order`, `OrderType` and `Asset` types.
- Add the Kraken Fee Credits (`KFEE`) as new `Asset` and add the `Asset::Unknown`
    variant that will be used when deserializing any unrecognized asset name.
- Add a new public API to get the Kraken system status, as well as a new private
    API to cancel all open orders.

## [0.2.0] - 2020-13-07
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
