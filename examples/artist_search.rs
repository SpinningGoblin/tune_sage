use tune_sage::api::{ArtistApi, ArtistQuery, ArtistSearchBuilder, Config, HttpRemote};

#[tokio::main]
pub async fn main() {
    let http_remote = HttpRemote;
    let artist_id = "ce8a3258-1863-469c-a832-37d22e7af624";
    let config = Config {
        base_url: "https://musicbrainz.org/ws/2".to_string(),
        user_agent: "TuneSage <https://github.com/derrickp/musicz>".to_string(),
    };

    let remote = ArtistApi {
        config,
        remote: Box::new(http_remote),
    };

    println!("{}", remote.by_id(artist_id).await.unwrap().name);

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
