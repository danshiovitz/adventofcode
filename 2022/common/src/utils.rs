use std::collections::HashMap;
use std::hash::Hash;
use std::ops::AddAssign;

pub fn inc_counter<T, U>(counter: &mut HashMap<T, U>, key: T, amount: U)
where
    T: Eq + Hash,
    U: AddAssign,
{
    if let Some(val) = counter.get_mut(&key) {
        *val += amount;
    } else {
        counter.insert(key, amount);
    }
}

pub fn mod_positive(value: i32, range: i32) -> i32 {
    let mut v = value;
    while v < 0 {
        v += range;
    }
    return v % range;
}

pub fn mod_one_through_range(value: i32, range: i32) -> i32 {
    return mod_positive(value - 1, range) + 1;
}
