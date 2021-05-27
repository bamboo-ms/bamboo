// Information required to uniquely identify the media
// in one or more databases

use uuid::Uuid;

pub enum Identifier {
    // MusicBrainz ID
    // Locally used to identify entities for music
    Mbid(Uuid),
    // https://wiki.musicbrainz.org/ISRC
    // International Standards Recording Code
    // See musicbrainz wiki for constraints
    // TODO: Create an ISRC type that enforces ISRC constraints
    Isrc(String),
}
