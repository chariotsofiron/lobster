#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod fill;
mod orderbook;

pub use crate::orderbook::OrderBook;
pub use fill::Fill;

pub type Quantity = u32;
pub type Price = u16;
pub const MAX_PRICE: Price = 100;
