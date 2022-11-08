use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PrimaryGroupType {
    #[serde(alias = "album", alias = "Album")]
    Album,
    #[serde(alias = "single", alias = "Single")]
    Single,
    #[serde(alias = "ep", alias = "EP")]
    EP,
    #[serde(alias = "broadcast", alias = "Broadcast")]
    Broadcast,
    #[serde(alias = "other", alias = "Other")]
    Other,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SecondaryGroupType {
    #[serde(alias = "compilation", alias = "Compilation")]
    Compilation,
    #[serde(alias = "soundtrack", alias = "Soundtrack")]
    Soundtrack,
    #[serde(alias = "spokenword", alias = "Spokenword")]
    SpokenWord,
    #[serde(alias = "interview", alias = "Interview")]
    Interview,
    #[serde(alias = "audiobook", alias = "Audiobook")]
    AudioBook,
    #[serde(
        alias = "audiodrama",
        alias = "AudioDrama",
        alias = "Audio Drama",
        alias = "audio-drama",
        alias = "Audio drama"
    )]
    AudioDrama,
    #[serde(alias = "live", alias = "Live")]
    Live,
    #[serde(alias = "remix", alias = "Remix")]
    Remix,
    #[serde(alias = "djmix", alias = "DJ-mix", alias = "dj-mix", alias = "DJ-Mix")]
    DjMix,
    #[serde(
        alias = "Mixtape/Street",
        alias = "mixtape",
        alias = "MixTape",
        alias = "street",
        alias = "mixtape-street"
    )]
    MixTapeStreet,
    #[serde(alias = "Demo", alias = "demo")]
    Demo,
}
