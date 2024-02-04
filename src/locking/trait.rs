use std::{collections::HashMap, sync::Arc};

use futures_util::{stream, StreamExt};

pub trait ExtIntParkingLotRwLockToOwned<K, V> {
    fn lock_free_to_owned(&self) -> HashMap<K, V>;
}

impl<K: Copy + std::hash::Hash + Eq, V: Clone> ExtIntParkingLotRwLockToOwned<K, V>
    for Arc<parking_lot::RwLock<HashMap<K, Arc<parking_lot::RwLock<V>>>>>
{
    fn lock_free_to_owned(&self) -> HashMap<K, V> {
        self.read()
            .iter()
            .map(|(&key, value_lock)| (key, value_lock.read().clone()))
            .collect()
    }
}

pub trait ExtIntTokioRwLockToOwned<K, V> {
    fn lock_free_to_owned(&self) -> impl std::future::Future<Output = HashMap<K, V>> + Send;
}

impl<K: Copy + std::hash::Hash + Eq + Send + Sync + 'static, V: Clone + Send + Sync + 'static>
    ExtIntTokioRwLockToOwned<K, V>
    for Arc<tokio::sync::RwLock<HashMap<K, Arc<tokio::sync::RwLock<V>>>>>
{
    async fn lock_free_to_owned(&self) -> HashMap<K, V> {
        stream::iter(self.read().await.iter())
            .then(|(&key, value_lock)| async move { (key, value_lock.read().await.clone()) })
            .collect::<Vec<(K, V)>>()
            .await
            .into_iter()
            .collect()
    }
}
