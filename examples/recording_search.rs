use std::sync::Arc;

use tokio::sync::Mutex;
use tune_sage::api::{
    cache::FileSystemCache,
    recordings::{RecordingApi, RecordingQuery, RecordingSearchBuilder},
    Config, HttpRemote,
};

#[tokio::main]
pub async fn main() {
    let config = Config {
        base_url: "https://musicbrainz.org/ws/2".to_string(),
        user_agent: "TuneSage <https://github.com/SpinningGoblin/tune_sage>".to_string(),
    };

    let cache = Arc::new(Mutex::new(FileSystemCache::for_folder("./derrick_garbage")));
    let http_remote = Arc::new(HttpRemote);

    let mut remote = RecordingApi {
        config: config.clone(),
        remote: http_remote.clone(),
        cache: cache.clone(),
    };

    let search = RecordingSearchBuilder::new()
        .recording_accent("Adderall")
        .artist("Slipknot")
        .build()
        .unwrap();

    let recording_list = remote
        .query(&RecordingQuery::Search(Box::new(search)), None)
        .await
        .unwrap();

    for recording in recording_list.recordings_scored_above(100) {
        println!(
            "recording score/title {:?}/{}",
            recording.score, recording.title
        );

        for release in recording.official_releases().iter() {
            println!(
                "release status/title {:?}/{}",
                release.status, release.title
            );
        }
    }

    println!("Example complete");
}
