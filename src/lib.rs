use std::path::Path;
use anyhow::{Result, Error};
use serde_json::{Value, json};

use pgp::SignedSecretKey;
use pgp::types::{KeyTrait, SecretKeyTrait};
use reqwest::{Client, ClientBuilder, Response};
use reqwest::header::HeaderMap;
use crate::gpg::decrypt_message;
use crate::urls::LOGIN_URL;

pub mod gpg;
pub mod models;
pub mod urls;

pub struct Passbolt
{
    url: String,
    private_key: SignedSecretKey,
    client: Client,
    headers: HeaderMap,
}

impl Passbolt {
    pub async fn new(url: String, private_key: SignedSecretKey) -> Result<Self>
    {
        let mut result = Passbolt {
            url,
            private_key,
            client: ClientBuilder::new()
                .cookie_store(true)
                .build()?,
            headers: HeaderMap::new()
        };

        //result.headers.insert("User-Agent", "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:95.0) Gecko/20100101 Firefox/95.0".parse()?);
        result.headers.insert("Content-Type", "application/json".parse()?);

        match result.authenticate().await? {
            true => Ok(result),
            false => Err(Error::msg("Could not authenticate"))
        }
    }

    async fn authenticate(&mut self) -> Result<bool>
    {
        let fingerprint = self.private_key
            .public_key()
            .fingerprint()
            .iter()
            .map(|b| format!("{:02X}", *b))
            .collect::<Vec::<_>>()
            .join("");

        let auth_response = self.post(LOGIN_URL, json!({
            "gpg_auth": {
                "keyid": fingerprint
            }
        })).await?;

        let armored_token = urlencoding::decode(auth_response
            .0
            .get("X-GPGAuth-User-Auth-Token")
            .ok_or(Error::msg("Could not parse URL encoded Auth Token"))?
            .to_str()?)?
            .to_string()
            .replace("\\+", " ");

        // TODO: Fails here, due to

        let token = decrypt_message(&self.private_key, armored_token.into_bytes())?;

        self.post(LOGIN_URL, json!({
            "gpg_auth": {
                "keyid": fingerprint,
                "user_token_result": token
            }
        })).await?;

        Ok(true)
    }

    fn save_csrf_token(&mut self, response: &Response) -> Result<()>
    {
        for cookie in response.cookies()
        {
            if cookie.name() == "csrfToken" {
                self.headers.insert("X-CSRF-Token", cookie.value().parse()?);
            }
        }

        Ok(())
    }

    pub async fn get(&mut self, url: &str) -> Result<(HeaderMap, Value)>
    {
        let mut complete_url = self.url.clone();
        complete_url.push_str(url);

        let response = self.client
            .get(complete_url)
            .headers(self.headers.clone())
            .send()
            .await?;

        self.save_csrf_token(&response)?;

        Ok((response.headers().clone(),
            serde_json::from_str(response
                .text_with_charset("utf-8")
                .await?
                .as_str())?
        ))
    }

    pub async fn post(&mut self, url: &str, data: Value) -> Result<(HeaderMap, Value)>
    {
        let mut complete_url = self.url.clone();
        complete_url.push_str(url);

        let response = self.client
            .post(complete_url)
            .headers(self.headers.clone())
            .body(data.to_string())
            .send()
            .await?;

        self.save_csrf_token(&response)?;

        Ok((response.headers().clone(),
            serde_json::from_str(response
                .text_with_charset("utf-8")
                .await?
                .as_str())?
        ))
    }
}
