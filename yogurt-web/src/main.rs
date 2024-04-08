#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), yogurt_web::backend::error::Error> {
    use yogurt_web::backend::web::middleware::mw_req_stamp::mw_req_stamp_resolver;
    use axum::middleware;
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_cookies::CookieManagerLayer;
    use yogurt_web::backend::_dev_utils;
    use yogurt_web::backend::model::ModelManager;
    use tracing_subscriber::EnvFilter;
    use yogurt_web::fileserv::file_and_error_handler;
    use yogurt_web::{app::*, backend};
    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        // .with_env_filter("yogurt_web=debug")
        .init();

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    let mm = ModelManager::new().await?;
    let route_rpc = backend::web::routes::routes_rpc::routes(mm.clone()).route_layer(
        middleware::from_fn(backend::web::middleware::mw_auth::mw_ctx_require),
    );

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .merge(backend::web::routes::routes_login::routes(mm.clone()))
        .nest("/api", route_rpc)
        .layer(middleware::map_response(
            backend::web::middleware::mw_res_map::mw_reponse_map,
        ))
        .layer(middleware::from_fn_with_state(
            mm.clone(),
            backend::web::middleware::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .layer(middleware::from_fn(mw_req_stamp_resolver));
        // .fallback_service(backend::web::routes::routes_static::serve_dir());

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
