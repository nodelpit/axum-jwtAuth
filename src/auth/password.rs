use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};

// Verify a plain-text password against a brcypt hash
pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}
// Hash a password with brcypt when creating or storing a password
pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}
