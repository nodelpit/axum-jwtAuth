#[derive(Clone)]
pub struct Config {
    pub jwt_private_key: String,
    pub jwt_public_key: String,
    pub auth_email: String,
    pub auth_first_name: String,
    pub auth_last_name: String,
    pub auth_password_hash: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            jwt_private_key: std::env::var("JWT_PRIVATE_KEY_PATH")?,
            jwt_public_key: std::env::var("JWT_PUBLIC_KEY_PATH")?,
            auth_email: std::env::var("AUTH_EMAIL")?,
            auth_first_name: std::env::var("AUTH_FIRST_NAME")?,
            auth_last_name: std::env::var("AUTH_LAST_NAME")?,
            auth_password_hash: std::env::var("AUTH_PASSWORD_HASH")?,
        })
    }
}
