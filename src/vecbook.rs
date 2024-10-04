use std::cmp::Ordering;

use crate::fill::Fill;
use crate::order::Order;
use crate::OrderBook;

pub struct VecBook<OrderType> {
    /// Bids, sorted by price ascending
    /// Best bid is at the end and is matched with first
    bids: Vec<OrderType>,
    /// Asks, sorted by price descending
    asks: Vec<OrderType>,
}

impl<OrderType> std::default::Default for VecBook<OrderType> {
    fn default() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
        }
    }
}

impl<OrderType: Order> OrderBook<OrderType> for VecBook<OrderType> {
    #[allow(clippy::arithmetic_side_effects)]
    fn len(&self) -> usize {
        self.bids.len() + self.asks.len()
    }

    fn bids<'a>(&'a self) -> impl Iterator<Item = &'a OrderType>
    where
        OrderType: 'a,
    {
        self.bids.iter().rev()
    }

    fn asks<'a>(&'a self) -> impl Iterator<Item = &'a OrderType>
    where
        OrderType: 'a,
    {
        self.asks.iter().rev()
    }

    #[allow(refining_impl_trait_reachable)]
    fn buy(&mut self, order: OrderType) -> FillIterator<OrderType> {
        FillIterator {
            maker_orders: &mut self.asks,
            taker_orders: &mut self.bids,
            taker_order: Some(order),
            taker_is_buy: true,
        }
    }

    #[allow(refining_impl_trait_reachable)]
    fn sell(&mut self, order: OrderType) -> FillIterator<OrderType> {
        FillIterator {
            maker_orders: &mut self.bids,
            taker_orders: &mut self.asks,
            taker_order: Some(order),
            taker_is_buy: false,
        }
    }

    fn remove(&mut self, order_id: <OrderType as Order>::OrderId) -> Option<OrderType> {
        if let Some(i) = self.bids.iter().position(|order| order.id() == order_id) {
            return Some(self.bids.remove(i));
        }
        if let Some(i) = self.asks.iter().position(|order| order.id() == order_id) {
            return Some(self.asks.remove(i));
        }
        None
    }
}

pub struct FillIterator<'a, OrderType: Order> {
    maker_orders: &'a mut Vec<OrderType>,
    taker_orders: &'a mut Vec<OrderType>,
    // This is an option to allow us to take it out of the iterator
    taker_order: Option<OrderType>,
    taker_is_buy: bool,
}

impl<'a, OrderType: Order> FillIterator<'a, OrderType> {
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

impl<'a, OrderType: Order> Iterator for FillIterator<'a, OrderType> {
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
        #[allow(clippy::arithmetic_side_effects)]
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
