use std::sync::Arc;

use tokio::sync::Mutex;
use tune_sage::api::{
    artists::{ArtistApi, ArtistIncludeRelation, ArtistQuery, ArtistSearchBuilder},
    cache::FileSystemCache,
    Config, HttpRemote,
};

#[tokio::main]
pub async fn main() {
    let artist_ids: Vec<&str> = vec![
        "ce8a3258-1863-469c-a832-37d22e7af624",
        "a466c2a2-6517-42fb-a160-1087c3bafd9f",
        "bf94f1fb-7122-4131-b7df-f26210ad8ec3",
        "1d097d38-d5ca-4cd4-9200-7f08eedd0875",
        "f59c5520-5f46-4d2c-b2c4-822eabf53419",
        "eb7c29b4-6951-4ac6-8516-4374fb51e6bc",
    ];

    let http_remote = HttpRemote;
    let config = Config {
        base_url: "https://musicbrainz.org/ws/2".to_string(),
        user_agent: "TuneSage <https://github.com/SpinningGoblin/tune_sage>".to_string(),
    };

    let cache = Arc::new(Mutex::new(FileSystemCache::for_folder("./derrick_garbage")));

    let mut remote = ArtistApi {
        config,
        remote: Arc::new(http_remote),
        cache,
    };

    for artist_id in artist_ids.iter() {
        for i in 0..=100 {
            let included_relations: Vec<ArtistIncludeRelation> = vec![
                ArtistIncludeRelation::Recordings,
                ArtistIncludeRelation::Releases,
            ];
            let fetched_artist = remote
                .by_id(artist_id, Some(included_relations))
                .await
                .unwrap();

            println!(
                "{} - {}",
                fetched_artist.releases.unwrap().get(0).unwrap().title,
                i
            );
        }
    }

    let search = ArtistSearchBuilder::new()
        .tag("death metal")
        .artist("death")
        .build()
        .unwrap();

    let artist_list = remote
        .query(&ArtistQuery::Search(Box::new(search)), None)
        .await
        .unwrap();

    for artist in artist_list.artists.iter() {
        println!("{}", artist.name);
    }

    println!("{:?} {:?}", artist_list.count, artist_list.artists.len());
}
