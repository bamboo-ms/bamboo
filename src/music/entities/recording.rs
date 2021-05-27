// "Generally, the audio represented by a recording corresponds
// to the audio at a stage in the production process before any
// final mastering but after any editing or mixing."
// reference: https://wiki.musicbrainz.org/Recording
//
use super::identifier::Identifier;
use chrono::Duration;

struct Recording {
    title: Option<String>,
    artists: Vec<bool>, // TODO: Use Artist type instead
    length: Option<Duration>,
    identifiers: Vec<Identifier>,

    // Comment to clarify which one of an entity with an identical
    // name as another this one is
    // reference: https://wiki.musicbrainz.org/Disambiguation_Comment
    disambiguation: Option<String>,

    // Any additional free-form information about the recording
    // reference: https://wiki.musicbrainz.org/Annotation
    annotation: Option<String>,
}
