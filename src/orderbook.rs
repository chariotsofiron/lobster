use std::collections::{HashMap, VecDeque};

use crate::{fill::Fill, Price, Quantity, MAX_PRICE};

pub struct OrderBook {
    order_qty: HashMap<usize, Quantity>,
    levels: [VecDeque<usize>; MAX_PRICE as usize + 1],
    best_bid: Price,
    best_ask: Price,
}

impl Default for OrderBook {
    fn default() -> Self {
        Self {
            order_qty: HashMap::new(),
            levels: std::array::from_fn(|_| VecDeque::new()),
            best_bid: 0,
            best_ask: MAX_PRICE,
        }
    }
}

impl OrderBook {
    pub fn buy(&mut self, id: usize, mut quantity: Quantity, price: Price) -> Vec<Fill> {
        let mut fills = Vec::new();
        fills
    }

    pub fn sell(&mut self, id: usize, mut quantity: Quantity, price: Price) -> Vec<Fill> {
        let mut fills = Vec::new();
        fills
    }

    pub fn remove(&mut self, id: usize) -> bool {
        true
    }
}
