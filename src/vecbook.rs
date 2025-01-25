//! A simple order book implementation using vectors.
use core::cmp::Ordering;
use core::default::Default;

use crate::fill::Fill;
use crate::order::Order;
use crate::OrderBook;

/// A simple order book implementation using vectors.
#[derive(Debug, Clone)]
pub struct VecBook<OrderType> {
    /// Bids, sorted by price ascending
    /// Best bid is at the end and is matched with first
    bids: Vec<OrderType>,
    /// Asks, sorted by price descending
    asks: Vec<OrderType>,
    /// Tracks fills
    fills: Vec<Fill<OrderType>>,
}

impl<OrderType> Default for VecBook<OrderType> {
    fn default() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
            fills: Vec::new(),
        }
    }
}

impl<OrderType: Order> OrderBook<OrderType> for VecBook<OrderType> {
    #[expect(clippy::arithmetic_side_effects)]
    fn len(&self) -> usize {
        self.bids.len() + self.asks.len()
    }

    fn clear(&mut self) {
        self.bids.clear();
        self.asks.clear();
    }

    fn bids<'book>(&'book self) -> impl Iterator<Item = &'book OrderType>
    where
        OrderType: 'book,
    {
        self.bids.iter().rev()
    }

    fn asks<'book>(&'book self) -> impl Iterator<Item = &'book OrderType>
    where
        OrderType: 'book,
    {
        self.asks.iter().rev()
    }

    fn add(&mut self, order: OrderType) -> &[Fill<OrderType>] {
        self.fills.clear();
        if order.is_buy() {
            self.match_with_asks(order);
        } else {
            self.match_with_bids(order);
        }
        &self.fills
    }

    fn remove(&mut self, order_id: OrderType::OrderId) -> Option<OrderType> {
        if let Some(i) = self.bids.iter().position(|order| order.id() == order_id) {
            return Some(self.bids.remove(i));
        }
        if let Some(i) = self.asks.iter().position(|order| order.id() == order_id) {
            return Some(self.asks.remove(i));
        }
        None
    }

    fn modify(&mut self, order_id: OrderType::OrderId, quantity: OrderType::Quantity) -> bool {
        if let Some(order) = self.bids.iter_mut().find(|order| order.id() == order_id) {
            if order.quantity() <= quantity {
                return false;
            }
            order.set_quantity(quantity);
            return true;
        }
        if let Some(order) = self.asks.iter_mut().find(|order| order.id() == order_id) {
            if order.quantity() <= quantity {
                return false;
            }
            order.set_quantity(quantity);
            return true;
        }
        false
    }
}

impl<OrderType: Order> VecBook<OrderType> {
    /// Match buy order with asks
    fn match_with_asks(&mut self, mut taker: OrderType) {
        loop {
            let Some(maker) = self.asks.last_mut() else {
                break;
            };
            if taker.price() < maker.price() {
                break;
            }
            match taker.quantity().cmp(&maker.quantity()) {
                Ordering::Equal => {
                    #[expect(clippy::unwrap_used)]
                    let fill = self.asks.pop().unwrap(); // infallible
                    self.fills.push(Fill::Full(fill));
                    return;
                }
                Ordering::Greater => {
                    taker.reduce_quantity(maker.quantity());
                    #[expect(clippy::unwrap_used)]
                    let fill = self.asks.pop().unwrap(); // infallible
                    self.fills.push(Fill::Full(fill));
                }
                Ordering::Less => {
                    maker.reduce_quantity(taker.quantity());
                    let mut fill = maker.clone();
                    fill.set_quantity(taker.quantity());
                    self.fills.push(Fill::Partial(fill));
                    return;
                }
            }
        }

        let index = self
            .bids
            .binary_search_by(|probe| probe.price().cmp(&taker.price()).then(Ordering::Greater))
            .unwrap_or_else(|i| i);

        self.bids.insert(index, taker);
    }

    /// Match sell order with bids
    fn match_with_bids(&mut self, mut taker: OrderType) {
        loop {
            let Some(maker) = self.bids.last_mut() else {
                break;
            };
            if taker.price() > maker.price() {
                break;
            }
            match taker.quantity().cmp(&maker.quantity()) {
                Ordering::Equal => {
                    #[expect(clippy::unwrap_used)]
                    let fill = self.bids.pop().unwrap(); // infallible
                    self.fills.push(Fill::Full(fill));
                    return;
                }
                Ordering::Greater => {
                    taker.reduce_quantity(maker.quantity());
                    #[expect(clippy::unwrap_used)]
                    let fill = self.bids.pop().unwrap(); // infallible
                    self.fills.push(Fill::Full(fill));
                }
                Ordering::Less => {
                    maker.reduce_quantity(taker.quantity());
                    let mut fill = maker.clone();
                    fill.set_quantity(taker.quantity());
                    self.fills.push(Fill::Partial(fill));
                    return;
                }
            }
        }

        let index = self
            .bids
            .binary_search_by(|probe| taker.price().cmp(&probe.price()).then(Ordering::Greater))
            .unwrap_or_else(|i| i);

        self.asks.insert(index, taker);
    }
}

impl<OrderType: Order> FromIterator<OrderType> for VecBook<OrderType> {
    fn from_iter<I: IntoIterator<Item = OrderType>>(iter: I) -> Self {
        let mut book = Self::default();
        for order in iter {
            assert!(book.add(order).is_empty(), "unexpected fill");
        }
        book
    }
}
