# passbolt-api-rs

This is a tool to interact with [Passbolt API](https://help.passbolt.com/api) using pure Rust.

## Features
* Basic functions
  * [X] Authentication
  * [X] PGP message decryption
  * [X] GET and POST methods
  * [ ] PUT, DELETE methods
* Advanced functions
  * [ ] Types
    * [ ] Users
    * [ ] Groups
    * [ ] Permissions
    * [ ] Resources
    * [ ] Secrets
    * [ ] Folders
    * [ ] Actions
    * [ ] Comments
* [ ] Configuration file
* [ ] CLI

## Usage
```rust
use passbolt_api_rs::gpg::{decrypt_message, ArmoredKey};
use passbolt_api_rs::Passbolt;
use pgp::SignedSecretKey;

use anyhow::Result;
use passbolt_api_rs::urls::{RESOURCE_URL, SECRET_URL};
use passbolt_api_rs::util::format;

const PRIVATE_KEY: &str = "
-----BEGIN PGP PRIVATE KEY BLOCK-----
Version: OpenPGP.js v4.10.9
Comment: https://openpgpjs.org

YOUR PRIVATE KEY
-----END PGP PRIVATE KEY BLOCK-----";

const PRIVATE_KEY_PW: &str = "PASSWORD FOR YOUR PRIVATE KEY";

#[tokio::main]
async fn main() -> Result<()> {
    let mut passbolt = Passbolt::new(
      "<URL of your Passbolt instance>".to_string(),
      SignedSecretKey::from_asc(PRIVATE_KEY).unwrap().0,
      PRIVATE_KEY_PW.to_string(),
    )
    .await?;
  
    let res_id = "<resource id>";
  
    let resource = passbolt
            .get(format(RESOURCE_URL, &[res_id]).as_str())
            .await?
            .1["body"]
            .to_string();
  
    let secret = passbolt
            .get(format(SECRET_URL, &[res_id]).as_str())
            .await?
            .1["body"]["data"]
            .to_string();
  
    println!(
      "{}\n{}",
      resource,
      decrypt_message(passbolt.private_key(), passbolt.private_key_pw(), secret)?
    );
  
    Ok(())
}
```
## [Known issues](https://github.com/M0dEx/passbolt-api-rs/issues)
