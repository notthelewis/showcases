use crate::data_type::DataType;
use dashmap::DashMap;
use std::sync::Arc;

pub type Store = Arc<DashMap<DataType, DataType>>;
