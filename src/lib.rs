use anyhow::{Error, Result};
use serde_json::{json, Value};

use crate::gpg::decrypt_message;
use crate::urls::{LOGIN_URL, ME_URL};
use pgp::types::{KeyTrait, SecretKeyTrait};
use pgp::SignedSecretKey;
use reqwest::header::HeaderMap;
use reqwest::{Client, ClientBuilder, Response};

pub mod gpg;
pub mod urls;
pub mod util;

pub struct Passbolt {
    url: String,
    private_key: SignedSecretKey,
    private_key_pw: String,
    client: Client,
    headers: HeaderMap,
}

impl Passbolt {
    pub async fn new(
        url: String,
        private_key: SignedSecretKey,
        private_key_pw: String,
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

        let mut result = match result.authenticate().await? {
            true => Ok(result),
            false => Err(Error::msg("Could not authenticate")),
        }?;

        // Get the CSRF token, since successful authentication does not provide one
        result.get(ME_URL).await?;

        Ok(result)
    }

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
                    "user_token_result": token
                }
            }),
        )
        .await?;

        Ok(true)
    }

    fn save_csrf_token(&mut self, response: &Response) -> Result<()> {
        for cookie in response.cookies() {
            if cookie.name() == "csrfToken" {
                self.headers.insert("X-CSRF-Token", cookie.value().parse()?);
            }
        }

        Ok(())
    }

    pub async fn get(&mut self, url: &str) -> Result<(HeaderMap, Value)> {
        let mut complete_url = self.url.clone();
        complete_url.push_str(url);

        let response = self
            .client
            .get(complete_url)
            .headers(self.headers.clone())
            .send()
            .await?;

        self.save_csrf_token(&response)?;

        Ok((
            response.headers().clone(),
            serde_json::from_str(response.text_with_charset("utf-8").await?.as_str())?,
        ))
    }

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
}
