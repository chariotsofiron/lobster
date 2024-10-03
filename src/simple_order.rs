use crate::order::Order;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct SimpleOrder {
    order_id: u64,
    quantity: u64,
    price: u64,
}

impl SimpleOrder {
    #[must_use]
    #[allow(dead_code)]
    pub const fn new(order_id: u64, quantity: u64, price: u64) -> Self {
        Self {
            order_id,
            quantity,
            price,
        }
    }
}

impl Order for SimpleOrder {
    type OrderId = u64;
    type Quantity = u64;
    type Price = u64;

    fn id(&self) -> Self::OrderId {
        self.order_id
    }

    fn quantity(&self) -> Self::Quantity {
        self.quantity
    }

    fn set_quantity(&mut self, quantity: Self::Quantity) {
        self.quantity = quantity;
    }

    fn price(&self) -> Self::Price {
        self.price
    }
}
