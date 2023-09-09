use crate::{Price, Quantity};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Fill {
    pub id: usize,
    pub quantity: Quantity,
    pub price: Price,
    pub done: bool,
}

impl Fill {
    #[must_use]
    pub const fn new(id: usize, quantity: Quantity, price: Price, done: bool) -> Self {
        Self {
            id,
            quantity,
            price,
            done,
        }
    }
}
