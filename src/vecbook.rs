use std::cmp::Reverse;

use crate::fill::Fill;
use crate::order::Order;
use crate::Orderbook;

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

impl<OrderType: Order> Orderbook<OrderType> for VecBook<OrderType> {
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

    fn buy(&mut self, mut order: OrderType) -> impl Iterator<Item = Fill<OrderType>> {
        let asks = &mut self.asks;
        let mut quantity = order.quantity();
        let price = order.price();
        let fills = std::iter::from_fn(move || match_orders(asks, &mut quantity, price));

        if quantity > OrderType::Quantity::default() {
            order.set_quantity(quantity);
            self.bids.insert(0, order);
            self.bids.sort_by_key(OrderType::price);
        }

        fills
    }

    fn sell(&mut self, mut order: OrderType) -> impl Iterator<Item = Fill<OrderType>> {
        let bids = &mut self.bids;
        let mut quantity = order.quantity();
        let price = order.price();
        let fills = std::iter::from_fn(move || match_orders(bids, &mut quantity, price));

        if quantity > OrderType::Quantity::default() {
            order.set_quantity(quantity);
            self.asks.insert(0, order);
            self.asks.sort_by_key(|order| Reverse(order.price()));
        }

        fills
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

fn match_orders<OrderType: Order>(
    maker_orders: &mut Vec<OrderType>,
    quantity: &mut OrderType::Quantity,
    price: OrderType::Price,
) -> Option<Fill<OrderType>> {
    if quantity == &OrderType::Quantity::default() {
        return None;
    }
    let order = maker_orders.last_mut()?;
    if order.price() > price {
        return None;
    }

    #[allow(clippy::arithmetic_side_effects)]
    if *quantity >= order.quantity() {
        let fill = Fill::new(order.id(), order.quantity(), order.price(), true);
        *quantity = *quantity - order.quantity();
        maker_orders.pop();
        Some(fill)
    } else {
        let fill = Fill::new(order.id(), *quantity, order.price(), false);
        order.set_quantity(order.quantity() - *quantity);
        *quantity = OrderType::Quantity::default();
        Some(fill)
    }
}

#[cfg(test)]
mod tests {
    use super::{Fill, Orderbook};
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
