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
}

impl<OrderType> Default for VecBook<OrderType> {
    fn default() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
        }
    }
}

impl<OrderType: Order> OrderBook<OrderType> for VecBook<OrderType> {
    #[expect(clippy::arithmetic_side_effects)]
    fn len(&self) -> usize {
        self.bids.len() + self.asks.len()
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

    // #[allow(refining_impl_trait_reachable)]
    fn add(&mut self, order: OrderType) -> impl Iterator<Item = Fill<OrderType>> {
        if order.is_buy() {
            FillIterator {
                maker_orders: &mut self.asks,
                taker_orders: &mut self.bids,
                taker_order: Some(order),
                taker_is_buy: true,
            }
        } else {
            FillIterator {
                maker_orders: &mut self.bids,
                taker_orders: &mut self.asks,
                taker_order: Some(order),
                taker_is_buy: false,
            }
        }
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
        if quantity == OrderType::Quantity::default() {
            return false;
        }
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

/// An iterator that yields fills for a taker order.
pub struct FillIterator<'book, OrderType: Order> {
    /// Maker orders are sorted by price descending.
    maker_orders: &'book mut Vec<OrderType>,
    /// Taker orders are sorted by price ascending.
    taker_orders: &'book mut Vec<OrderType>,
    /// This is an option to allow us to take it out of the iterator
    taker_order: Option<OrderType>,
    /// `true` if the taker order is a buy order, `false` if it is a sell order.
    taker_is_buy: bool,
}

impl<OrderType: Order> FillIterator<'_, OrderType> {
    /// Put the taker order back in the book if it was not fully matched.
    fn put_taker_order_in_book(&mut self) {
        let Some(order) = self.taker_order.take() else {
            return;
        };

        let index = self
            .taker_orders
            .binary_search_by(|probe| {
                let cmp = if self.taker_is_buy {
                    probe.price().cmp(&order.price())
                } else {
                    order.price().cmp(&probe.price())
                };
                cmp.then(Ordering::Greater)
            })
            .unwrap_or_else(|i| i);

        self.taker_orders.insert(index, order);
    }
}

impl<OrderType: Order> Iterator for FillIterator<'_, OrderType> {
    type Item = Fill<OrderType>;

    fn next(&mut self) -> Option<Self::Item> {
        let taker = self.taker_order.as_mut()?;
        if taker.quantity() == OrderType::Quantity::default() {
            return None;
        }

        // are there any valid orders to match with?
        let Some(order) = self.maker_orders.last_mut() else {
            self.put_taker_order_in_book();
            return None;
        };

        let is_taker_price_worse = if self.taker_is_buy {
            order.price() > taker.price()
        } else {
            order.price() < taker.price()
        };

        if is_taker_price_worse {
            self.put_taker_order_in_book();
            return None;
        }

        // match with resting order
        #[expect(clippy::arithmetic_side_effects)]
        if taker.quantity() >= order.quantity() {
            let fill = Fill::full(order.id(), order.quantity(), order.price());
            taker.set_quantity(taker.quantity() - order.quantity());
            self.maker_orders.pop();
            Some(fill)
        } else {
            let fill = Fill::partial(order.id(), taker.quantity(), order.price());
            order.set_quantity(order.quantity() - taker.quantity());
            taker.set_quantity(OrderType::Quantity::default());
            Some(fill)
        }
    }
}
