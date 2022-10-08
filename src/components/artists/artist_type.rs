use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ArtistType {
    Person,
    Group,
    Orchestra,
    Choir,
    Character,
    Other,
}

impl Default for ArtistType {
    fn default() -> Self {
        Self::Other
    }
}
