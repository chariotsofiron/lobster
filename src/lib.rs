//! Lobster order book library.
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    // clippy::as_conversions,
    // clippy::arithmetic_side_effects,
    // clippy::expect_used,
    // clippy::float_arithmetic,
    // clippy::integer_division,
    // clippy::unwrap_used,
    clippy::restriction,
)]
#![allow(
    clippy::missing_trait_methods,
    clippy::question_mark_used,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return,
    clippy::pub_use,
    clippy::blanket_clippy_restriction_lints,
    clippy::exhaustive_structs,
    clippy::allow_attributes_without_reason
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
