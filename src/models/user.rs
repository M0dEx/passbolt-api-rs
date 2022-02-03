use crate::gpg::ArmoredKey;
use anyhow::Result;
use chrono::{DateTime, Local};
use pgp::SignedPublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt user
pub struct User {
    id: String,
    username: String,
    active: bool,
    deleted: bool,
    is_mfa_enabled: bool,
    last_logged_in: Option<DateTime<Local>>,
    #[serde(rename = "created")]
    created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    modified_at: DateTime<Local>,
    role: Role,
    profile: Profile,
    #[serde(rename = "gpgkey")]
    gpg_key: GPGKey,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt role
pub struct Role {
    id: String,
    name: String,
    description: String,
    #[serde(rename = "created")]
    created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    modified_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt user's profile
pub struct Profile {
    id: String,
    first_name: String,
    last_name: String,
    #[serde(rename = "created")]
    created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    modified_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt user's GPG key information
pub struct GPGKey {
    id: String,
    #[serde(rename = "type")]
    key_type: String,
    bits: u16,
    fingerprint: String,
    expires: Option<DateTime<Local>>,
    deleted: bool,
    armored_key: String,
    #[serde(rename = "created")]
    created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    modified_at: DateTime<Local>,
}

impl User {
    /// Returns the public key of the user
    pub fn get_public_key(&self) -> Result<SignedPublicKey> {
        Ok(SignedPublicKey::from_asc(&self.gpg_key.armored_key)?.0)
    }
}
