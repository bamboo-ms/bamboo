use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tag {}

#[derive(Serialize, Deserialize)]
pub struct GetTagsResponse {
    tags: Vec<Tag>,
}

#[get("/tags")]
pub fn get_tags() -> Json<GetTagsResponse> {
    Json(GetTagsResponse { tags: vec![] })
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_get_tags() {
        use super::GetTagsResponse;
        use crate::rocket;
        use rocket::local::Client;
        use rocket_contrib::json::Json;

        let client = Client::new(rocket()).expect("Could not create Rocket instance.");
        let mut response = client.get("/v1/tags").dispatch();
        assert_eq!(response.body_string(), Some(r#"{"tags":[]}"#.into()));
    }
}
