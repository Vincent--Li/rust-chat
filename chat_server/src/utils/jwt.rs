use std::ops::Deref;

use jwt_simple::prelude::*;

use crate::{AppError, User};

const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
const JWT_ISS: &str = "chat_server";
const JWT_AUD: &str = "chat_web";

pub struct EncodingKey(Ed25519KeyPair);
pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    pub fn load_pem(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }

    pub fn sign_token(&self, user: impl Into<User>) -> Result<String, AppError> {
        let claims = Claims::with_custom_claims(user.into(), Duration::from_secs(JWT_DURATION));
        let claims = claims.with_issuer(JWT_ISS).with_audience(JWT_AUD);
        Ok(self.sign(claims)?)
    }
}

impl DecodingKey {
    pub fn load_pem(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    #[allow(unused)]
    pub fn verify(&self, token: &str) -> Result<User, AppError> {
        let mut opts = VerificationOptions::default();
        opts.allowed_issuers = Some(HashSet::from_strings(&[JWT_ISS]));
        opts.allowed_audiences = Some(HashSet::from_strings(&[JWT_AUD]));
        let claims = self.verify_token::<User>(token, Some(opts))?;

        Ok(claims.custom)
    }
}

impl Deref for EncodingKey {
    type Target = Ed25519KeyPair;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for DecodingKey {
    type Target = Ed25519PublicKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn jwt_sign_verify_should_work() -> Result<()> {
        let ek = EncodingKey::load_pem(include_str!("../../fixtures/encoding.pem"))?;
        let dk = DecodingKey::load_pem(include_str!("../../fixtures/decoding.pem"))?;

        let user = User::new(1, "Vincent", "vincent@gmail.com");
        let token = ek.sign_token(user)?;
        let _ = dk.verify(token.as_str());

        Ok(())
    }
}
