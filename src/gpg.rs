use anyhow::{Context, Result};

use std::collections::BTreeMap;
use std::io::Cursor;
use pgp::{Deserializable, Message, SignedPublicKey, SignedSecretKey};
use pgp::ser::Serialize;
use pgp::types::KeyTrait;

pub trait ArmoredKey: Serialize + Deserializable + KeyTrait + Clone {
    type KeyType: Serialize + Deserializable + KeyTrait + Clone;

    fn from_asc(data: &str) -> Result<(Self::KeyType, BTreeMap<String, String>)> {
        let bytes = data.as_bytes();
        Self::KeyType::from_armor_single(Cursor::new(bytes)).context("rPGP error")
    }
}

impl ArmoredKey for SignedPublicKey
{
    type KeyType = SignedPublicKey;
}

impl ArmoredKey for SignedSecretKey
{
    type KeyType = SignedSecretKey;
}

pub fn decrypt_message(private_key: &SignedSecretKey, encrypted_msg: Vec<u8>) -> Result<String>
{
    let (msg, _) = Message::from_armor_single(Cursor::new(encrypted_msg))?;

    let (decrypter, _) = msg.decrypt(|| "".into(), || "".into(), &[private_key])?;

    let result = decrypter.collect::<pgp::errors::Result<Vec<_>>>()?;

    match result[0].get_content()? {
        Some(message) => Ok(String::from_utf8(message)?),
        None => Ok(String::new())
    }
}