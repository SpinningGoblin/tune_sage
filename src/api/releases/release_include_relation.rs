pub enum ReleaseIncludeRelation {
    Artists,
    Recordings,
    ReleaseGroups,
    Labels,
}

impl ToString for ReleaseIncludeRelation {
    fn to_string(&self) -> String {
        match *self {
            ReleaseIncludeRelation::Recordings => "recordings".to_string(),
            ReleaseIncludeRelation::Artists => "artists".to_string(),
            ReleaseIncludeRelation::ReleaseGroups => "release-groups".to_string(),
            ReleaseIncludeRelation::Labels => "labels".to_string(),
        }
    }
}
