use crate::models::secret::Secret;
use crate::Passbolt;
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    id: String,
    name: String,
    username: String,
    uri: String,
    deleted: bool,
    resource_type_id: String,
    #[serde(rename = "created")]
    created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    modified_at: DateTime<Local>,
    created_by: String,
    modified_by: String,
}

impl Resource {
    /// Creates a new instance of the Resource struct
    pub fn new(
        id: String,
        name: String,
        username: String,
        uri: String,
        deleted: bool,
        resource_type_id: String,
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
            resource_type_id,
            created_at,
            modified_at,
            created_by,
            modified_by,
        }
    }

    /// Returns the secret associated with the resource
    pub async fn get_secret(&self, passbolt: &Passbolt) -> Result<Secret> {
        Ok(passbolt.get_secret(self.id.as_str()).await?)
    }
}
