use once_cell::sync::Lazy;
use regex::Regex;

pub enum NixError {
    HashMismatch {
        current: String,
        expected: String,
        source: String,
    },
}

fn get_capture(regex: &Lazy<Regex>, input: &str, i: usize) -> String {
    regex
        .captures(input)
        .unwrap()
        .get(i)
        .unwrap()
        .as_str()
        .to_string()
}

pub fn handle_error(stderr: &str) -> Vec<NixError> {
    static CURRENT_HASH: Lazy<Regex> = Lazy::new(|| Regex::new(r"specified:\s+([^\n]+)").unwrap());
    static EXPECTED_HASH: Lazy<Regex> = Lazy::new(|| Regex::new(r"got:\s+([^\n]+)").unwrap());
    static HASH_MISMATCH: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"error: hash mismatch in file downloaded from '([^']+)':").unwrap()
    });
    static FIXED_HASH_MISMATCH: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"error: hash mismatch in fixed-output derivation '([^']+)':").unwrap()
    });

    let mut errors = vec![];

    if HASH_MISMATCH.is_match(stderr) {
        errors.push(NixError::HashMismatch {
            current: get_capture(&CURRENT_HASH, stderr, 0),
            expected: get_capture(&EXPECTED_HASH, stderr, 0),
            source: get_capture(&HASH_MISMATCH, stderr, 0),
        });
    }
    if FIXED_HASH_MISMATCH.is_match(stderr) {
        errors.push(NixError::HashMismatch {
            current: get_capture(&CURRENT_HASH, stderr, 0),
            expected: get_capture(&EXPECTED_HASH, stderr, 0),
            source: get_capture(&FIXED_HASH_MISMATCH, stderr, 0),
        });
    }

    errors
}
