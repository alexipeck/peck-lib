use bloom::{BloomFilter, ASMS};
use std::{collections::HashSet, fmt, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct UIDAuthority {
    registry: Arc<Mutex<BloomFilter>>,
}

impl fmt::Debug for UIDAuthority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl From<BloomFilter> for UIDAuthority {
    fn from(value: BloomFilter) -> Self {
        Self {
            registry: Arc::new(Mutex::new(value)),
        }
    }
}

///bloom filter size is optimized on every restart
impl From<HashSet<Uuid>> for UIDAuthority {
    fn from(value: HashSet<Uuid>) -> Self {
        ///0.01% false positive rate
        let mut registry: BloomFilter = BloomFilter::with_rate(
            0.0001,
            if value.len() < 250000 {
                500000
            } else {
                (value.len() as f64 * 1.5) as u32
            },
        );
        for value in value {
            registry.insert(&value);
        }

        Self {
            registry: Arc::new(Mutex::new(registry)),
        }
    }
}

impl UIDAuthority {
    pub async fn generate_uid(&self) -> Uuid {
        loop {
            let uid: Uuid = Uuid::new_v4();
            if !self.registry.lock().await.insert(&uid) {
                continue;
            }
            return uid;
        }
    }

    pub async fn insert(&self, uid: &Uuid) {
        let _ = self.registry.lock().await.insert(uid);
    }

    pub async fn insert_bulk(&self, uids: Vec<Uuid>) {
        let mut registry_mutex_guard = self.registry.lock().await;
        for uid in uids {
            let _ = registry_mutex_guard.insert(&uid);
        }
    }
}
