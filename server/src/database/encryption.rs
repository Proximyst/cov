use aws_lc_rs::aead::{AES_256_GCM, Aad, RandomizedNonceKey};
use secrecy::{CloneableSecret, ExposeSecret, SecretBox};
use sqlx::{Database, Decode, Encode, Postgres, Type};
use std::{fmt::Debug, str::FromStr};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// The key used for encryption. This stores a private secret, which will be zeroized on drop.
///
/// This is safe to clone. The inner value is small and cheap to clone.
/// Upon drop, the inner value is entirely cleaned from memory.
#[derive(Debug, Clone)]
pub struct Key(SecretBox<KeyEncryptionKey>);

impl Key {
    /// Encrypt the given value in place. This will append the authentication tag to the value.
    ///
    /// The nonce used for encryption is returned. This must be stored alongside the encrypted value for decryption.
    pub fn encrypt<'a, V>(&self, value: &'a mut V) -> Result<Nonce, Error>
    where
        V: AsMut<[u8]> + for<'b> Extend<&'b u8>,
    {
        let key = RandomizedNonceKey::new(&AES_256_GCM, &self.0.expose_secret().0[..])?;
        let nonce = key.seal_in_place_append_tag(Aad::empty(), value)?;
        Ok(Nonce(nonce))
    }

    /// Decrypt the given value in place. The returned slice is the decrypted value, and may be shorter than the input slice.
    /// That means while this does not create a new memory region, you still need to change which slice you refer to.
    ///
    /// The nonce should be stored alongside the encrypted value.
    pub fn decrypt<'a>(&self, nonce: Nonce, value: &'a mut [u8]) -> Result<&'a mut [u8], Error> {
        let key = RandomizedNonceKey::new(&AES_256_GCM, &self.0.expose_secret().0[..])?;
        Ok(key.open_in_place(nonce.0, Aad::empty(), value)?)
    }
}

/// Nonce contains a single-use value for encryption. This should be stored alongside the encrypted value.
/// Without this value, decryption is impossible.
///
/// This is supported by [`sqlx`].
pub struct Nonce(aws_lc_rs::aead::Nonce);

impl Debug for Nonce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Nonce").field(&self.0.as_ref()).finish()
    }
}

impl From<aws_lc_rs::aead::Nonce> for Nonce {
    fn from(value: aws_lc_rs::aead::Nonce) -> Self {
        Self(value)
    }
}

impl Type<Postgres> for Nonce {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        <Vec<u8> as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &<Postgres as Database>::TypeInfo) -> bool {
        <Vec<u8> as Type<Postgres>>::compatible(ty)
    }
}

impl<'q> Encode<'q, Postgres> for Nonce {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let bytes = Vec::from(&self.0.as_ref()[..]);
        <Vec<u8> as Encode<Postgres>>::encode(bytes, buf)
    }
}

impl<'r> Decode<'r, Postgres> for Nonce {
    fn decode(
        value: <Postgres as Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = <Vec<u8> as Decode<Postgres>>::decode(value)?;
        Ok(Self(
            aws_lc_rs::aead::Nonce::try_assume_unique_for_key(&bytes).map_err(|_| Error)?,
        ))
    }
}

/// An error with absolutely no details.
///
/// When this is returned, there is no way to save the operation from failing.
/// This is semantically equivalent to [`aws_lc_rs::error::Unspecified`].
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("cryptographic operation failed")]
pub struct Error;

impl From<aws_lc_rs::error::Unspecified> for Error {
    fn from(_: aws_lc_rs::error::Unspecified) -> Self {
        Self
    }
}

/// A KEK for AEAD encryption. This should be stored carefully.
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
struct KeyEncryptionKey([u8; 32]);

impl CloneableSecret for KeyEncryptionKey {}

/// Parsing a [`Key`] failed. This is consumed by [`clap`].
#[derive(Debug, thiserror::Error)]
pub enum KeyParseError {
    #[error("key must be 64 hex characters long")]
    InvalidLength,

    #[error("key contains non-hex characters")]
    NonHex,
}

/// The implementation is consumed by [`clap`].
impl FromStr for Key {
    type Err = KeyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 64 {
            return Err(KeyParseError::InvalidLength);
        }

        let mut key = [0u8; 32];
        for (idx, val) in (0..64)
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .enumerate()
        {
            key[idx] = val.map_err(|_| KeyParseError::NonHex)?;
        }

        Ok(Self(SecretBox::new(Box::new(KeyEncryptionKey(key)))))
    }
}
