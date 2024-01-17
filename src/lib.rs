use std::ops::Index;
use std::slice::SliceIndex;
pub enum Item {
    First,
    Second,
    Third,
}

// Перенести все общие операции в трейт.
pub trait SetOfThreeNumbers {
    fn default_values() -> Self;
    fn get_item(&self, item: Item) -> f64;
    fn set_item(&mut self, item: Item, value: f64);
    fn is_default(&self) -> bool;
    fn sum(&self) -> f64;
}

pub struct Tuple(u32, f32, f64);

impl SetOfThreeNumbers for Tuple {
    fn default_values() -> Self {
        Self(0, 0.0, 0.0)
    }
    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as _,
            Item::Second => self.1 as _,
            Item::Third => self.2,
        }
    }
    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as _,
            Item::Second => self.1 = value as _,
            Item::Third => self.2 = value,
        };
    }
    fn is_default(&self) -> bool {
        self.0 == 0 && self.1 == 0.0 && self.2 == 0.0
    }

    fn sum(&self) -> f64 {
        self.0 as f64 + self.1 as f64 + self.2
    }
}

pub struct Array([f64; 3]);

impl SetOfThreeNumbers for Array {
    fn default_values() -> Self {
        Array([0.0; 3])
    }

    fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }

    fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value
    }

    fn is_default(&self) -> bool {
        self.0.iter().all(|&value| value == 0.0)
    }

    fn sum(&self) -> f64 {
        self.0.iter().sum()
    }
}
