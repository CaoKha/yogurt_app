// region:    --- Modules

mod error;
mod config;
pub mod middleware;
pub mod routes;

pub use self::error::ClientError;
pub use self::error::{Error, Result};
use crate::backend::auth::token::generate_web_token;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

// endregion: --- Modules

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> Result<()> {
	let token = generate_web_token(user, salt)?;

	let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
	cookie.set_http_only(true);
	cookie.set_path("/");

	cookies.add(cookie);

	Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
	let mut cookie = Cookie::from(AUTH_TOKEN);
	cookie.set_path("/");

	cookies.remove(cookie);

	Ok(())
}