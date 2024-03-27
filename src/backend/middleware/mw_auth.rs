use crate::backend::{ctx::Ctx, error::Error};
use axum::{body::Body, http::Request, middleware::Next, response::Response};

pub async fn mw_require_auth(ctx: Result<Ctx, Error>, req: Request<Body>, next: Next) -> Result<Response, Error> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)

}
