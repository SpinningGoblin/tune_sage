pub enum RecordingIncludeRelation {
    Releases,
    Artists,
    ReleaseGroups,
}

impl ToString for RecordingIncludeRelation {
    fn to_string(&self) -> String {
        match *self {
            RecordingIncludeRelation::Releases => "releases".to_string(),
            RecordingIncludeRelation::Artists => "artists".to_string(),
            RecordingIncludeRelation::ReleaseGroups => "release-groups".to_string(),
        }
    }
}
