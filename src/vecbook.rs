use std::cmp::Reverse;

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
            quantity: order.quantity(),
            price: order.price(),
            taker_order: Some(order),
            taker_is_buy: true,
        }
    }

    #[allow(refining_impl_trait_reachable)]
    fn sell(&mut self, order: OrderType) -> FillIterator<OrderType> {
        FillIterator {
            maker_orders: &mut self.bids,
            taker_orders: &mut self.asks,
            quantity: order.quantity(),
            price: order.price(),
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
    quantity: OrderType::Quantity,
    price: OrderType::Price,
    // This is an option to allow us to take it out of the iterator
    taker_order: Option<OrderType>,
    taker_is_buy: bool,
}

impl<'a, OrderType: Order> FillIterator<'a, OrderType> {
    fn put_taker_order_in_book(&mut self) {
        // safety: taker_order is some until the last iteration
        // this is the only place where we take it
        #[allow(clippy::unwrap_used)]
        let mut order = self.taker_order.take().unwrap();
        order.set_quantity(self.quantity);
        self.taker_orders.insert(0, order);

        if self.taker_is_buy {
            self.taker_orders.sort_by_key(OrderType::price);
        } else {
            self.taker_orders
                .sort_by_key(|order| Reverse(order.price()));
        }
    }
}

impl<'a, OrderType: Order> Iterator for FillIterator<'a, OrderType> {
    type Item = Fill<OrderType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.quantity == OrderType::Quantity::default() {
            return None;
        }

        // are there any valid orders to match with?
        let Some(order) = self.maker_orders.last_mut() else {
            self.put_taker_order_in_book();
            return None;
        };

        let is_taker_price_worse = if self.taker_is_buy {
            order.price() > self.price
        } else {
            order.price() < self.price
        };

        if is_taker_price_worse {
            self.put_taker_order_in_book();
            return None;
        }

        // match with resting order
        #[allow(clippy::arithmetic_side_effects)]
        if self.quantity >= order.quantity() {
            let fill = Fill::new(order.id(), order.quantity(), order.price(), true);
            self.quantity = self.quantity - order.quantity();
            self.maker_orders.pop();
            Some(fill)
        } else {
            let fill = Fill::new(order.id(), self.quantity, order.price(), false);
            order.set_quantity(order.quantity() - self.quantity);
            self.quantity = OrderType::Quantity::default();
            Some(fill)
        }
    }
}
