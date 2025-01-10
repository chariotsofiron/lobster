//! Order book trait.
use crate::{Fill, Order};

pub trait OrderBook<OrderType: Order>: Default + FromIterator<OrderType> {
    /// Returns the number of open orders in the order book.
    #[must_use]
    fn len(&self) -> usize;

    /// Returns `true` if the order book contains no open orders.
    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the bids from best to worst.
    fn bids<'book>(&'book self) -> impl Iterator<Item = &'book OrderType>
    where
        OrderType: 'book;

    /// Returns an iterator over the asks from best to worst.
    fn asks<'book>(&'book self) -> impl Iterator<Item = &'book OrderType>
    where
        OrderType: 'book;

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

    /// Adds a new order to the order book and returns an iterator of fills.
    fn add(&mut self, order: OrderType) -> impl Iterator<Item = Fill<OrderType>>;

    /// Removes an order by id.
    fn remove(&mut self, order_id: OrderType::OrderId) -> Option<OrderType>;

    /// Modifies the quantity of an order by order id.
    /// Quantity must be non-zero and less than the current order quantity.
    /// Returns `true` if the order's quantity was modified.
    fn modify(&mut self, order_id: OrderType::OrderId, quantity: OrderType::Quantity) -> bool;
}
