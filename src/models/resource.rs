use crate::json::{parse_json_bool, parse_json_datetime, parse_json_string};
use crate::models::SerdeJSON;
use anyhow::Result;
use chrono::{DateTime, Local};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Resource {
    id: String,
    name: String,
    username: String,
    uri: String,
    deleted: bool,
    created_at: DateTime<Local>,
    modified_at: DateTime<Local>,
    created_by: String,
    modified_by: String,
}

#[derive(Debug, Clone)]
pub struct Secret {
    id: String,
    user_id: String,
    data: String,
    created_at: DateTime<Local>,
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

impl SerdeJSON for Resource {
    type Model = Resource;

    fn serialize(model: &Self::Model) -> Result<Value> {
        todo!()
    }

    fn deserialize(json: &Value) -> Result<Self::Model> {
        Ok(Self::Model {
            id: parse_json_string(&json["id"])?,
            name: parse_json_string(&json["name"])?,
            username: parse_json_string(&json["username"])?,
            uri: parse_json_string(&json["uri"])?,
            deleted: parse_json_bool(&json["deleted"])?,
            created_at: parse_json_datetime(&json["created"])?,
            modified_at: parse_json_datetime(&json["modified"])?,
            created_by: parse_json_string(&json["created_by"])?,
            modified_by: parse_json_string(&json["modified_by"])?,
        })
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

impl SerdeJSON for Secret {
    type Model = Secret;

    fn serialize(model: &Self::Model) -> anyhow::Result<Value> {
        todo!()
    }

    fn deserialize(json: &Value) -> anyhow::Result<Self::Model> {
        Ok(Self::Model {
            id: parse_json_string(&json["id"])?,
            user_id: parse_json_string(&json["user_id"])?,
            data: parse_json_string(&json["data"])?,
            created_at: parse_json_datetime(&json["created"])?,
            modified_at: parse_json_datetime(&json["modified"])?,
        })
    }
}
