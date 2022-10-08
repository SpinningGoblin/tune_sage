use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextRepresentation {
    pub script: Option<String>,
    pub language: Option<String>,
}
