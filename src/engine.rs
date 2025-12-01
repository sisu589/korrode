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
    // Pull everything into a Vec
    let cand_vec: Vec<String> = candidates.into_iter().map(Into::into).collect();
    // Parallel search oer the Vec
    cand_vec
        .par_iter()
        .find_any(|pwd| {
            // Zero-out the password after use
            let pwd_secure = Zeroizing::new((*pwd).clone());
            let computed = algo.hash(&pwd_secure);
            computed == target_bytes
        })
        .cloned()
}
