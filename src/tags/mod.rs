use crate::media_type::MediaType;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Tag {
    id: Uuid;
    types: Vec<MediaType>,
}

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
    use rocket::local::LocalResponse;

    #[test]
    fn check_get_tags() {
        use crate::rocket;
        use rocket::local::Client;

        let client: Client = Client::new(rocket()).expect("Could not create Rocket client.");
        let mut response: LocalResponse = client.get("/v1/tags").dispatch();
        assert_eq!(response.body_string(), Some(r#"{"tags":[]}"#.into()));
    }
}
