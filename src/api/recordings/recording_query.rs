use urlencoding::encode;

use crate::{
    api::QueryOperator,
    components::releases::{PrimaryGroupType, SecondaryGroupType, Status},
};

use super::RecordingSearch;

pub enum RecordingQuery {
    Term(String),
    Search(Box<RecordingSearch>),
}

impl RecordingQuery {
    pub fn to_query(&self) -> String {
        match self {
            RecordingQuery::Term(term) => term.to_string(),
            RecordingQuery::Search(search) => {
                let mut searches: Vec<String> = Vec::new();

                if let Some(alias) = &search.alias {
                    searches.push(format!("alias:{}", encode(alias)));
                }

                if let Some(arid) = &search.arid {
                    searches.push(format!("arid:{}", encode(arid)));
                }

                if let Some(artist) = &search.artist {
                    searches.push(format!("artist:{}", encode(artist)));
                }

                if let Some(artist_name) = &search.artist_name {
                    searches.push(format!("artistname:{}", encode(artist_name)));
                }

                if let Some(comment) = &search.comment {
                    searches.push(format!("comment:{}", encode(comment)));
                }

                if let Some(country) = &search.country {
                    searches.push(format!("country:{}", encode(country)));
                }

                if let Some(credit_name) = &search.credit_name {
                    searches.push(format!("creditname:{}", encode(credit_name)));
                }

                if let Some(date) = &search.date {
                    searches.push(format!("date:{}", encode(date)));
                }

                if let Some(dur) = &search.dur {
                    searches.push(format!("dur:{}", dur));
                }

                if let Some(first_release_date) = &search.first_release_date {
                    searches.push(format!("firstreleasedate:{}", encode(first_release_date)));
                }

                if let Some(format) = &search.format {
                    searches.push(format!("format:{}", encode(format)));
                }

                if let Some(isrc) = &search.isrc {
                    searches.push(format!("isrc:{}", encode(isrc)));
                }

                if let Some(number) = &search.number {
                    searches.push(format!("number:{}", encode(number)));
                }

                if let Some(position) = &search.position {
                    searches.push(format!("position:{}", position));
                }

                if let Some(primary_type) = &search.primary_type {
                    let type_search = match primary_type {
                        PrimaryGroupType::Album => "Album",
                        PrimaryGroupType::Single => "Single",
                        PrimaryGroupType::EP => "EP",
                        PrimaryGroupType::Broadcast => "Broadcast",
                        PrimaryGroupType::Other => "Other",
                    };
                    searches.push(format!("primarytype:{}", encode(type_search)));
                }

                if let Some(qdur) = &search.qdur {
                    searches.push(format!("qdur:{}", qdur));
                }

                if let Some(recording) = &search.recording {
                    searches.push(format!("recording:{}", encode(recording)));
                }

                if let Some(recording_accent) = &search.recording_accent {
                    searches.push(format!("recordingaccent:{}", encode(recording_accent)));
                }

                if let Some(reid) = &search.reid {
                    searches.push(format!("reid:{}", encode(reid)));
                }

                if let Some(release) = &search.release {
                    searches.push(format!("release:{}", encode(release)));
                }

                if let Some(rgid) = &search.rgid {
                    searches.push(format!("rgid:{}", encode(rgid)));
                }

                if let Some(rid) = &search.rid {
                    searches.push(format!("rid:{}", encode(rid)));
                }

                if let Some(secondary_type) = &search.secondary_type {
                    let type_search = match secondary_type {
                        SecondaryGroupType::Compilation => "Compilation",
                        SecondaryGroupType::Soundtrack => "Soundtrack",
                        SecondaryGroupType::SpokenWord => "Spokenword",
                        SecondaryGroupType::Interview => "Interview",
                        SecondaryGroupType::AudioBook => "Audiobook",
                        SecondaryGroupType::AudioDrama => "Audio drama",
                        SecondaryGroupType::Live => "Live",
                        SecondaryGroupType::Remix => "Remix",
                        SecondaryGroupType::DjMix => "DJ-mix",
                        SecondaryGroupType::MixTapeStreet => "Mixtape/Street",
                    };
                    searches.push(format!("secondarytype:{}", encode(type_search)));
                }

                if let Some(status) = &search.status {
                    let status_search = match status {
                        Status::Official => "Official",
                        Status::Promotion => "Promotion",
                        Status::Bootleg => "Bootleg",
                        Status::PseudoRelease => "Pseudorelease",
                        Status::Withdrawn => "Withdrawn",
                        Status::Cancelled => "Cancelled",
                    };
                    searches.push(format!("status:{}", encode(status_search)));
                }

                if let Some(tag) = &search.tag {
                    searches.push(format!("tag:{}", encode(tag)));
                }

                if let Some(tid) = &search.tid {
                    searches.push(format!("tid:{}", encode(tid)));
                }

                if let Some(tnum) = &search.tnum {
                    searches.push(format!("tnum:{}", tnum));
                }

                if let Some(tracks) = &search.tracks {
                    searches.push(format!("tracks:{}", tracks));
                }

                if let Some(tracks_release) = &search.tracks_release {
                    searches.push(format!("tracksrelease:{}", tracks_release));
                }

                if let Some(video) = &search.video {
                    searches.push(format!("video:{}", video));
                }

                let separator = match search.operator {
                    QueryOperator::And => "%20AND%20",
                    QueryOperator::Or => "%20OR%20",
                };

                searches.join(separator)
            }
        }
    }
}
