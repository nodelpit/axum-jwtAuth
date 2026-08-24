use jsonwebtoken::{DecodingKey, EncodingKey};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub auth_email: String,
    pub auth_first_name: String,
    pub auth_last_name: String,
    pub auth_password_hash: String,
}
