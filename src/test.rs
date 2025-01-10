//! Tests for the order book.
#[cfg(test)]
mod tests {
    use crate::{simple_order::SimpleOrder, Fill, OrderBook, VecBook};

    type MyBook = VecBook<SimpleOrder>;
    type MyOrder = SimpleOrder;

    #[test]
    fn partial_fill() {
        let mut book = MyBook::default();

        let o1 = MyOrder::sell(0, 2, 5);
        let o2 = MyOrder::buy(1, 1, 5);

        book.add(o1).for_each(drop);
        let mut fills = book.add(o2);
        assert_eq!(fills.next(), Some(Fill::Partial(o1.with_quantity(1))));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn complete_fill() {
        let mut book = MyBook::default();

        let o1 = MyOrder::sell(0, 2, 5);
        let o2 = MyOrder::buy(1, 2, 5);

        book.add(o1).for_each(drop);
        let mut fills = book.add(o2);

        assert_eq!(fills.next(), Some(Fill::Full(o1)));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn order_with_zero_quantity() {
        let mut book = MyBook::default();
        book.add(MyOrder::sell(0, 2, 5)).for_each(drop);
        let mut fills = book.add(MyOrder::buy(1, 0, 5));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn overfill_match_with_resting() {
        let mut book = MyBook::default();

        let o1 = MyOrder::sell(0, 2, 5);
        let o2 = MyOrder::buy(1, 3, 5);

        book.add(o1).for_each(drop);
        let mut fills = book.add(o2);

        assert_eq!(fills.next(), Some(Fill::Full(o1)));
        assert_eq!(fills.next(), None);

        drop(fills);

        let mut fills = book.add(MyOrder::sell(2, 4, 5));

        assert_eq!(fills.next(), Some(Fill::Full(o2.with_quantity(1))));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn add_order_then_remove_twice() {
        let mut book = MyBook::default();
        let order_id = 1;
        let order = MyOrder::buy(order_id, 1, 2);
        let fills: Vec<_> = book.add(order).collect();
        assert!(fills.is_empty());
        assert_eq!(book.len(), 1);
        assert_eq!(book.remove(order_id), Some(order));
        assert_eq!(book.len(), 0);
        assert_eq!(book.remove(order_id), None);
    }

    #[test]
    fn multiple_fills_with_cancel() {
        let mut book = MyBook::default();

        let o1 = MyOrder::sell(0, 2, 5);
        let o2 = MyOrder::sell(1, 3, 6);
        let o3 = MyOrder::sell(2, 4, 7);

        book.add(o1).for_each(drop);
        book.add(o2).for_each(drop);
        book.add(o3).for_each(drop);
        book.remove(0);
        let mut fills = book.add(MyOrder::buy(3, 6, 6));

        assert_eq!(fills.next(), Some(Fill::Full(o2)));
        assert_eq!(fills.next(), None);

        let o0 = MyOrder::buy(0, 4, 5);
        let o1 = MyOrder::buy(1, 3, 6);
        let o2 = MyOrder::buy(2, 2, 7);

        let mut book = MyBook::default();
        book.add(o0).for_each(drop);
        book.add(o1).for_each(drop);
        book.add(o2).for_each(drop);
        book.remove(0);
        let mut fills = book.add(MyOrder::sell(3, 6, 6));

        assert_eq!(fills.next(), Some(Fill::Full(o2.with_quantity(2))));
        assert_eq!(fills.next(), Some(Fill::Full(o1)));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn fire_for_order_that_was_filled_exactly() {
        let mut book = MyBook::default();
        let o0 = MyOrder::sell(0, 2, 23);
        book.add(o0).for_each(drop);
        let mut fills = book.add(MyOrder::buy(1, 2, 23));
        assert_eq!(fills.next(), Some(Fill::Full(o0)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.add(MyOrder::buy(2, 2, 23));
        assert_eq!(fills.next(), None);

        let mut book = MyBook::default();
        let o0 = MyOrder::buy(0, 2, 23);
        book.add(o0).for_each(drop);
        let mut fills = book.add(MyOrder::sell(1, 2, 23));
        assert_eq!(fills.next(), Some(Fill::Full(o0)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.add(MyOrder::sell(2, 2, 23));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn fire_for_order_that_was_filled_excessively() {
        let mut book = MyBook::default();
        let o0 = MyOrder::sell(0, 1, 23);
        book.add(o0).for_each(drop);
        let mut fills = book.add(MyOrder::buy(1, 2, 23));
        assert_eq!(fills.next(), Some(Fill::Full(o0)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.add(MyOrder::buy(2, 1, 23));
        assert_eq!(fills.next(), None);

        let mut book = MyBook::default();
        let o0 = MyOrder::buy(0, 1, 23);
        book.add(o0).for_each(drop);
        let mut fills = book.add(MyOrder::sell(1, 2, 23));
        assert_eq!(fills.next(), Some(Fill::Full(o0)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.add(MyOrder::sell(2, 1, 23));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn trade_twice_with_resting_order() {
        let mut book = MyBook::default();
        let o0 = MyOrder::sell(0, 2, 23);
        book.add(o0).for_each(drop);
        let mut fills = book.add(MyOrder::buy(1, 1, 23));
        assert_eq!(fills.next(), Some(Fill::Partial(o0.with_quantity(1))));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.add(MyOrder::buy(2, 1, 23));
        assert_eq!(fills.next(), Some(Fill::Full(o0.with_quantity(1))));
        assert_eq!(fills.next(), None);

        let mut book = MyBook::default();
        let o0 = MyOrder::buy(0, 2, 23);
        book.add(o0).for_each(drop);
        let mut fills = book.add(MyOrder::sell(1, 1, 23));
        assert_eq!(fills.next(), Some(Fill::Partial(o0.with_quantity(1))));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.add(MyOrder::sell(2, 1, 23));
        assert_eq!(fills.next(), Some(Fill::Full(o0.with_quantity(1))));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn test_queue_priority() {
        let orders = vec![
            MyOrder::sell(0, 1, 23),
            MyOrder::sell(1, 1, 23),
            MyOrder::sell(2, 1, 23),
        ];

        let mut book = MyBook::from_iter(orders.clone());
        let fills = book.add(MyOrder::buy(3, 3, 23));
        assert!(orders.into_iter().map(Fill::Full).eq(fills), "fills unexpected");

        let orders = vec![
            MyOrder::buy(0, 1, 23),
            MyOrder::buy(1, 1, 23),
            MyOrder::buy(2, 1, 23),
        ];

        let mut book = MyBook::from_iter(orders.clone());
        let fills = book.add(MyOrder::sell(3, 3, 23));
        assert!(orders.into_iter().map(Fill::Full).eq(fills), "fills unexpected");
    }

    #[test]
    fn test_modify_order() {
        let mut book = MyBook::default();
        let o0 = MyOrder::sell(0, 2, 23);
        book.add(o0).for_each(drop);
        assert_eq!(book.modify(0, 0), false);
        assert_eq!(book.modify(0, 1), true);
        assert_eq!(book.modify(0, 1), false);
    }
}
