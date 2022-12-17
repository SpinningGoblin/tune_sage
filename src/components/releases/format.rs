use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Format {
    #[serde(
        alias = "Compact Disc",
        alias = "CompactDisc",
        alias = "compact disc",
        alias = "compactdisc"
    )]
    CompactDisc,
    #[serde(alias = "vinyl", alias = "Vinyl")]
    Vinyl,
    #[serde(
        alias = "Digital Media",
        alias = "digital media",
        alias = "DigitalMedia",
        alias = "digitalmedia"
    )]
    DigitalMedia,
    #[serde(alias = "cassette", alias = "Cassette")]
    Cassette,
    #[serde(alias = "dvd", alias = "DVD")]
    DVD,
    SACD,
    DualDisc,
    MiniDisc,
    #[serde(alias = "Blu-Ray", alias = "Blu Ray", alias = "BluRay")]
    BluRay,
    #[serde(alias = "other", alias = "Other")]
    Other,
}
