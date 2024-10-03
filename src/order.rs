use std::ops::Sub;

pub trait Order: Clone {
    type OrderId: Copy + PartialEq;
    type Quantity: Default + Ord + PartialOrd + Sub<Output = Self::Quantity> + Copy;
    type Price: Ord + Copy + PartialEq;

    fn id(&self) -> Self::OrderId;
    fn quantity(&self) -> Self::Quantity;
    fn set_quantity(&mut self, quantity: Self::Quantity);
    fn price(&self) -> Self::Price;
}
