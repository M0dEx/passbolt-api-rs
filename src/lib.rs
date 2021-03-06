use anyhow::{Error, Result};
use serde_json::{json, Value};

use crate::gpg::decrypt_message;
use crate::models::resource::Resource;
use crate::models::secret::Secret;
use crate::models::user::User;
use crate::urls::{LOGIN_URL, ME_URL, RESOURCE_URL, SECRET_URL, USER_URL};
use crate::util::format;
use pgp::types::{KeyTrait, SecretKeyTrait};
use pgp::SignedSecretKey;
use reqwest::header::HeaderMap;
use reqwest::{Client, ClientBuilder, Response};
use secstr::SecUtf8;

pub mod gpg;
pub mod models;
pub mod urls;
pub mod util;

/// Struct representing a session used to access the Passbolt instance
pub struct Passbolt {
    url: String,
    pub private_key: SignedSecretKey,
    pub private_key_pw: SecUtf8,
    client: Client,
    headers: HeaderMap,
}

impl Passbolt {
    /// Creates a new instance of the Passbolt struct
    pub async fn new(
        url: String,
        private_key: SignedSecretKey,
        private_key_pw: SecUtf8,
    ) -> Result<Self> {
        let mut result = Passbolt {
            url,
            private_key,
            private_key_pw,
            client: ClientBuilder::new().cookie_store(true).build()?,
            headers: HeaderMap::new(),
        };

        //result.headers.insert("User-Agent", "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:95.0) Gecko/20100101 Firefox/95.0".parse()?);
        result
            .headers
            .insert("Content-Type", "application/json".parse()?);

        let result = match result.authenticate().await? {
            true => Ok(result),
            false => Err(Error::msg("Could not authenticate")),
        }?;

        // Get the CSRF token, since successful authentication does not provide one
        result.get(ME_URL).await?;

        Ok(result)
    }

    /// Authenticates using the given private key
    async fn authenticate(&mut self) -> Result<bool> {
        let fingerprint = self
            .private_key
            .public_key()
            .fingerprint()
            .iter()
            .map(|b| format!("{:02X}", *b))
            .collect::<Vec<_>>()
            .join("");

        let auth_response = self
            .post(
                LOGIN_URL,
                json!({
                    "gpg_auth": {
                        "keyid": fingerprint
                    }
                }),
            )
            .await?;

        let token = decrypt_message(
            &self.private_key,
            &self.private_key_pw,
            auth_response
                .0
                .get("X-GPGAuth-User-Auth-Token")
                .ok_or(Error::msg("Could not parse URL encoded Auth Token"))?
                .to_str()?
                .to_string(),
        )?;

        self.post(
            LOGIN_URL,
            json!({
                "gpg_auth": {
                    "keyid": fingerprint,
                    "user_token_result": token.unsecure()
                }
            }),
        )
        .await?;

        Ok(true)
    }

    /// Fetches the CSRF token from the cookie store and puts in into the X-CSRF-Token header
    fn save_csrf_token(&mut self, response: &Response) -> Result<()> {
        for cookie in response.cookies() {
            if cookie.name() == "csrfToken" {
                println!("CSRF Token: {}", cookie.value().to_string());
                self.headers.insert("X-CSRF-Token", cookie.value().parse()?);
            }
        }

        Ok(())
    }

    /// Connects to the server using the GET method and returns the response and it's headers
    pub async fn get(&self, url: &str) -> Result<(HeaderMap, Value)> {
        let mut complete_url = self.url.clone();
        complete_url.push_str(url);

        let response = self
            .client
            .get(complete_url)
            .headers(self.headers.clone())
            .send()
            .await?;

        Ok((
            response.headers().clone(),
            serde_json::from_str(response.text_with_charset("utf-8").await?.as_str())?,
        ))
    }

    /// Connects to the server using the DELETE method and returns the response and it's headers
    pub async fn delete(&mut self, url: &str) -> Result<(HeaderMap, Value)> {
        let mut complete_url = self.url.clone();
        complete_url.push_str(url);

        let response = self
            .client
            .delete(complete_url)
            .headers(self.headers.clone())
            .send()
            .await?;

        self.save_csrf_token(&response)?;

        Ok((
            response.headers().clone(),
            serde_json::from_str(response.text_with_charset("utf-8").await?.as_str())?,
        ))
    }

    /// Connects to the server using the POST method, sends the given data and returns the response and it's headers
    pub async fn post(&mut self, url: &str, data: Value) -> Result<(HeaderMap, Value)> {
        let mut complete_url = self.url.clone();
        complete_url.push_str(url);

        let response = self
            .client
            .post(complete_url)
            .headers(self.headers.clone())
            .body(data.to_string())
            .send()
            .await?;

        self.save_csrf_token(&response)?;

        Ok((
            response.headers().clone(),
            serde_json::from_str(response.text_with_charset("utf-8").await?.as_str())?,
        ))
    }

    /// Connects to the server using the PUT method, sends the given data and returns the response and it's headers
    pub async fn put(&mut self, url: &str, data: Value) -> Result<(HeaderMap, Value)> {
        let mut complete_url = self.url.clone();
        complete_url.push_str(url);

        let response = self
            .client
            .put(complete_url)
            .headers(self.headers.clone())
            .body(data.to_string())
            .send()
            .await?;

        self.save_csrf_token(&response)?;

        Ok((
            response.headers().clone(),
            serde_json::from_str(response.text_with_charset("utf-8").await?.as_str())?,
        ))
    }

    /// Returns the resource specified by it's ID
    pub async fn get_resource(&self, resource_id: &str) -> Result<Resource> {
        Ok(serde_json::from_value(
            self.get(format(RESOURCE_URL, &[resource_id]).as_str())
                .await?
                .1["body"]
                .clone(),
        )?)
    }

    /// Returns the secret specified by it's resource's ID
    pub async fn get_secret(&self, resource_id: &str) -> Result<Secret> {
        Ok(serde_json::from_value(
            self.get(format(SECRET_URL, &[resource_id]).as_str())
                .await?
                .1["body"]
                .clone(),
        )?)
    }

    /// Returns the user specified by their ID
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        Ok(serde_json::from_value(
            self.get(format(USER_URL, &[user_id]).as_str()).await?.1["body"].clone(),
        )?)
    }
}
