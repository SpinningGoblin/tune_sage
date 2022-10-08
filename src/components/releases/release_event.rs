use serde::{Deserialize, Serialize};

use crate::components::Area;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleaseEvent {
    pub date: Option<String>,
    pub area: Option<Area>,
}
