mod fill;
mod orderbook;

pub use fill::Fill;
pub use orderbook::OrderBook;

pub type Quantity = u32;
pub type Price = u16;
pub const MAX_PRICE: Price = 100;
