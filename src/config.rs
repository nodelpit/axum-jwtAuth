#[derive(Clone)]
pub struct Config {
    pub jwt_private_key: String,
    pub jwt_public_key: String,
    pub auth_email: String,
    pub auth_first_name: String,
    pub auth_last_name: String,
    pub auth_password_hash: String,
}
