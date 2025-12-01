// src/hashers.rs
use bcrypt::{BcryptError, verify as bcrypt_verify};
use md5;
use sha2::{Digest, Sha256};

#[derive(Clone, Copy)]
pub enum Algo {
    Sha256,
    Md5,
    Bcrypt,
}

impl Algo {
    /// Compute a raw digest for non-bcrypt alogs
    pub fn hash(&self, password: &str) -> Vec<u8> {
        match self {
            Algo::Sha256 => Sha256::digest(password.as_bytes()).to_vec(),
            Algo::Md5 => {
                // conver to Vec
                md5::compute(password.as_bytes()).to_vec()
            }
            Algo::Bcrypt => vec![], // bcrypt uses verify instead of raw hash
        }
    }
    // Verify a bcrypt hash (stored hash includes salt & cost)
    pub fn verify_bcrypt(&self, password: &str, stored_hash: &str) -> Result<bool, BcryptError> {
        if let Algo::Bcrypt = self {
            bcrypt_verify(password, stored_hash)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bcrypt::DEFAULT_COST;

    #[test]
    fn bcrypt_verify_works() {
        let password = "s3cur3P@ss!";
        let hash = bcrypt::hash(password, DEFAULT_COST).expect("failed to create bcrypt hash");
        let result = Algo::Bcrypt
            .verify_bcrypt(password, &hash)
            .expect("bcrypt verification errored");
        assert!(result, "bcrypt verification should succeed");
    }

    #[test]
    fn sha256_hash_is_correct() {
        // Known Sha-256 of hello
        let expected = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
        let got = Algo::Sha256.hash("hello");
        assert_eq!(hex::encode(got), expected);
    }

    #[test]
    fn md5_hash_is_correct() {
        // known MD% of 'Hello'
        let expected = "5d41402abc4b2a76b9719d911017c592";
        let got = Algo::Md5.hash("hello");
        assert_eq!(hex::encode(got), expected)
    }
}
