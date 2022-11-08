use std::sync::Arc;

use tokio::sync::Mutex;
use tune_sage::api::{
    artists::ArtistApi,
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

    let mut artist_api = ArtistApi {
        config: config.clone(),
        remote: http_remote.clone(),
        cache: cache.clone(),
    };

    for recording in recording_list.recordings.iter() {
        println!("{}", recording.title);

        if let Some(releases) = &recording.releases {
            for release in releases.iter() {
                println!("{}", release.title);
            }
        }

        if let Some(artist_credits) = &recording.artist_credit {
            let artist_credit = artist_credits.get(0).unwrap();
            let artist = artist_api
                .by_id(&artist_credit.artist.as_ref().unwrap().id, None)
                .await
                .unwrap();
            println!("{}", artist.name);
        }
    }

    println!(
        "{:?} {:?}",
        recording_list.count,
        recording_list.recordings.len()
    );

    println!("Example complete");
}
