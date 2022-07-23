# Problem Details

[![Build status](https://github.com/sazzer/http_siren/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sazzer/http_siren/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/http_siren)](https://crates.io/crates/http_siren)
[![Documentation](https://docs.rs/http_siren/badge.svg)](https://docs.rs/http_siren)

This crate provides an implementation of the [Siren Hypermedia specification](https://github.com/kevinswiber/siren).

## Example Usage

The following is a valid handler for Axum that returns a subset of the example from the Siren specification:

```rust
async fn example() -> http_siren::Response<OrderProperties> {
    http_siren::Document::new(OrderProperties {
        order_number: 42,
        item_count: 3,
        status: "pending".to_owned(),
    })
    .with_class("order")
    .with_embedded_link(
        http_siren::Link::new("http://api.x.io/orders/42/items")
            .with_class("items")
            .with_class("collection")
            .with_rel("http://x.io/rels/order-items"),
    ).into()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OrderProperties {
    pub order_number: u32,
    pub item_count: u32,
    pub status: String,
}
```

When used with a supported HTTP Server, this will automatically generate the correct JSON response and set the Content-Type header to the correct value of `application/vnd.siren+json`.

## Supported HTTP Servers

Currently this is only supported with the following HTTP Servers:

- [Axum](https://crates.io/crates/axum)

Examples of use with the different HTTP Servers can be found in the [examples](https://github.com/sazzer/http_siren/tree/main/examples) directory.

# Features

HTTP Server support is behind feature flags for the appropriate HTTP Server. As such, you will need to enable the correct feature for the HTTP Server that you are using.

Currently supported features are:

- `axum` - For the [Axum](https://crates.io/crates/axum) HTTP Server.

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Minimum supported Rust version

The MSRV for `http_siren` is 1.60.0. However, the HTTP Servers that are used with it might need a higher version.

## License

This project is licensed under the MIT license.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `http_siren` by you, shall be licensed as MIT, without any additional terms or conditions.
