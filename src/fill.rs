use crate::{OrderId, Price, Quantity};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Fill {
    /// The order id of the maker.
    pub id: OrderId,
    pub quantity: Quantity,
    pub price: Price,
    pub done: bool,
}

impl Fill {
    #[must_use]
    pub const fn new(id: OrderId, quantity: Quantity, price: Price, done: bool) -> Self {
        Self {
            id,
            quantity,
            price,
            done,
        }
    }
}
