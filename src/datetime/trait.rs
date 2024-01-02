use chrono::{DateTime, Utc};

pub trait Expired {
    fn expired(&self) -> bool;
}

impl Expired for DateTime<Utc> {
    fn expired(&self) -> bool {
        (self.timestamp() - Utc::now().timestamp()).is_negative()
    }
}
