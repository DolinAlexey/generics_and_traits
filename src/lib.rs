pub enum Item {
    First,
    Second,
    Third,
}
impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
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
    // Методы sum и is_default реализовать только один раз - в самом трейте.
    fn is_default(&self) -> bool {
        self.0 == 0 && self.1 == 0.0 && self.2 == 0.0
    }
    fn sum(&self) -> f64 {
        self.0 as f64 + self.1 as f64 + self.2
    }
}

pub struct Array([f64; 3]);

// Реализовать трейт на обоих типах.
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

// Написать обобщённую тестовую логику, которая будет работать для обоих типов.
// Здесь вопрос - как можно реализовать fn test_tuple() и fn test_array() в одной функции?
#[cfg(test)]
mod tests {
    //use super::*;
    use crate::Array;
    use crate::Item;
    use crate::SetOfThreeNumbers;
    use crate::Tuple;

    fn test_set_of_three_numbers<T: SetOfThreeNumbers>(set: T) {
        assert_eq!(set.is_default(), true);
        assert_eq!(set.sum(), 0.0);
        assert_eq!(set.get_item(Item::First), 0.0);
        assert_eq!(set.get_item(Item::Second), 0.0);
    }

    #[test]
    fn test_set_of_three_numbers_for_tuple_and_array() {
        let tuple = Tuple::default_values();
        test_set_of_three_numbers(tuple);

        let array = Array::default_values();
        test_set_of_three_numbers(array);
    }
}
