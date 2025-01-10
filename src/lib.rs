//! Lobster order book library.
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::restriction
)]
#![allow(
    clippy::allow_attributes_without_reason,
    clippy::blanket_clippy_restriction_lints,
    clippy::exhaustive_enums,
    clippy::implicit_return,
    clippy::missing_inline_in_public_items,
    clippy::missing_trait_methods,
    clippy::pub_use,
    clippy::question_mark_used,
    clippy::ref_patterns
)]
mod fill;
mod order;
mod orderbook;
mod simple_order;
mod test;
mod vecbook;

pub use fill::Fill;
pub use order::Order;
pub use orderbook::OrderBook;
pub use simple_order::SimpleOrder;
pub use vecbook::VecBook;
