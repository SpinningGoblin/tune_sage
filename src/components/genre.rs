use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Genre {
    pub id: String,
    pub name: String,
    pub disambiguation: Option<String>,
    pub count: i64,
}
