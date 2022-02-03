# passbolt-api-rs

This is a tool to interact with [Passbolt API](https://help.passbolt.com/api) using pure Rust.

## Features
* Basic functions
  * [X] Authentication
  * [X] PGP message decryption
  * [X] GET and POST methods
  * [X] PUT, DELETE methods
* Advanced functions
  * [ ] Types
    * [X] Users
    * [ ] Groups
    * [ ] Permissions
    * [X] Resources
    * [X] Secrets
    * [ ] Folders
    * [ ] Actions
    * [ ] Comments
* [ ] Configuration file
* [ ] CLI

## Usage
```rust
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
    let user_id = "<user id>";
  
    /// "Raw" methods to interact in case a model does not exist/does not support wanted functionality
    let resource_json = passbolt
            .get(format(RESOURCE_URL, &[res_id]).as_str())
            .await?
            .1["body"]
            .to_string();
  
    /// Native Rust structs representing the objects and common interaction with them
    let resource: Resource = passbolt
            .get_resource(res_id)
            .await?;
  
    let secret: Secret = resource
            .get_secret(&passbolt)
            .await?;
  
    let user: User = passbolt
            .get_user(user_id)
            .await?;
  
    println!("{:?}", resource);
    println!("{:?}", secret.decrypt_data(&passbolt)?);
  
    Ok(())
}
```
## [Known issues](https://github.com/M0dEx/passbolt-api-rs/issues)

## Disclaimer
This is a community driven project and it is not associated with Passbolt SA.
