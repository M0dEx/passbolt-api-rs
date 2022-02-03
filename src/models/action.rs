use chrono::{DateTime, Local};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt action (read, edit, ...)
pub struct Action {
    id: String,
    action_log_id: String,
    #[serde(rename = "type")]
    action_type: String,
    #[serde(rename = "created")]
    created_at: DateTime<Local>,
    #[serde(rename = "creator", deserialize_with = "deserialize_creator")]
    creator_id: String,
}

/// Deserializes the creator id from it's JSON object
fn deserialize_creator<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let creator_struct = Value::deserialize(d)?;

    Ok(creator_struct["id"]
        .as_str()
        .ok_or(D::Error::custom(
            "Could not parse creator ID from the Action sub-struct",
        ))?
        .to_string())
}
