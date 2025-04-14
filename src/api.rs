use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

// Simple GraphQL request structure
#[derive(Serialize)]
struct GraphQLRequest {
    query: String,
    variables: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct SceneFile {
    pub size: Option<i64>,
    pub duration: Option<f64>,
    pub video_codec: Option<String>,
    #[serde(rename = "audioCodec")]
    pub audio_codec: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    #[serde(rename = "framerate")]
    pub frame_rate: Option<f64>,
    #[serde(rename = "bitrate")]
    pub bit_rate: Option<i32>,
}

// Response structures for Stash API
#[derive(Debug, Deserialize)]
pub struct SystemStatus {
    #[serde(rename = "databaseSchema")]
    pub database_schema: i32,
    pub status: String,
    pub os: String,
    #[serde(rename = "workingDir")]
    pub working_dir: String,
    #[serde(rename = "homeDir")]
    pub home_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct SystemStatusResponse {
    pub system_status: SystemStatus,
}

#[derive(Debug, Deserialize)]
pub struct Scene {
    pub id: String,
    pub title: Option<String>,
    pub code: Option<String>,
    pub details: Option<String>,
    pub director: Option<String>,
    pub url: Option<String>,
    pub urls: Option<Vec<String>>,
    pub date: Option<String>,
    #[serde(rename = "rating100")]
    pub rating: Option<i32>,
    pub organized: Option<bool>,
    pub o_counter: Option<i32>,
    pub interactive: Option<bool>,
    pub interactive_speed: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub last_played_at: Option<String>,
    pub resume_time: Option<f64>,
    pub play_duration: Option<f64>,
    pub play_count: Option<i32>,
    pub files: Option<Vec<SceneFile>>,
    pub scene_markers: Option<Vec<SceneMarker>>,
    #[serde(rename = "sceneStreams")]
pub scene_streams: Option<Vec<SceneStreamEndpoint>>,
}

#[derive(Debug, Deserialize)]
pub struct ScenesResponse {
    pub count: i32,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Deserialize)]
pub struct FindScenesResponse {
    pub find_scenes: ScenesResponse,
}

#[derive(Debug, Deserialize)]
pub struct Performer {
    pub id: String,
    pub name: String,
    pub gender: Option<String>,
    pub favorite: bool,
    pub scene_count: i32,
    pub image_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PerformersResponse {
    pub count: i32,
    pub performers: Vec<Performer>,
}

#[derive(Debug, Deserialize)]
pub struct FindPerformersResponse {
    pub find_performers: PerformersResponse,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub scene_count: i32,
    pub image_count: i32,
    pub performer_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct TagsResponse {
    pub count: i32,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct FindTagsResponse {
    pub find_tags: TagsResponse,
}

pub struct StashClient {
    client: Client,
    base_url: String,
}

impl StashClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_system_status(&self) -> Result<SystemStatus, Box<dyn Error>> {
        let query = r#"
        {
            systemStatus {
                databaseSchema
                status
                os
                workingDir
                homeDir
            }
        }
        "#;

        let request = GraphQLRequest {
            query: query.to_string(),
            variables: None,
        };

        let res = self.client
            .post(&format!("{}/graphql", self.base_url))
            .json(&request)
            .send()
            .await?;

        let response: serde_json::Value = res.json().await?;
        
        // Check for errors
        if let Some(errors) = response.get("errors") {
            if errors.is_array() && !errors.as_array().unwrap().is_empty() {
                return Err(format!("GraphQL errors: {:?}", errors).into());
            }
        }

        // Parse the response
        if let Some(data) = response.get("data") {
            if let Some(system_status) = data.get("systemStatus") {
                let status: SystemStatus = serde_json::from_value(system_status.clone())?;
                return Ok(status);
            }
        }

        Err("Failed to parse system status response".into())
    }

    pub async fn find_scenes(&self, page: i32, per_page: i32) -> Result<ScenesResponse, Box<dyn Error>> {
        let query = r#"
        query FindScenes($filter: FindFilterType) {
            findScenes(filter: $filter) {
                count
                scenes {
                    id
                    title
                    code
                    details
                    date
                    rating100
                    organized
                    o_counter
                    interactive
                    play_count
                    files {
                        size
                        duration
                        video_codec
                        audio_codec
                        width
                        height
                        frame_rate
                        bit_rate
                    }
                    sceneStreams {
                        url
                        mime_type
                        label
                    }
                }
            }
        }
        "#;

        let variables = serde_json::json!({
            "filter": {
                "page": page,
                "per_page": per_page
            }
        });

        let request = GraphQLRequest {
            query: query.to_string(),
            variables: Some(variables),
        };

        let res = self.client
            .post(&format!("{}/graphql", self.base_url))
            .json(&request)
            .send()
            .await?;

        let response: serde_json::Value = res.json().await?;
        
        // Check for errors
        if let Some(errors) = response.get("errors") {
            if errors.is_array() && !errors.as_array().unwrap().is_empty() {
                return Err(format!("GraphQL errors: {:?}", errors).into());
            }
        }

        // Parse the response
        if let Some(data) = response.get("data") {
            if let Some(find_scenes) = data.get("findScenes") {
                let scenes_response: ScenesResponse = serde_json::from_value(find_scenes.clone())?;
                return Ok(scenes_response);
            }
        }

        Err("Failed to parse scenes response".into())
    }

    pub async fn find_performers(&self, page: i32, per_page: i32) -> Result<PerformersResponse, Box<dyn Error>> {
        let query = r#"
        query FindPerformers($filter: FindFilterType) {
            findPerformers(filter: $filter) {
                count
                performers {
                    id
                    name
                    gender
                    favorite
                    scene_count
                    image_path
                }
            }
        }
        "#;

        let variables = serde_json::json!({
            "filter": {
                "page": page,
                "per_page": per_page
            }
        });

        let request = GraphQLRequest {
            query: query.to_string(),
            variables: Some(variables),
        };

        let res = self.client
            .post(&format!("{}/graphql", self.base_url))
            .json(&request)
            .send()
            .await?;

        let response: serde_json::Value = res.json().await?;
        
        // Check for errors
        if let Some(errors) = response.get("errors") {
            if errors.is_array() && !errors.as_array().unwrap().is_empty() {
                return Err(format!("GraphQL errors: {:?}", errors).into());
            }
        }

        // Parse the response
        if let Some(data) = response.get("data") {
            if let Some(find_performers) = data.get("findPerformers") {
                let performers_response: PerformersResponse = serde_json::from_value(find_performers.clone())?;
                return Ok(performers_response);
            }
        }

        Err("Failed to parse performers response".into())
    }

    pub async fn find_tags(&self, page: i32, per_page: i32) -> Result<TagsResponse, Box<dyn Error>> {
        let query = r#"
        query FindTags($filter: FindFilterType) {
            findTags(filter: $filter) {
                count
                tags {
                    id
                    name
                    scene_count
                    image_count
                    performer_count
                }
            }
        }
        "#;

        let variables = serde_json::json!({
            "filter": {
                "page": page,
                "per_page": per_page
            }
        });

        let request = GraphQLRequest {
            query: query.to_string(),
            variables: Some(variables),
        };

        let res = self.client
            .post(&format!("{}/graphql", self.base_url))
            .json(&request)
            .send()
            .await?;

        let response: serde_json::Value = res.json().await?;
        
        // Check for errors
        if let Some(errors) = response.get("errors") {
            if errors.is_array() && !errors.as_array().unwrap().is_empty() {
                return Err(format!("GraphQL errors: {:?}", errors).into());
            }
        }

        // Parse the response
        if let Some(data) = response.get("data") {
            if let Some(find_tags) = data.get("findTags") {
                let tags_response: TagsResponse = serde_json::from_value(find_tags.clone())?;
                return Ok(tags_response);
            }
        }

        Err("Failed to parse tags response".into())
    }
}
#[derive(Debug, Deserialize)]
pub struct SceneStreamEndpoint {
    pub url: String,
    pub mime_type: Option<String>,
    pub label: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct SceneMarker {
    pub id: String,
    pub title: String,
    pub seconds: f64,
}
