use crate::{models::CurrentUser, state::AppState};

// The .env file acts as a fake DB containing a single user.
pub fn retrieve_user_by_email(email: &str, state: &AppState) -> Option<CurrentUser> {
    if email != state.inner.auth_email {
        return None;
    }
    Some(CurrentUser {
        email: state.inner.auth_email.clone(),
        first_name: state.inner.auth_first_name.clone(),
        last_name: state.inner.auth_last_name.clone(),
        password_hash: state.inner.auth_password_hash.clone(),
    })
}
