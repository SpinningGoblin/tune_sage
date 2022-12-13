use serde::{Deserialize, Serialize};

use super::{release_event::ReleaseEvent, IncludedReleaseGroup, Status, TextRepresentation};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IncludedRelease {
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
}

impl IncludedRelease {
    pub fn is_official(&self) -> bool {
        self.status.eq(&Some(Status::Official))
    }
}

#[cfg(test)]
mod tests {
    use super::IncludedRelease;

    #[test]
    fn deserialize() {
        let s = r#"{
            "id": "5d854800-c062-4bb0-b905-fe721fb5ae14",
            "status-id": "4e304316-386d-3409-af2e-78857eec5cfe",
            "packaging-id": "119eba76-b343-3e02-a292-f0f00644bb9b",
            "disambiguation": "",
            "status": "Official",
            "text-representation": { "script": "Latn", "language": "eng" },
            "quality": "normal",
            "release-events": [
              {
                "date": "2022-09-30",
                "area": {
                  "name": "[Worldwide]",
                  "type-id": null,
                  "sort-name": "[Worldwide]",
                  "id": "525d4e18-3d00-31b9-a58b-a146a916de8f",
                  "disambiguation": "",
                  "iso-3166-1-codes": ["XW"],
                  "type": null
                }
              }
            ],
            "country": "XW",
            "title": "Viscera",
            "barcode": null,
            "date": "2022-09-30",
            "packaging": "None"
          }"#;

        let included_release: IncludedRelease = serde_json::from_str(s).unwrap();
        assert_eq!("5d854800-c062-4bb0-b905-fe721fb5ae14", &included_release.id);
        assert_eq!(1, included_release.release_events.unwrap().len());
    }

    #[test]
    fn is_official() {
        let s = r#"{
            "id": "5d854800-c062-4bb0-b905-fe721fb5ae14",
            "status-id": "4e304316-386d-3409-af2e-78857eec5cfe",
            "packaging-id": "119eba76-b343-3e02-a292-f0f00644bb9b",
            "disambiguation": "",
            "status": "Official",
            "text-representation": { "script": "Latn", "language": "eng" },
            "quality": "normal",
            "release-events": [
              {
                "date": "2022-09-30",
                "area": {
                  "name": "[Worldwide]",
                  "type-id": null,
                  "sort-name": "[Worldwide]",
                  "id": "525d4e18-3d00-31b9-a58b-a146a916de8f",
                  "disambiguation": "",
                  "iso-3166-1-codes": ["XW"],
                  "type": null
                }
              }
            ],
            "country": "XW",
            "title": "Viscera",
            "barcode": null,
            "date": "2022-09-30",
            "packaging": "None"
          }"#;

        let included_release: IncludedRelease = serde_json::from_str(s).unwrap();
        assert!(included_release.is_official());
    }
}
