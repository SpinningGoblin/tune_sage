use serde::{Deserialize, Serialize};

use super::{PrimaryGroupType, SecondaryGroupType};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IncludedReleaseGroup {
    pub id: String,
    pub disambiguation: Option<String>,
    pub title: String,
    #[serde(alias = "primary-type-id")]
    pub primary_type_id: Option<String>,
    #[serde(alias = "first-release-date")]
    pub first_release_date: Option<String>,
    #[serde(alias = "primary-type")]
    pub primary_type: Option<PrimaryGroupType>,
    #[serde(alias = "secondary-types")]
    pub secondary_types: Option<Vec<SecondaryGroupType>>,
    #[serde(alias = "secondary-type-ids")]
    pub secondary_type_ids: Option<Vec<String>>,
}
