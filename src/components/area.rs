use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Area {
    #[serde(alias = "type")]
    pub area_type: Option<AreaType>,
    pub id: String,
    pub disambiguation: Option<String>,
    pub name: String,
    #[serde(alias = "sort-name")]
    pub sort_name: Option<String>,
    #[serde(alias = "iso-3166-1-codes", default)]
    pub iso_3166_1_codes: Vec<String>,
    #[serde(alias = "iso-3166-2-codes", default)]
    pub iso_3166_2_codes: Vec<String>,
    #[serde(alias = "iso-3166-3-codes", default)]
    pub iso_3166_3_codes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AreaType {
    Country,
    Subdivision,
    County,
    Municipality,
    City,
    District,
    Island,
}
