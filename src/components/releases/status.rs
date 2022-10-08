use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum Status {
    #[serde(alias = "official", alias = "Official")]
    Official,
    #[serde(alias = "promotion", alias = "Promotion")]
    Promotion,
    #[serde(alias = "bootleg", alias = "Bootleg")]
    Bootleg,
    #[serde(rename(deserialize = "pseudo-release"), alias = "Pseudo-Release")]
    PseudoRelease,
    #[serde(alias = "withdrawn", alias = "Withdrawn")]
    Withdrawn,
    #[serde(alias = "cancelled", alias = "Cancelled")]
    Cancelled,
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::Status;

    #[derive(Deserialize)]
    struct TestStatus {
        pub status: Status,
    }

    #[test]
    fn deserialize_pseudo() {
        let s = "{\"status\":\"pseudo-release\"}";
        let test_status: TestStatus = serde_json::from_str(s).unwrap();
        assert_eq!(Status::PseudoRelease, test_status.status);
    }

    #[test]
    fn deserialize_official() {
        let s = "{\"status\":\"official\"}";
        let test_status: TestStatus = serde_json::from_str(s).unwrap();
        assert_eq!(Status::Official, test_status.status);
    }
}
