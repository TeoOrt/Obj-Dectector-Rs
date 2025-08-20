use std::collections::{BTreeMap, HashMap};

use anyhow::Result;

pub trait ConfigReader {
    fn get_values<T>()->Result<Vec<T>>; 
    fn get_keys<T>()->Result<Vec<T>>;
    fn get_key_values<T>()->Result<HashMap<T,T>>;
    fn get_tree<K,T>()->Result<BTreeMap<K,T>>;
}
