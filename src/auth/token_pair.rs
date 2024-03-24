use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::datetime::serde::datetime_utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub token: String,
    #[serde(with = "datetime_utc")]
    pub expiry: DateTime<Utc>,
}
