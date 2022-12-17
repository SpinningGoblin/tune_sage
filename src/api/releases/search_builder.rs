use serde::{Deserialize, Serialize};

use crate::{
    api::QueryOperator,
    components::releases::{Format, PrimaryGroupType, SecondaryGroupType, Status},
};

use super::ReleaseSearch;

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum ReleaseSearchBuilderError {
    #[error("Not enough options given for search")]
    NotEnoughOptionsGiven,
}

#[derive(Default)]
pub struct ReleaseSearchBuilder {
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
    pub operator: Option<QueryOperator>,
}

impl ReleaseSearchBuilder {
    pub fn new() -> ReleaseSearchBuilder {
        Self::default()
    }

    pub fn alias(&mut self, alias: &str) -> &mut ReleaseSearchBuilder {
        self.alias = Some(alias.to_string());

        self
    }

    pub fn arid(&mut self, arid: &str) -> &mut ReleaseSearchBuilder {
        self.arid = Some(arid.to_string());

        self
    }

    pub fn artist(&mut self, artist: &str) -> &mut ReleaseSearchBuilder {
        self.artist = Some(artist.to_string());

        self
    }

    pub fn artist_name(&mut self, artist_name: &str) -> &mut ReleaseSearchBuilder {
        self.artist_name = Some(artist_name.to_string());

        self
    }

    pub fn asin(&mut self, asin: &str) -> &mut ReleaseSearchBuilder {
        self.asin = Some(asin.to_string());

        self
    }

    pub fn barcode(&mut self, barcode: &str) -> &mut ReleaseSearchBuilder {
        self.barcode = Some(barcode.to_string());

        self
    }

    pub fn cat_no(&mut self, cat_no: &str) -> &mut ReleaseSearchBuilder {
        self.cat_no = Some(cat_no.to_string());

        self
    }

    pub fn comment(&mut self, comment: &str) -> &mut ReleaseSearchBuilder {
        self.comment = Some(comment.to_string());

        self
    }

    pub fn country(&mut self, country: &str) -> &mut ReleaseSearchBuilder {
        self.country = Some(country.to_string());

        self
    }

    pub fn credit_name(&mut self, credit_name: &str) -> &mut ReleaseSearchBuilder {
        self.credit_name = Some(credit_name.to_string());

        self
    }

    pub fn date(&mut self, date: &str) -> &mut ReleaseSearchBuilder {
        self.date = Some(date.to_string());

        self
    }

    pub fn disc_ids(&mut self, disc_ids: &str) -> &mut ReleaseSearchBuilder {
        self.disc_ids = Some(disc_ids.to_string());

        self
    }

    pub fn format(&mut self, format: Format) -> &mut ReleaseSearchBuilder {
        self.format = Some(format);

        self
    }

    pub fn la_id(&mut self, la_id: &str) -> &mut ReleaseSearchBuilder {
        self.la_id = Some(la_id.to_string());

        self
    }

    pub fn label(&mut self, label: &str) -> &mut ReleaseSearchBuilder {
        self.label = Some(label.to_string());

        self
    }

    pub fn lang(&mut self, lang: &str) -> &mut ReleaseSearchBuilder {
        self.lang = Some(lang.to_string());

        self
    }

    pub fn mediums(&mut self, mediums: u64) -> &mut ReleaseSearchBuilder {
        self.mediums = Some(mediums);

        self
    }

    pub fn re_id(&mut self, re_id: &str) -> &mut ReleaseSearchBuilder {
        self.re_id = Some(re_id.to_string());

        self
    }

    pub fn ipi(&mut self, ipi: &str) -> &mut ReleaseSearchBuilder {
        self.ipi = Some(ipi.to_string());

        self
    }

    pub fn release(&mut self, release: &str) -> &mut ReleaseSearchBuilder {
        self.release = Some(release.to_string());

        self
    }

    pub fn release_accent(&mut self, release_accent: &str) -> &mut ReleaseSearchBuilder {
        self.release_accent = Some(release_accent.to_string());

        self
    }

    pub fn rg_id(&mut self, rg_id: &str) -> &mut ReleaseSearchBuilder {
        self.rg_id = Some(rg_id.to_string());

        self
    }

