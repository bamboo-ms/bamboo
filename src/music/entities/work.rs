// a work is a distinct intellectual or artistic creation, which can be
// expressed in the form of one or more audio recordings.
// reference: https://wiki.musicbrainz.org/Work

use super::recording::Recording;
use crate::media::title::Title;

// https://github.com/metabrainz/musicbrainz-server/blob/master/t/sql/initial.sql
// I believe this is the list of all valid work types, but
// TODO: we should confirm that this is true
enum Type {
    Aria,
    Ballet,
    Cantata,
    Concerto,
    Sonata,
    Suite,
    Madrigal,
    Mass,
    Motet,
    Opera,
    Oratorio,
    Overture,
    Partita,
    Quartet,
    SongCycle,
    Symphony,
    Song,
    SymphonicPoem,
    Zarzuela,
    Etude,
    Poem,
    Soundtrack,
    Prose,
    Operetta,
    AudioDrama,
    BeijingOpera,
    Play,
    Musical,
    IncidentalMusic,
}

enum WorkRelationship {
    PartOfWork(Work),
    Derivative(Work),
}

struct Work {
    title: Option<Title>,
    r#type: Type,
    identifiers: Vec<Identifier>,
    recordings: Vec<Recording>,

    // Comment to clarify which one of an entity with an identical
    // name as another this one is
    // reference: https://wiki.musicbrainz.org/Disambiguation_Comment
    disambiguation: Option<String>,

    // Any additional free-form information about the work
    // reference: https://wiki.musicbrainz.org/Annotation
    annotation: Option<String>,
}
