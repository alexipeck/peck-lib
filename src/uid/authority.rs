use std::{collections::HashSet, sync::Arc};

use parking_lot::Mutex;
use uuid::Uuid;

use super::error::UIDAuthorityError;

pub struct UIDAuthority {
    registry: Arc<Mutex<HashSet<Uuid>>>,
}

impl From<Arc<Mutex<HashSet<Uuid>>>> for UIDAuthority {
    fn from(value: Arc<Mutex<HashSet<Uuid>>>) -> Self {
        Self { registry: value }
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

    pub fn insert(&self, uid: Uuid) -> Result<(), UIDAuthorityError> {
        if !self.registry.lock().insert(uid) {
            return Err(UIDAuthorityError::DuplicateUIDInserted(uid));
        } else {
            Ok(())
        }
    }

    pub fn insert_bulk(&self, uids: Vec<Uuid>) -> Result<(), UIDAuthorityError> {
        let mut duplicates: Vec<Uuid> = Vec::new();
        let mut lock = self.registry.lock();
        for uid in uids {
            if !lock.insert(uid) {
                duplicates.push(uid);
            }
        }
        if duplicates.is_empty() {
            Ok(())
        } else {
            return Err(UIDAuthorityError::DuplicateUIDsInserted(duplicates));
        }
    }
}
