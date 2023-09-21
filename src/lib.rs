#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![warn(clippy::unwrap_used, clippy::expect_used)]
mod fill;
mod orderbook;

pub use crate::orderbook::OrderBook;
pub use fill::Fill;

pub type OrderId = u64;
pub type Quantity = u32;
pub type Price = u8;
pub const MAX_PRICE: Price = Price::MAX;
