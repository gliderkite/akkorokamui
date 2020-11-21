# Changelog
All notable changes to this project will be documented in this file.


## [Unreleased]


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
    user's private key, which will be used by the HTTP client.
- `Asset`s enumeration with utility methods for combining them into asset pairs.
- Crate `Error` type for error reporting.
- `Order` and `OrderType` enumerations to enhance queries type safety for the
    most commonly used parameters.