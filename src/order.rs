use std::ops::Sub;

pub trait Order: Clone {
    type OrderId: PartialEq + std::fmt::Debug;
    type Quantity: Copy + Ord + Default + Sub<Output = Self::Quantity> + std::fmt::Debug;
    type Price: Copy + Ord + std::fmt::Debug;

    fn id(&self) -> Self::OrderId;
    fn quantity(&self) -> Self::Quantity;
    fn set_quantity(&mut self, quantity: Self::Quantity);
    fn price(&self) -> Self::Price;
}
