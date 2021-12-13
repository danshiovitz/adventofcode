use std::collections::HashMap;

pub type FlagSet = i32;
pub type Trid = usize;

pub struct FlagManager {
    names: HashMap<String, Trid>,
}

impl FlagManager {
    pub fn from<'a, I: Iterator<Item = &'a str>>(names: I) -> FlagManager where I: IntoIterator<Item = &'a str> {
        let name_map = names.enumerate().map(|(idx, n)| (n.to_owned(), idx)).collect();
        return FlagManager { names: name_map };
    }

    pub fn init(&self) -> FlagSet {
        let ret: FlagSet = 0;
        return ret;
    }

    pub fn set_name(&self, val: &mut FlagSet, name: &str) {
        let id = *self.names.get(name).unwrap();
        return self.set(val, id);
    }

    pub fn set(&self, val: &mut FlagSet, id: Trid) {
        let bf = 1 << id;
        *val |= bf;
    }

    pub fn get_name(&self, val: &FlagSet, name: &str) -> bool {
        let id = *self.names.get(name).unwrap();
        return self.get(val, id);
    }

    pub fn get(&self, val: &FlagSet, id: Trid) -> bool {
        let bf = 1 << id;
        return *val & bf != 0;
    }

    pub fn translate(&self, name: &str) -> Trid {
        return *self.names.get(name).unwrap();
    }

    pub fn translate_back(&self, idx: Trid) -> String {
        return self.names.iter().filter_map(|(k, v)| if idx == *v { Some(k.clone()) } else { None }).next().unwrap();
    }
}
