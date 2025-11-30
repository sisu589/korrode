// src/engine.rs
use crate::hashers::Algo;
use rayon::prelude::*;
use zeroize::Zeroizing;

/// Parallel cracking engine
///
/// * 'algo' - selected hashing algorithm
/// * 'target_hex' - expected hash (hex) for non-bcrypt
/// * 'candidates' - any iterator yielding candidate passwords
///
/// Returns the cracked password if found.
pub fn crack<I>(algo: Algo, target_hex: &str, candidates: I) -> Option<String>
where
    I: IntoIterator,
    I::Item: Send + Into<String>,
{
    // Decode target hash once (skip for bcrypt - handled separately)
    let target_bytes = match algo {
        Algo::Bcrypt => return None,
        _ => hex::decode(target_hex).expect("Invalid hex target"),
    };

    candidates.into_iter().par_bridge().find_any(|pwd| {
        let pwd_str: String = pwd.into();
        let pwd_secure = Zeroizing::new(pwd_str);
        let computed = algo.hash(&pwd_secure);
        computed == target_bytes
    })
}
