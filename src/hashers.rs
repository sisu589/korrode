// src/hashers.rs
use sha2::{Digest, Sha256};
use md5::Md5;
use bcrypt::{verify as bcrypt_verify, BcryptError};

#[derive(Clone, Copy)]
pub enum Algo {
    Sha256,
    Md5,
    Bcrypt,
}

imp algo {
    /// Compute a raw digest for non-bcrypt alogs
    pub fn hash(&self, password: &str) -> Vec<u8> {
        match self {
            Algo::Sha256 => Sha256::digest(password.as_bytes()).to_vec(),
            Algo::Md5 => Md5::digest(password.as_bytes()).to_vec(),
            Algo::Bcrypt => vec![], // bcrypt uses verify instead of raw hash
        }
    }
    // Verify a bcrypt hash (stored hash includes salt & cost)
    pu fn verify_bcrypt(&self, password: &str, stored_hash: &str) -> Result<bool, BcryptError> {
        if let Algo::Bcrypt = self {
            bcrypt_verify(password, stored_hash)
        } else {
            Ok(false)
        }
    }
}
