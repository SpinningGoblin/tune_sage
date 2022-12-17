use crate::{
    api::QueryOperator,
    components::releases::{Format, PrimaryGroupType, SecondaryGroupType, Status},
};

pub struct ReleaseSearch {
    pub alias: Option<String>,
    pub arid: Option<String>,
    pub primary_type: Option<PrimaryGroupType>,
    pub artist: Option<String>,
    pub artist_name: Option<String>,
    pub asin: Option<String>,
    pub barcode: Option<String>,
    pub cat_no: Option<String>,
    pub comment: Option<String>,
    pub country: Option<String>,
    pub credit_name: Option<String>,
    pub date: Option<String>,
    pub disc_ids: Option<String>,
    pub format: Option<Format>,
    pub la_id: Option<String>,
    pub label: Option<String>,
    pub lang: Option<String>,
    pub mediums: Option<u64>,
    pub re_id: Option<String>,
    pub ipi: Option<String>,
    pub release: Option<String>,
    pub release_accent: Option<String>,
    pub rg_id: Option<String>,
    pub script: Option<String>,
    pub secondary_type: Option<SecondaryGroupType>,
    pub status: Option<Status>,
    pub tag: Option<String>,
    pub tracks: Option<u64>,
    pub tracks_medium: Option<u64>,
    pub release_type: Option<PrimaryGroupType>,
    pub operator: QueryOperator,
}