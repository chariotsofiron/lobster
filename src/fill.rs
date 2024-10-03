use crate::order::Order;

#[derive(Copy, Clone, Debug)]
pub struct Fill<OrderType: Order> {
    pub id: OrderType::OrderId,
    pub quantity: OrderType::Quantity,
    pub price: OrderType::Price,
    pub done: bool,
}

impl<OrderType: Order> PartialEq for Fill<OrderType> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.quantity == other.quantity
            && self.price == other.price
            && self.done == other.done
    }
}

impl<OrderType: Order> Fill<OrderType> {
    pub const fn new(
        id: OrderType::OrderId,
        quantity: OrderType::Quantity,
        price: OrderType::Price,
        done: bool,
    ) -> Self {
        Self {
            id,
            quantity,
            price,
            done,
        }
    }
}
