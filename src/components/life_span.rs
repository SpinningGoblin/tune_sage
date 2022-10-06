use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LifeSpan {
    pub ended: Option<bool>,
    pub begin: Option<String>,
    pub end: Option<String>,
}
