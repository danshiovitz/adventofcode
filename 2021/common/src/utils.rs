use std::collections::HashMap;
use std::hash::Hash;
use std::ops::AddAssign;

pub fn inc_counter<T, U>(counter: &mut HashMap<T, U>, key: T, amount: U) where T: Eq + Hash, U: AddAssign {
    if let Some(val) = counter.get_mut(&key) {
        *val += amount;
    } else {
        counter.insert(key, amount);
    }
}
