use crate::{data_type::DataType, errors::GETSET_KEY_NO_EXIST};
use dashmap::DashMap;
use std::sync::Arc;

/// Store is the concurrent hashmap that is the core of `Blewis`
pub struct Store(Arc<DashMap<DataType, DataType>>);

impl Store {
    pub fn new() -> Self {
        Store(Arc::new(DashMap::new()))
    }

    /// Retrieves a value from the store
    pub fn get(&self, key: &DataType) -> Option<DataType> {
        let res = self.0.get(key);
        match res {
            Some(v) => Some(v.to_owned()),
            None => None
        }
    }

    /// Simillar to set but instead of returning an Option<DataType>, it returns an error code if
    /// a previous value was not set
    pub fn get_set(&self, key: &DataType, new_val: &DataType) -> DataType {
        let existing_val = self.0.insert(key.to_owned(), new_val.to_owned());

        if existing_val.is_none() {
            return GETSET_KEY_NO_EXIST;
        }

        existing_val.unwrap().to_owned()
    }

    pub fn get_del(&self, key: &DataType) -> Option<DataType> {
        match self.0.remove(key) {
            Some((_, entry)) => Some(entry),
            None => None,
        }
    }

    /// When the set command is ran, if a value with the key already exists, it replaces it and returns
    /// the old value
    pub fn set(&self, key: &DataType, value: &DataType) -> Option<DataType> {
        self.0.insert(key.to_owned(), value.to_owned())
    }
}

mod test {
    #![allow(unused_imports)]

    use super::Store;
    use crate::{
        data_type::{BoopBool, BoopError, BoopString, DataType, Int},
        errors::GETSET_KEY_NO_EXIST,
    };
    use bytes::Bytes;
    use dashmap::DashMap;
    use std::sync::Arc;

    #[test]
    fn set_and_get() {
        let store: Store = Store::new();
        let key = Int::new_u8(0x00);
        let val = Int::new_u8(0x01);

        let set_res = store.set(&key, &val);
        assert!(set_res.is_none());
        assert_eq!(store.get(&key).unwrap(), val);
    }

    #[test]
    fn getset_entry_exists() {
        let store: Store = Store::new();

        let key = BoopString::new(Bytes::from_static(b"getset"));
        let val = BoopBool::new(true);

        store.0.insert(key.clone(), val.clone());
        assert_eq!(store.get_set(&key, &BoopBool::new(false)), val);
    }

    #[test]
    fn getset_entry_does_not_exist() {
        let store: Store = Store::new();
        assert_eq!(
            store.get_set(
                &BoopString::new(Bytes::from_static(b"no_exist")),
                &BoopBool::new(false)
            ),
            GETSET_KEY_NO_EXIST
        );
    }

    #[test]
    fn getdel_entry_exists() {
        let store: Store = Store::new();
        let key = BoopString::new(Bytes::from_static(b"getset"));
        let val = BoopBool::new(true);

        store.0.insert(key.clone(), val.clone());

        assert_eq!(store.get_del(&key.clone()).unwrap(), val);
        assert!(store.get_del(&key.clone()).is_none());
        assert!(store.get(&key).is_none())
    }
}
