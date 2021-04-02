use lazy_static::lazy_static;

lazy_static! {
    // config taken from https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#introduction
    static ref ARGON2_CONFIG: argon2::Config<'static> = argon2::Config {
        mem_cost: 15 * 1024, // 15 MB
        time_cost: 2,
        ..Default::default()
    };
}

/// Returns true if a given `password` and `hash` match
pub fn verify_matches(password: &[u8], hash: &str) -> bool {
    argon2::verify_encoded(hash, password).unwrap()
}

pub fn hash_password(password: &[u8]) -> String {
    // TODO(will): generate better salt
    let salt = vec![0; password.len() * 2];

    argon2::hash_encoded(password, salt.as_slice(), &ARGON2_CONFIG).unwrap()
}
