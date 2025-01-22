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

    /// Clears the order book, removing all orders.
    fn clear(&mut self);

    /// Returns an iterator over all orders in the order book.
    fn iter<'book>(&'book self) -> impl Iterator<Item = &'book OrderType>
    where
        OrderType: 'book,
    {
        self.bids().chain(self.asks())
    }

    /// Returns a reference to an order by id.
    fn get(&self, order_id: OrderType::OrderId) -> Option<&OrderType> {
        self.iter().find(|order| order.id() == order_id)
    }

    /// Returns `true` if the order book contains an order with the given id.
    fn contains(&self, order_id: OrderType::OrderId) -> bool {
        self.get(order_id).is_some()
    }

    /// Returns an iterator over the bids from best to worst.
    fn bids<'book>(&'book self) -> impl Iterator<Item = &'book OrderType>
    where
        OrderType: 'book;

    /// Returns an iterator over the asks from best to worst.
    fn asks<'book>(&'book self) -> impl Iterator<Item = &'book OrderType>
    where
        OrderType: 'book;

    /// Returns the bid with the highest price.
    #[must_use]
    fn best_bid(&self) -> Option<&OrderType> {
        self.bids().next()
    }

    /// Returns the ask with the lowest price.
    #[must_use]
    fn best_ask(&self) -> Option<&OrderType> {
        self.asks().next()
    }

    /// Adds a new order to the order book and returns a slice of fills.
    /// Order id should be unique for each new order.
    /// Orders with zero quantity are not added.
    fn add(&mut self, order: OrderType) -> &[Fill<OrderType>];

    /// Removes an order by id.
    fn remove(&mut self, order_id: OrderType::OrderId) -> Option<OrderType>;

    /// Modifies the quantity of an order by order id.
    /// Quantity must be non-zero and less than the current order quantity.
    /// Returns `true` if the order's quantity was modified.
    fn modify(&mut self, order_id: OrderType::OrderId, quantity: OrderType::Quantity) -> bool;
}
