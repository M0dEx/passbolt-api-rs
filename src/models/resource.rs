use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    id: String,
    name: String,
    username: String,
    uri: String,
    deleted: bool,
    #[serde(rename(serialize = "created", deserialize = "created"))]
    created_at: DateTime<Local>,
    #[serde(rename(serialize = "modified", deserialize = "modified"))]
    modified_at: DateTime<Local>,
    created_by: String,
    modified_by: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Secret {
    id: String,
    user_id: String,
    data: String,
    #[serde(rename(serialize = "created", deserialize = "created"))]
    created_at: DateTime<Local>,
    #[serde(rename(serialize = "modified", deserialize = "modified"))]
    modified_at: DateTime<Local>,
}

impl Resource {
    /// Creates a new instance of the Resource struct
    pub fn new(
        id: String,
        name: String,
        username: String,
        uri: String,
        deleted: bool,
        created_at: DateTime<Local>,
        modified_at: DateTime<Local>,
        created_by: String,
        modified_by: String,
    ) -> Self {
        Resource {
            id,
            name,
            username,
            uri,
            deleted,
            created_at,
            modified_at,
            created_by,
            modified_by,
        }
    }
}

impl Secret {
    /// Creates a new instance of the Secret struct
    pub fn new(
        id: String,
        user_id: String,
        data: String,
        created_at: DateTime<Local>,
        modified_at: DateTime<Local>,
    ) -> Self {
        Secret {
            id,
            user_id,
            data,
            created_at,
            modified_at,
        }
    }
}
