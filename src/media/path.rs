use serde::{Deserialize, Serialize};

// A path to a location of a file
#[derive(Serialize, Deserialize)]
pub enum Path {
    Local(String),
    Web(String),
}
