use crate::{
    api::QueryOperator,
    components::releases::{PrimaryGroupType, SecondaryGroupType, Status},
};

pub struct RecordingSearch {
    pub alias: Option<String>,
    pub arid: Option<String>,
    pub artist: Option<String>,
    pub artist_name: Option<String>,
    pub comment: Option<String>,
    pub country: Option<String>,
    pub credit_name: Option<String>,
    pub date: Option<String>,
    pub dur: Option<u64>,
    pub first_release_date: Option<String>,
    pub format: Option<String>,
    pub isrc: Option<String>,
    pub number: Option<String>,
    pub position: Option<u64>,
    pub primary_type: Option<PrimaryGroupType>,
    pub qdur: Option<u64>,
    pub recording: Option<String>,
    pub recording_accent: Option<String>,
    pub reid: Option<String>,
    pub release: Option<String>,
    pub rgid: Option<String>,
    pub rid: Option<String>,
    pub secondary_type: Option<SecondaryGroupType>,
    pub status: Option<Status>,
    pub tag: Option<String>,
    pub tid: Option<String>,
    pub tnum: Option<u64>,
    pub tracks: Option<u64>,
    pub tracks_release: Option<u64>,
    pub video: Option<bool>,
    pub operator: QueryOperator,
}