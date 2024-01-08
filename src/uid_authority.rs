use std::{sync::Arc, collections::HashSet};

use parking_lot::Mutex;
use uuid::Uuid;


pub struct UIDAuthority {
    registry: Arc<Mutex<HashSet<Uuid>>>,
}

impl From<Arc<Mutex<HashSet<Uuid>>>> for UIDAuthority {
    fn from(value: Arc<Mutex<HashSet<Uuid>>>) -> Self {
        Self {
            registry: value,
        }
    }
}

impl UIDAuthority {
    pub fn generate_uid(&self) -> Uuid {
        loop {
            let uid = Uuid::new_v4();
            if !self.registry.lock().insert(uid) {
                continue;
            }
            return uid;
        }
    }

    /// Adds a value to the set.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this value, `true` is returned.
    /// - If the set already contained this value, `false` is returned,
    ///   and the set is not modified: original value is not replaced,
    ///   and the value passed as argument is dropped.
    pub fn insert(&self, uid: Uuid) -> bool {
        self.registry.lock().insert(uid)
    }
}