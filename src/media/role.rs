use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Role {
    Artist,
    Producer,
}
