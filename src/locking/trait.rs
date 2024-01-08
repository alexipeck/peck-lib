use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;

pub trait ExtIntRwLockToOwned<K, V> {
    fn lock_free_to_owned(&self) -> HashMap<K, V>;
}

impl<K: Copy + std::hash::Hash + Eq, V: Clone> ExtIntRwLockToOwned<K, V> for Arc<RwLock<HashMap<K, Arc<RwLock<V>>>>> {
    fn lock_free_to_owned(&self) -> HashMap<K, V> {
        self.read()
            .iter()
            .map(|(&key, value_lock)| (key, value_lock.read().clone()))
            .collect()
    }
}