use crate::data_type::{BoopError, DataType};
use dashmap::DashMap;
use std::sync::Arc;

/// Store is the conccurent hashmap that is the core of `Blewis`. It is a concurrent hashmap based
/// on `DashMap` which itself is based on Google's SwissTable. It is a highly performant,
/// concurrent HashMap that uses shards of RWLocks. The store does allow for weird data structures
/// that perhaps might seem counter intuitive at first. This is to cater for weird and wonderful
/// use cases. It is, for example, possible to store a Boolean as a key and an array of Errors for
/// the value. In fact, any data type that can be encoded via BOOP can be used as a key or a value.
pub struct Store(Arc<DashMap<DataType, DataType>>);

impl Store {
    #[inline(always)]
    pub fn new() -> Self {
        Store(Arc::new(DashMap::new()))
    }

    /// Creates a new Store with a preset capacity
    #[inline(always)]
    pub fn with_capacity(cap: usize) -> Self {
        Store(Arc::new(DashMap::with_capacity(cap)))
    }

    /// Creates a new Store with a preset capacity and shard amount. The shard amount must be a
    /// power of two. If a none power of two is selected, the program will panic.
    #[inline(always)]
    pub fn with_capacity_and_shard_amount(cap: usize, shard_amount: usize) -> Self {
        Store(Arc::new(DashMap::with_capacity_and_shard_amount(
            cap,
            shard_amount,
        )))
    }

    /// Retrieves a value from the store
    #[inline(always)]
    pub fn get(&self, key: &DataType) -> Option<DataType> {
        let res = self.0.get(key);

        res.map(|v| v.to_owned())
    }

    /// Simillar to set but instead of returning an Option<DataType>, it returns an error code if
    /// a previous value was not set
    #[inline(always)]
    pub fn get_set(&self, key: &DataType, new_val: &DataType) -> DataType {
        let existing_val = self.0.insert(key.to_owned(), new_val.to_owned());

        if existing_val.is_none() {
            return DataType::Error(BoopError {
                is_server_err: true,
                err_code: 0x10,
                err_msg: bytes::Bytes::from_static(b"no_exist"),
            });
        }

        existing_val.unwrap().to_owned()
    }

    #[inline(always)]
    pub fn get_del(&self, key: &DataType) -> Option<DataType> {
        self.0.remove(key).map(|(_, entry)| entry )
    }

    /// When the set command is ran, if a value with the key already exists, it replaces it and returns
    /// the old value
    #[inline(always)]
    pub fn set(&self, key: &DataType, value: &DataType) -> Option<DataType> {
        self.0.insert(key.to_owned(), value.to_owned())
    }
}

mod test {
    #![allow(unused_imports)]

    use super::Store;
    use crate::data_type::{BoopBool, BoopError, BoopString, DataType, Int};
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

        let key = BoopString::new_wrapped(Bytes::from_static(b"getset"));
        let val = BoopBool::new_wrapped(true);

        store.0.insert(key.clone(), val.clone());
        assert_eq!(store.get_set(&key, &BoopBool::new_wrapped(false)), val);
    }

    #[test]
    fn getset_entry_does_not_exist() {
        let store: Store = Store::new();
        assert_eq!(
            store.get_set(
                &BoopString::new_wrapped(Bytes::from_static(b"no_exist")),
                &BoopBool::new_wrapped(false)
            ),
            DataType::Error(BoopError {
                is_server_err: true,
                err_code: 0x10,
                err_msg: bytes::Bytes::from_static(b"no_exist"),
            })
        );
    }

    #[test]
    fn getdel_entry_exists() {
        let store: Store = Store::new();
        let key = BoopString::new_wrapped(Bytes::from_static(b"getset"));
        let val = BoopBool::new_wrapped(true);

        store.0.insert(key.clone(), val.clone());

        assert_eq!(store.get_del(&key.clone()).unwrap(), val);
        assert!(store.get_del(&key.clone()).is_none());
        assert!(store.get(&key).is_none())
    }
}
