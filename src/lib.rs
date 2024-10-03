#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::as_conversions,
    clippy::arithmetic_side_effects,
    clippy::expect_used,
    clippy::float_arithmetic,
    clippy::integer_division,
    clippy::unwrap_used
)]
mod fill;
mod order;
mod simple_order;
mod vecbook;

pub use fill::Fill;
use order::Order;
pub use simple_order::SimpleOrder;

pub trait Orderbook<OrderType: Order>: Default {
    /// Returns the number of open orders in the order book.
    #[must_use]
    fn len(&self) -> usize;

    /// Returns `true` if the book contains no open orders.
    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the bids from best to worst.
    fn bids<'a>(&'a self) -> impl Iterator<Item = &'a OrderType>
    where
        OrderType: 'a;

    /// Returns an iterator over the asks from best to worst.
    fn asks<'a>(&'a self) -> impl Iterator<Item = &'a OrderType>
    where
        OrderType: 'a;

    /// Returns the best bid.
    #[must_use]
    fn best_bid(&self) -> Option<&OrderType> {
        self.bids().next()
    }

    /// Returns the best ask.
    #[must_use]
    fn best_ask(&self) -> Option<&OrderType> {
        self.asks().next()
    }

    /// Adds a buy order to the order book and returns an iterator of fills.
    fn buy(&mut self, order: OrderType) -> impl Iterator<Item = Fill<OrderType>>;

    /// Adds a sell order to the order book and returns an iterator of fills.
    fn sell(&mut self, order: OrderType) -> impl Iterator<Item = Fill<OrderType>>;

    /// Removes an order by id.
    fn remove(&mut self, id: OrderType::OrderId) -> Option<OrderType>;
}