    pub fn script(&mut self, script: &str) -> &mut ReleaseSearchBuilder {
        self.script = Some(script.to_string());

        self
    }

    pub fn status(&mut self, status: Status) -> &mut ReleaseSearchBuilder {
        self.status = Some(status);

        self
    }

    pub fn tag(&mut self, tag: &str) -> &mut ReleaseSearchBuilder {
        self.tag = Some(tag.to_string());

        self
    }

    pub fn tracks(&mut self, tracks: u64) -> &mut ReleaseSearchBuilder {
        self.tracks = Some(tracks);

        self
    }

    pub fn tracks_medium(&mut self, tracks_medium: u64) -> &mut ReleaseSearchBuilder {
        self.tracks_medium = Some(tracks_medium);

        self
    }

    pub fn primary_type(&mut self, primary_type: PrimaryGroupType) -> &mut ReleaseSearchBuilder {
        self.primary_type = Some(primary_type);

        self
    }

    pub fn release_type(&mut self, release_type: PrimaryGroupType) -> &mut ReleaseSearchBuilder {
        self.release_type = Some(release_type);

        self
    }

    pub fn secondary_type(
        &mut self,
        secondary_type: SecondaryGroupType,
    ) -> &mut ReleaseSearchBuilder {
        self.secondary_type = Some(secondary_type);

        self
    }

    pub fn operator(&mut self, operator: QueryOperator) -> &mut ReleaseSearchBuilder {
        self.operator = Some(operator);

        self
    }

    fn invalid_builder_options(&self) -> bool {
        self.alias.is_none()
            && self.arid.is_none()
            && self.artist.is_none()
            && self.artist_name.is_none()
            && self.asin.is_none()
            && self.barcode.is_none()
            && self.cat_no.is_none()
            && self.comment.is_none()
            && self.credit_name.is_none()
            && self.date.is_none()
            && self.disc_ids.is_none()
            && self.format.is_none()
            && self.ipi.is_none()
            && self.la_id.is_none()
            && self.label.is_none()
            && self.lang.is_none()
            && self.mediums.is_none()
            && self.primary_type.is_none()
            && self.re_id.is_none()
            && self.release.is_none()
            && self.release_accent.is_none()
            && self.release_type.is_none()
            && self.rg_id.is_none()
            && self.script.is_none()
            && self.secondary_type.is_none()
            && self.status.is_none()
            && self.tag.is_none()
            && self.tracks.is_none()
            && self.tracks_medium.is_none()
    }

    pub fn build(&self) -> Result<ReleaseSearch, ReleaseSearchBuilderError> {
        if self.invalid_builder_options() {
            return Err(ReleaseSearchBuilderError::NotEnoughOptionsGiven);
        }

        Ok(ReleaseSearch {
            alias: self.alias.to_owned(),
            artist: self.artist.to_owned(),
            comment: self.comment.to_owned(),
            country: self.country.to_owned(),
            ipi: self.ipi.to_owned(),
            tag: self.tag.to_owned(),
            operator: self.operator.to_owned().unwrap_or_default(),
            arid: self.arid.to_owned(),
            primary_type: self.primary_type.to_owned(),
            artist_name: self.artist_name.to_owned(),
            asin: self.asin.to_owned(),
            barcode: self.barcode.to_owned(),
            cat_no: self.cat_no.to_owned(),
            credit_name: self.credit_name.to_owned(),
            date: self.date.to_owned(),
            disc_ids: self.disc_ids.to_owned(),
            format: self.format.to_owned(),
            la_id: self.la_id.to_owned(),
            label: self.label.to_owned(),
            lang: self.lang.to_owned(),
            mediums: self.mediums.to_owned(),
            re_id: self.re_id.to_owned(),
            release: self.release.to_owned(),
            release_accent: self.release_accent.to_owned(),
            rg_id: self.rg_id.to_owned(),
            script: self.script.to_owned(),
            secondary_type: self.secondary_type.to_owned(),
            status: self.status.to_owned(),
            tracks: self.tracks.to_owned(),
            tracks_medium: self.tracks_medium.to_owned(),
            release_type: self.release_type.to_owned(),
        })
    }
}
