#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(
    clippy::module_name_repetitions,
    clippy::unused_async,
    clippy::unused_self
)]

mod document;
mod response;
pub mod values;

pub use document::*;
pub use response::*;
