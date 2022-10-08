pub enum ArtistIncludeRelation {
    Recordings,
    Releases,
    ReleaseGroups,
    Works,
}

impl ToString for ArtistIncludeRelation {
    fn to_string(&self) -> String {
        match *self {
            ArtistIncludeRelation::Recordings => "recordings".to_string(),
            ArtistIncludeRelation::Releases => "releases".to_string(),
            ArtistIncludeRelation::ReleaseGroups => "release-groups".to_string(),
            ArtistIncludeRelation::Works => "works".to_string(),
        }
    }
}
