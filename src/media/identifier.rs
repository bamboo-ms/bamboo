use serde::{Deserialize, Serialize};

// A unique identifier for the media within a public Database
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Identifier {
    // A unique identifier for the database, for example imdb or tvdb
    pub r#type: String,
    // The identifier
    pub value: String,
}
