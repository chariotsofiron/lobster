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

    fn buy(&mut self, order: OrderType) -> impl Iterator<Item = Fill<OrderType>> {
        FillIterator {
            maker_orders: &mut self.asks,
            taker_orders: &mut self.bids,
            quantity: order.quantity(),
            price: order.price(),
            taker_order: Some(order),
            taker_is_buy: true,
        }
    }

    fn sell(&mut self, order: OrderType) -> impl Iterator<Item = Fill<OrderType>> {
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
    taker_order: Option<OrderType>,
    taker_is_buy: bool,
}

impl<'a, OrderType: Order> Iterator for FillIterator<'a, OrderType> {
    type Item = Fill<OrderType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.quantity == OrderType::Quantity::default() {
            return None;
        }

        // are there any valid orders to match with?
        let order = self.maker_orders.last_mut();

        if self.taker_is_buy {
            let nothing_left = order
                .as_ref()
                .map_or(true, |order| order.price() > self.price);
            if nothing_left {
                let mut taker_order = self.taker_order.take()?;
                taker_order.set_quantity(self.quantity);
                self.taker_orders.insert(0, taker_order);
                self.taker_orders.sort_by_key(OrderType::price);
                return None;
            }
        } else {
            let nothing_left = order
                .as_ref()
                .map_or(true, |order| order.price() < self.price);
            if nothing_left {
                let mut taker_order = self.taker_order.take()?;
                taker_order.set_quantity(self.quantity);
                self.taker_orders.insert(0, taker_order);
                self.taker_orders
                    .sort_by_key(|order| Reverse(order.price()));
                return None;
            }
        }
        let order = order?;

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

#[cfg(test)]
mod tests {
    use super::{Fill, OrderBook};
    use crate::simple_order::SimpleOrder;

    type MyBook = super::VecBook<SimpleOrder>;
    type MyOrder = SimpleOrder;

    #[test]
    fn partial_fill() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 2, 5)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(1, 1, 5));
        assert_eq!(fills.next(), Some(Fill::new(0, 1, 5, false)));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn complete_fill() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 2, 5)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(1, 2, 5));
        assert_eq!(fills.next(), Some(Fill::new(0, 2, 5, true)));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn order_with_zero_quantity() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 2, 5)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(1, 0, 5));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn overfill_match_with_resting() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 2, 5)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(1, 3, 5));
        assert_eq!(fills.next(), Some(Fill::new(0, 2, 5, true)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.sell(MyOrder::new(2, 4, 5));
        assert_eq!(fills.next(), Some(Fill::new(1, 1, 5, true)));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn add_order_then_remove_twice() {
        let mut book = MyBook::default();
        let order_id = 1;
        let order = MyOrder::new(order_id, 1, 2);
        let fills: Vec<_> = book.buy(order).collect();
        assert!(fills.is_empty());
        assert_eq!(book.len(), 1);
        assert_eq!(book.remove(order_id), Some(order));
        assert_eq!(book.len(), 0);
        assert_eq!(book.remove(order_id), None);
    }

    #[test]
    fn multiple_fills_with_cancel() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 2, 5)).for_each(drop);
        book.sell(MyOrder::new(1, 3, 6)).for_each(drop);
        book.sell(MyOrder::new(2, 4, 7)).for_each(drop);
        book.remove(0);
        let mut fills = book.buy(MyOrder::new(3, 6, 6));
        assert_eq!(fills.next(), Some(Fill::new(1, 3, 6, true)));
        assert_eq!(fills.next(), None);

        let mut book = MyBook::default();
        book.buy(MyOrder::new(0, 4, 5)).for_each(drop);
        book.buy(MyOrder::new(1, 3, 6)).for_each(drop);
        book.buy(MyOrder::new(2, 2, 7)).for_each(drop);
        book.remove(0);
        let mut fills = book.sell(MyOrder::new(3, 6, 6));
        assert_eq!(fills.next(), Some(Fill::new(2, 2, 7, true)));
        assert_eq!(fills.next(), Some(Fill::new(1, 3, 6, true)));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn fire_for_order_that_was_filled_exactly() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 2, 23)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(1, 2, 23));
        assert_eq!(fills.next(), Some(Fill::new(0, 2, 23, true)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.buy(MyOrder::new(2, 2, 23));
        assert_eq!(fills.next(), None);

        let mut book = MyBook::default();
        book.buy(MyOrder::new(0, 2, 23)).for_each(drop);
        let mut fills = book.sell(MyOrder::new(1, 2, 23));
        assert_eq!(fills.next(), Some(Fill::new(0, 2, 23, true)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.sell(MyOrder::new(2, 2, 23));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn fire_for_order_that_was_filled_excessively() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 1, 23)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(1, 2, 23));
        assert_eq!(fills.next(), Some(Fill::new(0, 1, 23, true)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.buy(MyOrder::new(2, 1, 23));
        assert_eq!(fills.next(), None);

        let mut book = MyBook::default();
        book.buy(MyOrder::new(0, 1, 23)).for_each(drop);
        let mut fills = book.sell(MyOrder::new(1, 2, 23));
        assert_eq!(fills.next(), Some(Fill::new(0, 1, 23, true)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.sell(MyOrder::new(2, 1, 23));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn trade_twice_with_resting_order() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 2, 23)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(1, 1, 23));
        assert_eq!(fills.next(), Some(Fill::new(0, 1, 23, false)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.buy(MyOrder::new(2, 1, 23));
        assert_eq!(fills.next(), Some(Fill::new(0, 1, 23, true)));
        assert_eq!(fills.next(), None);

        let mut book = MyBook::default();
        book.buy(MyOrder::new(0, 2, 23)).for_each(drop);
        let mut fills = book.sell(MyOrder::new(1, 1, 23));
        assert_eq!(fills.next(), Some(Fill::new(0, 1, 23, false)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.sell(MyOrder::new(2, 1, 23));
        assert_eq!(fills.next(), Some(Fill::new(0, 1, 23, true)));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn test_queue_priority() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 1, 23)).for_each(drop);
        book.sell(MyOrder::new(1, 1, 23)).for_each(drop);
        book.sell(MyOrder::new(2, 1, 23)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(3, 3, 23));

        assert_eq!(fills.next(), Some(Fill::new(0, 1, 23, true)));
        assert_eq!(fills.next(), Some(Fill::new(1, 1, 23, true)));
        assert_eq!(fills.next(), Some(Fill::new(2, 1, 23, true)));
    }
}
