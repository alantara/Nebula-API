/*
    General Routes
 */

mod health;
mod auth;

pub use health::get_health;
pub use auth::{post_auth_register, post_auth_login};
