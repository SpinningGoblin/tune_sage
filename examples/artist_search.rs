use tune_sage::api::{
    artists::{ArtistApi, ArtistIncludeRelation, ArtistQuery, ArtistSearchBuilder},
    Config, HttpRemote,
};

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

    let included_relations: Vec<ArtistIncludeRelation> = vec![
        ArtistIncludeRelation::Recordings,
        ArtistIncludeRelation::Releases,
    ];

    let fetched_artist = remote
        .by_id(artist_id, Some(included_relations))
        .await
        .unwrap();

    println!("{}", fetched_artist.releases.unwrap().get(0).unwrap().title);

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
