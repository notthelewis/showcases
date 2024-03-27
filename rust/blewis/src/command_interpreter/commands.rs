use dashmap::mapref::one;

use crate::{data_type::DataType, store::Store};

pub enum Command {
    Get(Get),
    Set(Set),
}

pub struct Get {
    store: Store,
}
pub struct Set {
    store: Store,
}

impl Get {
    pub fn run(&self, k: &DataType) -> Option<one::Ref<DataType, DataType>> {
        self.store.get(k)
    }
}

impl Set {
    // When the set command is ran, if a value with the key already exists, it replaces it and returns
    // the old value
    pub fn run(&self, key: &DataType, value: &DataType) -> Option<DataType> {
        self.store.insert(key.to_owned(), value.to_owned())
    }
}

mod test {
    #![allow(unused_imports)]

    use super::{Get, Set};
    use crate::{
        data_type::{DataType, Int},
        store::Store,
    };
    use dashmap::DashMap;
    use std::sync::Arc;

    #[test]
    fn test_set_get() {
        let store: Store = Arc::new(DashMap::new());
        let get = Get {
            store: store.clone(),
        };
        let set = Set {
            store: store.clone(),
        };

        let key = Int::new_u8(0x00);
        let val = Int::new_u8(0x01);

        let set_res = set.run(&key, &val);
        assert!(set_res.is_none());
        assert_eq!(*get.run(&key).unwrap(), val);
    }
}
