# Tune Sage
This a pretty thin wrapper around the MusicBrainz JSON API. It exposes the structures from their API without too much abstraction.

You can setup a cache for it to use when calling out to MusicBrainz where the library will first check for the values in the cache before making the call. File system and in memory caches are implemented in the crate.
