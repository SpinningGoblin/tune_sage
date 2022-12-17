use serde::{Deserialize, Serialize};

use crate::{api::QueryOperator, components::artists::ArtistType};

use super::{ArtistGenderQuery, ArtistSearch};

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum ArtistSearchBuilderError {
    #[error("Not enough options given for search")]
    NotEnoughOptionsGiven,
}

#[derive(Default)]
pub struct ArtistSearchBuilder {
    alias: Option<String>,
    primary_alias: Option<String>,
    area: Option<String>,
    artist: Option<String>,
    artist_accent: Option<String>,
    begin: Option<String>,
    begin_area: Option<String>,
    comment: Option<String>,
    country: Option<String>,
    end: Option<String>,
    end_area: Option<String>,
    ended: Option<bool>,
    gender: Option<ArtistGenderQuery>,
    ipi: Option<String>,
    isni: Option<String>,
    sort_name: Option<String>,
    tag: Option<String>,
    artist_type: Option<ArtistType>,
    operator: Option<QueryOperator>,
}

impl ArtistSearchBuilder {
    pub fn new() -> ArtistSearchBuilder {
        Self::default()
    }

    pub fn alias(&mut self, alias: &str) -> &mut ArtistSearchBuilder {
        self.alias = Some(alias.to_string());

        self
    }

    pub fn primary_alias(&mut self, primary_alias: &str) -> &mut ArtistSearchBuilder {
        self.primary_alias = Some(primary_alias.to_string());

        self
    }

    pub fn artist(&mut self, artist: &str) -> &mut ArtistSearchBuilder {
        self.artist = Some(artist.to_string());

        self
    }

    pub fn artist_accent(&mut self, artist_accent: &str) -> &mut ArtistSearchBuilder {
        self.artist_accent = Some(artist_accent.to_string());

        self
    }

    pub fn begin(&mut self, begin: &str) -> &mut ArtistSearchBuilder {
        self.begin = Some(begin.to_string());

        self
    }

    pub fn begin_area(&mut self, begin_area: &str) -> &mut ArtistSearchBuilder {
        self.begin_area = Some(begin_area.to_string());

        self
    }

    pub fn comment(&mut self, comment: &str) -> &mut ArtistSearchBuilder {
        self.comment = Some(comment.to_string());

        self
    }

    pub fn country(&mut self, country: &str) -> &mut ArtistSearchBuilder {
        self.country = Some(country.to_string());

        self
    }

    pub fn end(&mut self, end: &str) -> &mut ArtistSearchBuilder {
        self.end = Some(end.to_string());

        self
    }

    pub fn end_area(&mut self, end_area: &str) -> &mut ArtistSearchBuilder {
        self.end_area = Some(end_area.to_string());

        self
    }

    pub fn ended(&mut self, ended: bool) -> &mut ArtistSearchBuilder {
        self.ended = Some(ended);

        self
    }

    pub fn gender(&mut self, gender: ArtistGenderQuery) -> &mut ArtistSearchBuilder {
        self.gender = Some(gender);

        self
    }

    pub fn ipi(&mut self, ipi: &str) -> &mut ArtistSearchBuilder {
        self.ipi = Some(ipi.to_string());

        self
    }

    pub fn isni(&mut self, isni: &str) -> &mut ArtistSearchBuilder {
        self.isni = Some(isni.to_string());

        self
    }

    pub fn sort_name(&mut self, sort_name: &str) -> &mut ArtistSearchBuilder {
        self.sort_name = Some(sort_name.to_string());

        self
    }

    pub fn tag(&mut self, tag: &str) -> &mut ArtistSearchBuilder {
        self.tag = Some(tag.to_string());

        self
    }

    pub fn artist_type(&mut self, artist_type: ArtistType) -> &mut ArtistSearchBuilder {
        self.artist_type = Some(artist_type);

        self
    }

    pub fn operator(&mut self, operator: QueryOperator) -> &mut ArtistSearchBuilder {
        self.operator = Some(operator);

        self
    }

    fn invalid_builder_options(&self) -> bool {
        self.alias.is_none()
            && self.primary_alias.is_none()
            && self.area.is_none()
            && self.artist.is_none()
            && self.artist_accent.is_none()
            && self.begin.is_none()
            && self.begin_area.is_none()
            && self.comment.is_none()
            && self.country.is_none()
            && self.end.is_none()
            && self.end_area.is_none()
            && self.ended.is_none()
            && self.gender.is_none()
            && self.ipi.is_none()
            && self.isni.is_none()
            && self.sort_name.is_none()
            && self.tag.is_none()
            && self.artist_type.is_none()
    }

    pub fn build(&self) -> Result<ArtistSearch, ArtistSearchBuilderError> {
        if self.invalid_builder_options() {
            return Err(ArtistSearchBuilderError::NotEnoughOptionsGiven);
        }

        Ok(ArtistSearch {
            alias: self.alias.clone(),
            primary_alias: self.primary_alias.clone(),
            area: self.area.clone(),
            artist: self.artist.clone(),
            artist_accent: self.artist_accent.clone(),
            begin: self.begin.clone(),
            begin_area: self.begin_area.clone(),
            comment: self.comment.clone(),
            country: self.country.clone(),
            end: self.end.clone(),
            end_area: self.end_area.clone(),
            ended: self.ended,
            gender: self.gender.clone(),
            ipi: self.ipi.clone(),
            isni: self.isni.clone(),
            sort_name: self.sort_name.clone(),
            tag: self.tag.clone(),
            artist_type: self.artist_type.clone(),
            operator: self.operator.clone().unwrap_or_default(),
        })
    }
}
