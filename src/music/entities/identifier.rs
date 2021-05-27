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
    // TODO: Create an Isrc type that enforces ISRC constraints
    Isrc(String),
    // https://en.wikipedia.org/wiki/International_Standard_Musical_Work_Code
    // International Standard Musical Work Code
    // TODO: Create an Iswc type that enforced Iswc constraints
    Iswc(String),
}
