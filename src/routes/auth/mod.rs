/*
    Auth Routes
 */

mod register;
mod login;

pub use register::post_auth_register;
pub use login::post_auth_login;
