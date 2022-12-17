use serde::{Deserialize, Serialize};

use crate::{
    api::QueryOperator,
    components::releases::{PrimaryGroupType, SecondaryGroupType, Status},
};

use super::RecordingSearch;

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum RecordingSearchBuilderError {
    #[error("Not enough options given for search")]
    NotEnoughOptionsGiven,
}

#[derive(Default)]
pub struct RecordingSearchBuilder {
    alias: Option<String>,
    arid: Option<String>,
    artist: Option<String>,
    artist_name: Option<String>,
    comment: Option<String>,
    country: Option<String>,
    credit_name: Option<String>,
    date: Option<String>,
    dur: Option<u64>,
    first_release_date: Option<String>,
    format: Option<String>,
    isrc: Option<String>,
    number: Option<String>,
    position: Option<u64>,
    primary_type: Option<PrimaryGroupType>,
    qdur: Option<u64>,
    recording: Option<String>,
    recording_accent: Option<String>,
    reid: Option<String>,
    release: Option<String>,
    rgid: Option<String>,
    rid: Option<String>,
    secondary_type: Option<SecondaryGroupType>,
    status: Option<Status>,
    tag: Option<String>,
    tid: Option<String>,
    tnum: Option<u64>,
    tracks: Option<u64>,
    tracks_release: Option<u64>,
    video: Option<bool>,
    operator: Option<QueryOperator>,
}

impl RecordingSearchBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alias(&mut self, alias: &str) -> &mut RecordingSearchBuilder {
        self.alias = Some(alias.to_string());

