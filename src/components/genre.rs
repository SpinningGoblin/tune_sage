use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Genre {
    pub id: String,
    pub name: String,
    pub disambiguation: String,
    pub count: u64,
}
