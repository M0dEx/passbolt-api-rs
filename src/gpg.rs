use anyhow::{Context, Result};

use pgp::ser::Serialize;
use pgp::types::KeyTrait;
use pgp::{Deserializable, Message, SignedPublicKey, SignedSecretKey};
use std::collections::BTreeMap;
use std::io::Cursor;

pub trait ArmoredKey: Serialize + Deserializable + KeyTrait + Clone {
    type KeyType: Serialize + Deserializable + KeyTrait + Clone;

    fn from_asc(data: &str) -> Result<(Self::KeyType, BTreeMap<String, String>)> {
        let bytes = data.as_bytes();
        Self::KeyType::from_armor_single(Cursor::new(bytes)).context("rPGP error")
    }
}

impl ArmoredKey for SignedPublicKey {
    type KeyType = SignedPublicKey;
}

impl ArmoredKey for SignedSecretKey {
    type KeyType = SignedSecretKey;
}

fn decode_url_armor(encoded_armor: String) -> Result<Message> {
    let decoded = urlencoding::decode(encoded_armor.as_str())?
        .to_string()
        .replace("\\+", " ")
        .replace("\\r\\n", "\n")
        .replace("\\n", "\n");

    let (msg, _) = Message::from_armor_single(Cursor::new(decoded))?;

    Ok(msg)
}

pub fn decrypt_message(
    private_key: &SignedSecretKey,
    private_key_pw: &String,
    encrypted_msg: String,
) -> Result<String> {
    let msg = decode_url_armor(encrypted_msg)?;

    let (decrypter, _) = msg.decrypt(|| "".into(), || private_key_pw.clone(), &[private_key])?;

    let result = decrypter.collect::<pgp::errors::Result<Vec<_>>>()?;

    match result[0].get_content()? {
        Some(message) => Ok(String::from_utf8(message)?),
        None => Ok(String::new()),
    }
}
