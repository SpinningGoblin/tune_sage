use serde::{Deserialize, Serialize};

use super::{release_event::ReleaseEvent, IncludedReleaseGroup, Media, Status, TextRepresentation};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Release {
    pub id: String,
    #[serde(alias = "status-id")]
    pub status_id: Option<String>,
    pub status: Option<Status>,
    pub title: String,
    pub disambiguation: Option<String>,
    pub country: Option<String>,
    pub quality: Option<String>,
    pub date: Option<String>,
    pub barcode: Option<String>,
    pub packaging: Option<String>,
    #[serde(alias = "packaging-id")]
    pub packaging_id: Option<String>,
    #[serde(alias = "text-representation")]
    pub text_representation: Option<TextRepresentation>,
    #[serde(alias = "release-events")]
    pub release_events: Option<Vec<ReleaseEvent>>,
    #[serde(alias = "release-group")]
    pub release_group: Option<IncludedReleaseGroup>,
    pub media: Option<Vec<Media>>,
}
