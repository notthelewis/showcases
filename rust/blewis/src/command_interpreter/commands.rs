use crate::{data_type::DataType, store::Store};

pub enum Command<'a> {
    Get(Get<'a>),
    Set(Set<'a>),
}

pub struct Get<'a> {
    store: &'a mut Store,
}
pub struct Set<'a> {
    store: &'a mut Store,
}

impl<'a> Get<'a> {
    pub fn run(
        &'a self,
        key: &DataType,
    ) -> Option<dashmap::mapref::one::Ref<'a, DataType, DataType>> {
        self.store.get(key)
    }
}

impl<'a> Set<'a> {
    // When the set command is ran, if a value with the key already exists, it replaces it and returns
    // the old value
    pub fn run(&'a self, key: &DataType, value: &DataType) -> Option<DataType> {
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
            store: &mut store.clone(),
        };
        let set = Set {
            store: &mut store.clone(),
        };

        let key = Int::new_u8(0x00);
        let val = Int::new_u8(0x01);

        let set_res = set.run(&key, &val);
        assert!(set_res.is_none());
        assert_eq!(*get.run(&key).unwrap(), val);
    }
}
