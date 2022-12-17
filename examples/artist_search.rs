use std::sync::Arc;

use tokio::sync::Mutex;
use tune_sage::api::{
    artists::{ArtistApi, ArtistIncludeRelation, ArtistQuery, ArtistSearchBuilder},
    cache::FileSystemCache,
    Config, HttpRemote,
};

#[tokio::main]
pub async fn main() {
    let artist_ids: Vec<&str> = vec!["f3e6c393-01e7-483f-8532-9388f27ee724"];

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
        let included_relations: Vec<ArtistIncludeRelation> = vec![
            ArtistIncludeRelation::Recordings,
            ArtistIncludeRelation::Releases,
        ];
        let fetched_artist = remote
            .by_id(artist_id, Some(included_relations))
            .await
            .unwrap();

        println!("{}", fetched_artist.releases.unwrap().get(0).unwrap().title,);
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
