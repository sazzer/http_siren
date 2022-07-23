#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(
    clippy::module_name_repetitions,
    clippy::unused_async,
    clippy::unused_self
)]

//! This crate provides an implementation of the [Siren Hypermedia specification](https://github.com/kevinswiber/siren).
//!
//! When used with a supported HTTP Server, this will automatically generate the correct JSON response and set the Content-Type header to the correct value of `application/vnd.siren+json`.
//!
//! ## Example Usage
//!
//! The following is a valid handler for Axum that returns a subset of the example from the Siren specification:
//!
//! ```rust
//! # use serde::Serialize;
//!
//! async fn example() -> http_siren::Response<OrderProperties> {
//!     http_siren::Document::new(OrderProperties {
//!         order_number: 42,
//!         item_count: 3,
//!         status: "pending".to_owned(),
//!     })
//!     .with_class("order")
//!     .with_embedded_link(
//!         http_siren::Link::new("http://api.x.io/orders/42/items")
//!             .with_class("items")
//!             .with_class("collection")
//!             .with_rel("http://x.io/rels/order-items"),
//!     ).into()
//! }
//!
//! #[derive(Serialize)]
//! #[serde(rename_all = "camelCase")]
//! struct OrderProperties {
//!     pub order_number: u32,
//!     pub item_count: u32,
//!     pub status: String,
//! }
//! ```
//!
//! # Features
//! HTTP Server support is behind feature flags for the appropriate HTTP Server. As such, you will need to
//! enable the correct feature for the HTTP Server that you are using.
//!
//! Currently supported features are:
//! * `axum` - For the [Axum](https://crates.io/crates/axum) HTTP Server.

mod document;
mod response;
pub mod values;

pub use document::*;
pub use response::*;
