use std::sync::Arc;

use tokio::sync::Mutex;
use tune_sage::api::{
    cache::FileSystemCache,
    releases::{ReleaseApi, ReleaseIncludeRelation},
    Config, HttpRemote,
};

#[tokio::main]
pub async fn main() {
    let release_ids: Vec<&str> = vec!["d3038e41-12d0-46ed-aa28-dcfc92268232"];

    let http_remote = HttpRemote;
    let config = Config {
        base_url: "https://musicbrainz.org/ws/2".to_string(),
        user_agent: "TuneSage <https://github.com/SpinningGoblin/tune_sage>".to_string(),
    };

    let cache = Arc::new(Mutex::new(FileSystemCache::for_folder("./derrick_garbage")));

    let mut remote = ReleaseApi {
        config,
        remote: Arc::new(http_remote),
        cache,
    };

    for release_id in release_ids.iter() {
        let included_relations: Vec<ReleaseIncludeRelation> =
            vec![ReleaseIncludeRelation::Recordings];
        let fetched_release = remote
            .by_id(release_id, Some(included_relations))
            .await
            .unwrap();

        println!(
            "{:?}",
            fetched_release
                .media
                .unwrap()
                .get(0)
                .unwrap()
                .tracks
                .as_ref()
                .unwrap()
                .get(0)
                .unwrap()
                .id
        );
    }
}
