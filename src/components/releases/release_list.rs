use serde::{Deserialize, Serialize};

use super::Release;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseList {
    pub created: String,
    #[serde(default)]
    pub count: u32,
    #[serde(default)]
    pub offset: u32,
    #[serde(default)]
    pub releases: Vec<Release>,
}
