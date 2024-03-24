//use crate::error::{AuthenticationError, Error, RSAError, SerdeError};
use aead::OsRng;
use email_address::EmailAddress;
use rsa::Pkcs1v15Encrypt;
use rsa::RsaPublicKey;
use serde::{Deserialize, Serialize};

use crate::datetime::r#trait::Expired;

use super::error::Error;
use super::error::RSAError;
use super::error::SerdeError;
use super::token_pair::TokenPair;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginFlow {
    token_pair: TokenPair,
    public_encryption_key: String,
}

impl LoginFlow {
    pub fn new(token_pair: TokenPair, public_encryption_key: String) -> Self {
        Self {
            token_pair,
            public_encryption_key,
        }
    }
    pub fn expired(&self) -> bool {
        self.token_pair.expiry.expired()
    }
    pub fn get_token(&self) -> &str {
        &self.token_pair.token
    }
    pub fn get_public_encryption_key(&self) -> &str {
        &self.public_encryption_key
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginCredentials {
    pub email: EmailAddress,
    pub password: String,
    pub two_fa_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLogin {
    pub key: String,
    pub encrypted_credentials: Vec<u8>,
}

impl UserLogin {
    pub fn new(
        login_flow_key: String,
        login_credentials: LoginCredentials,
        public_encryption_key: RsaPublicKey,
    ) -> Result<Self, Error> {
        let serialised_login_credentials = match serde_json::to_vec(&login_credentials) {
            Ok(serialised_login_credentials) => serialised_login_credentials,
            Err(err) => return Err(Error::SerialiseLoginCredentials(SerdeError(err))),
        };
        let encrypted_login_credentlais = match public_encryption_key.encrypt(
            &mut OsRng,
            Pkcs1v15Encrypt,
            &serialised_login_credentials,
        ) {
            Ok(encrypted_login_credentlais) => encrypted_login_credentlais,
            Err(err) => return Err(Error::EncryptLoginCredentials(RSAError(err))),
        };
        Ok(Self {
            key: login_flow_key,
            encrypted_credentials: encrypted_login_credentlais,
        })
    }
}
