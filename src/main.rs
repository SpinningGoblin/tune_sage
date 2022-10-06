use tune_sage::api::{ArtistApi, ArtistQuery, Config, HttpRemote};

#[tokio::main]
pub async fn main() {
    let http_remote = HttpRemote;
    let _artist_id = "ce8a3258-1863-469c-a832-37d22e7af624";
    let config = Config {
        base_url: "https://musicbrainz.org/ws/2".to_string(),
        user_agent: "TuneSage <https://github.com/derrickp/musicz>".to_string(),
    };

    let remote = ArtistApi {
        config,
        remote: Box::new(http_remote),
    };

    let artist_list = remote
        .query(&ArtistQuery::Term("Slipknot".to_string()))
        .await
        .unwrap();
    println!("{:?}", artist_list);
}
