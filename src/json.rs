use anyhow::{Error, Result};
use chrono::{DateTime, Local};
use serde_json::Value;

/// Parses a string from the given Serde Value
pub fn parse_json_string(value: &Value) -> Result<String> {
    Ok(value
        .as_str()
        .ok_or(Error::msg("Could not parse a field from the given JSON"))?
        .to_string())
}

/// Parses a datetime in RFC3339 format converted to the local timezone from the given Serde Value
pub fn parse_json_datetime(value: &Value) -> Result<DateTime<Local>> {
    let str = parse_json_string(value)?;

    Ok(DateTime::parse_from_rfc3339(str.as_str())?.with_timezone(&Local))
}

/// Parses a boolean from the given Serde Value
pub fn parse_json_bool(value: &Value) -> Result<bool> {
    Ok(value
        .as_bool()
        .ok_or(Error::msg("Could not parse a field from the given JSON"))?)
}
