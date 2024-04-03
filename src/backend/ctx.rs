use axum::{async_trait, extract::FromRequestParts};
use http::request::Parts;
use tracing::debug;
use crate::backend::error::Error;

#[derive(Clone, Debug)]
/// Context -- authenticate person who call the api
pub struct Ctx {
	user_id: u64,
}

// Constructor.
impl Ctx {
	pub fn new(user_id: u64) -> Self {
		Self { user_id }
	}
}

// Property Accessors.
impl Ctx {
	pub fn user_id(&self) -> u64 {
		self.user_id
	}
}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
		debug!("{:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<Result<Ctx,Self::Rejection>>()
			.ok_or(Error::AuthFailCtxNotInRequestExt)?
			.clone()
	}
}

// endregion: --- Ctx Extractor
