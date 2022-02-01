use anyhow::Result;
use serde_json::Value;

pub mod resource;
pub mod user;

pub trait SerdeJSON {
    type Model;

    /// Serializes the model into a Serde JSON Value
    fn serialize(model: &Self::Model) -> Result<serde_json::Value>;

    /// Deserializes the Serde JSON Value into a model
    fn deserialize(json: &Value) -> Result<Self::Model>;
}