        self
    }

    pub fn arid(&mut self, arid: &str) -> &mut RecordingSearchBuilder {
        self.arid = Some(arid.to_string());

        self
    }

    pub fn artist(&mut self, artist: &str) -> &mut RecordingSearchBuilder {
        self.artist = Some(artist.to_string());

        self
    }

    pub fn artist_name(&mut self, artist_name: &str) -> &mut RecordingSearchBuilder {
        self.artist_name = Some(artist_name.to_string());

        self
    }

    pub fn comment(&mut self, comment: &str) -> &mut RecordingSearchBuilder {
        self.comment = Some(comment.to_string());

        self
    }

    pub fn country(&mut self, country: &str) -> &mut RecordingSearchBuilder {
        self.country = Some(country.to_string());

        self
    }

    pub fn credit_name(&mut self, credit_name: &str) -> &mut RecordingSearchBuilder {
        self.credit_name = Some(credit_name.to_string());

        self
    }

    pub fn date(&mut self, date: &str) -> &mut RecordingSearchBuilder {
        self.date = Some(date.to_string());

        self
    }

    pub fn dur(&mut self, dur: u64) -> &mut RecordingSearchBuilder {
        self.dur = Some(dur);

        self
    }

    pub fn first_release_date(&mut self, first_release_date: &str) -> &mut RecordingSearchBuilder {
        self.first_release_date = Some(first_release_date.to_string());

        self
    }

    pub fn format(&mut self, format: &str) -> &mut RecordingSearchBuilder {
        self.format = Some(format.to_string());

        self
    }

    pub fn isrc(&mut self, isrc: &str) -> &mut RecordingSearchBuilder {
        self.isrc = Some(isrc.to_string());

        self
    }

    pub fn number(&mut self, number: &str) -> &mut RecordingSearchBuilder {
        self.number = Some(number.to_string());

        self
    }

    pub fn position(&mut self, position: u64) -> &mut RecordingSearchBuilder {
        self.position = Some(position);

        self
    }

    pub fn primary_type(&mut self, primary_type: PrimaryGroupType) -> &mut RecordingSearchBuilder {
        self.primary_type = Some(primary_type);

        self
    }

    pub fn qdur(&mut self, qdur: u64) -> &mut RecordingSearchBuilder {
        self.qdur = Some(qdur);

        self
    }

    pub fn recording(&mut self, recording: &str) -> &mut RecordingSearchBuilder {
        self.recording = Some(recording.to_string());

        self
    }

    pub fn recording_accent(&mut self, recording_accent: &str) -> &mut RecordingSearchBuilder {
        self.recording_accent = Some(recording_accent.to_string());

        self
    }

    pub fn rgid(&mut self, rgid: &str) -> &mut RecordingSearchBuilder {
        self.rgid = Some(rgid.to_string());

        self
    }

    pub fn rid(&mut self, rid: &str) -> &mut RecordingSearchBuilder {
        self.rid = Some(rid.to_string());

        self
    }

    pub fn secondary_type(
        &mut self,
        secondary_type: SecondaryGroupType,
    ) -> &mut RecordingSearchBuilder {
        self.secondary_type = Some(secondary_type);

        self
    }

    pub fn status(&mut self, status: Status) -> &mut RecordingSearchBuilder {
        self.status = Some(status);

        self
    }

    pub fn tag(&mut self, tag: &str) -> &mut RecordingSearchBuilder {
        self.tag = Some(tag.to_string());

        self
    }

    pub fn tid(&mut self, tid: &str) -> &mut RecordingSearchBuilder {
        self.tid = Some(tid.to_string());

        self
    }

    pub fn tnum(&mut self, tnum: u64) -> &mut RecordingSearchBuilder {
        self.tnum = Some(tnum);

        self
    }

    pub fn tracks(&mut self, tracks: u64) -> &mut RecordingSearchBuilder {
        self.tracks = Some(tracks);

        self
    }

    pub fn tracks_release(&mut self, tracks_release: u64) -> &mut RecordingSearchBuilder {
        self.tracks_release = Some(tracks_release);

        self
    }

    pub fn video(&mut self, video: bool) -> &mut RecordingSearchBuilder {
        self.video = Some(video);

        self
    }

    pub fn operator(&mut self, operator: QueryOperator) -> &mut RecordingSearchBuilder {
        self.operator = Some(operator);

        self
    }

    fn invalid_builder_options(&self) -> bool {
        self.alias.is_none()
            && self.arid.is_none()
            && self.artist.is_none()
            && self.artist_name.is_none()
            && self.comment.is_none()
            && self.country.is_none()
            && self.credit_name.is_none()
            && self.date.is_none()
            && self.dur.is_none()
            && self.first_release_date.is_none()
            && self.format.is_none()
            && self.isrc.is_none()
            && self.number.is_none()
            && self.position.is_none()
            && self.primary_type.is_none()
            && self.qdur.is_none()
            && self.recording.is_none()
            && self.recording_accent.is_none()
            && self.reid.is_none()
            && self.release.is_none()
            && self.rgid.is_none()
            && self.rid.is_none()
            && self.secondary_type.is_none()
            && self.status.is_none()
            && self.tag.is_none()
            && self.tid.is_none()
            && self.tnum.is_none()
            && self.tracks.is_none()
            && self.tracks_release.is_none()
            && self.video.is_none()
    }

    pub fn build(&self) -> Result<RecordingSearch, RecordingSearchBuilderError> {
        if self.invalid_builder_options() {
            return Err(RecordingSearchBuilderError::NotEnoughOptionsGiven);
        }

        Ok(RecordingSearch {
            alias: self.alias.clone(),
            artist: self.artist.clone(),
            comment: self.comment.clone(),
            country: self.country.clone(),
            tag: self.tag.clone(),
            operator: self.operator.clone().unwrap_or_default(),
            arid: self.arid.clone(),
            artist_name: self.artist_name.clone(),
            credit_name: self.credit_name.clone(),
            date: self.date.clone(),
            dur: self.dur,
            first_release_date: self.first_release_date.clone(),
            format: self.format.clone(),
            isrc: self.isrc.clone(),
            number: self.number.clone(),
            position: self.position,
            primary_type: self.primary_type.clone(),
            qdur: self.qdur,
            recording: self.recording.clone(),
            recording_accent: self.recording_accent.clone(),
            reid: self.reid.clone(),
            release: self.release.clone(),
            rgid: self.rgid.clone(),
            rid: self.rid.clone(),
            secondary_type: self.secondary_type.clone(),
            status: self.status.clone(),
            tid: self.tid.clone(),
            tnum: self.tnum,
            tracks: self.tracks,
            tracks_release: self.tracks_release,
            video: self.video,
        })
    }
}
