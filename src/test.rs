#[cfg(test)]
mod tests {
    use crate::{simple_order::SimpleOrder, Fill, OrderBook, VecBook};

    type MyBook = VecBook<SimpleOrder>;
    type MyOrder = SimpleOrder;

    #[test]
    fn partial_fill() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 2, 5)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(1, 1, 5));
        assert_eq!(fills.next(), Some(Fill::partial(0, 1, 5)));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn complete_fill() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 2, 5)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(1, 2, 5));
        assert_eq!(fills.next(), Some(Fill::full(0, 2, 5)));
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
        assert_eq!(fills.next(), Some(Fill::full(0, 2, 5)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.sell(MyOrder::new(2, 4, 5));
        assert_eq!(fills.next(), Some(Fill::full(1, 1, 5)));
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
        assert_eq!(fills.next(), Some(Fill::full(1, 3, 6)));
        assert_eq!(fills.next(), None);

        let mut book = MyBook::default();
        book.buy(MyOrder::new(0, 4, 5)).for_each(drop);
        book.buy(MyOrder::new(1, 3, 6)).for_each(drop);
        book.buy(MyOrder::new(2, 2, 7)).for_each(drop);
        book.remove(0);
        let mut fills = book.sell(MyOrder::new(3, 6, 6));
        assert_eq!(fills.next(), Some(Fill::full(2, 2, 7)));
        assert_eq!(fills.next(), Some(Fill::full(1, 3, 6)));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn fire_for_order_that_was_filled_exactly() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 2, 23)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(1, 2, 23));
        assert_eq!(fills.next(), Some(Fill::full(0, 2, 23)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.buy(MyOrder::new(2, 2, 23));
        assert_eq!(fills.next(), None);

        let mut book = MyBook::default();
        book.buy(MyOrder::new(0, 2, 23)).for_each(drop);
        let mut fills = book.sell(MyOrder::new(1, 2, 23));
        assert_eq!(fills.next(), Some(Fill::full(0, 2, 23)));
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
        assert_eq!(fills.next(), Some(Fill::full(0, 1, 23)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.buy(MyOrder::new(2, 1, 23));
        assert_eq!(fills.next(), None);

        let mut book = MyBook::default();
        book.buy(MyOrder::new(0, 1, 23)).for_each(drop);
        let mut fills = book.sell(MyOrder::new(1, 2, 23));
        assert_eq!(fills.next(), Some(Fill::full(0, 1, 23)));
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
        assert_eq!(fills.next(), Some(Fill::partial(0, 1, 23)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.buy(MyOrder::new(2, 1, 23));
        assert_eq!(fills.next(), Some(Fill::full(0, 1, 23)));
        assert_eq!(fills.next(), None);

        let mut book = MyBook::default();
        book.buy(MyOrder::new(0, 2, 23)).for_each(drop);
        let mut fills = book.sell(MyOrder::new(1, 1, 23));
        assert_eq!(fills.next(), Some(Fill::partial(0, 1, 23)));
        assert_eq!(fills.next(), None);
        drop(fills);
        let mut fills = book.sell(MyOrder::new(2, 1, 23));
        assert_eq!(fills.next(), Some(Fill::full(0, 1, 23)));
        assert_eq!(fills.next(), None);
    }

    #[test]
    fn test_queue_priority() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 1, 23)).for_each(drop);
        book.sell(MyOrder::new(1, 1, 23)).for_each(drop);
        book.sell(MyOrder::new(2, 1, 23)).for_each(drop);
        let mut fills = book.buy(MyOrder::new(3, 3, 23));

        assert_eq!(fills.next(), Some(Fill::full(0, 1, 23)));
        assert_eq!(fills.next(), Some(Fill::full(1, 1, 23)));
        assert_eq!(fills.next(), Some(Fill::full(2, 1, 23)));

        let mut book = MyBook::default();
        book.buy(MyOrder::new(0, 1, 23)).for_each(drop);
        book.buy(MyOrder::new(1, 1, 23)).for_each(drop);
        book.buy(MyOrder::new(2, 1, 23)).for_each(drop);
        let mut fills = book.sell(MyOrder::new(3, 3, 23));

        assert_eq!(fills.next(), Some(Fill::full(0, 1, 23)));
        assert_eq!(fills.next(), Some(Fill::full(1, 1, 23)));
        assert_eq!(fills.next(), Some(Fill::full(2, 1, 23)));
    }

    #[test]
    fn test_modify_order() {
        let mut book = MyBook::default();
        book.sell(MyOrder::new(0, 2, 23)).for_each(drop);
        assert_eq!(book.modify(0, 0), false);
        assert_eq!(book.modify(0, 1), true);
        assert_eq!(book.modify(0, 1), false);
    }
}
