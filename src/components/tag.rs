use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tag {
    pub count: u64,
    pub name: String,
}
