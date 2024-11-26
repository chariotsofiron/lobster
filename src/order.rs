use std::ops::Sub;

pub trait Order {
    type OrderId: Eq;
    type Quantity: Copy + Ord + Default + Sub<Output = Self::Quantity>;
    type Price: Copy + Ord;

    fn id(&self) -> Self::OrderId;
    fn quantity(&self) -> Self::Quantity;
    fn set_quantity(&mut self, quantity: Self::Quantity);
    fn price(&self) -> Self::Price;
}
