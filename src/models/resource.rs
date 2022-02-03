use crate::models::action::Action;
use crate::models::secret::Secret;
use crate::models::user::User;
use crate::urls::ACTION_URL;
use crate::util::format;
use crate::Passbolt;
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

const PAGE_LIMIT: u32 = 100;

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt resource
pub struct Resource {
    pub id: String,
    pub name: String,
    pub username: String,
    pub uri: String,
    pub deleted: bool,
    pub resource_type_id: String,
    #[serde(rename = "created")]
    pub created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    pub modified_at: DateTime<Local>,
    pub created_by: String,
    pub modified_by: String,
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
    pub async fn secret(&self, passbolt: &Passbolt) -> Result<Secret> {
        Ok(passbolt.get_secret(self.id.as_str()).await?)
    }

    /// Returns the complete history of the resource
    pub async fn history(&self, passbolt: &Passbolt) -> Result<Vec<Action>> {
        let mut history: Vec<Action> = Vec::new();
        let mut page: u32 = 1;

        loop {
            let mut history_page: Vec<Action> = serde_json::from_value(
                passbolt
                    .get(
                        format(
                            ACTION_URL,
                            &[
                                self.id.as_str(),
                                page.to_string().as_str(),
                                PAGE_LIMIT.to_string().as_str(),
                            ],
                        )
                        .as_str(),
                    )
                    .await?
                    .1["body"]
                    .clone(),
            )?;

            if history_page.is_empty() {
                break;
            }

            history.append(&mut history_page);

            page += 1;
        }

        Ok(history)
    }

    /// Returns all actions recorded for this resource created by the specified user
    pub async fn history_user(&self, passbolt: &Passbolt, user_id: &String) -> Result<Vec<Action>> {
        let history = self.history(passbolt).await?;

        Ok(history
            .into_iter()
            .filter(|action| action.creator_id == *user_id)
            .collect())
    }
}
