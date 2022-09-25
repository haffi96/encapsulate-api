use argon2::{self, Config};

use crate::errors::ServiceError;

pub static SECRET_KEY: &str = "foobar";
const SALT: &[u8] = b"supersecuresalt";

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let config = Config {
        secret: SECRET_KEY.as_bytes(),
        ..Default::default()
    };
    argon2::hash_encoded(password.as_bytes(), SALT, &config)
        .map_err(|err| ServiceError::InternalServerError)
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[])
        .map_err(|err| ServiceError::Unauthorized)
}
