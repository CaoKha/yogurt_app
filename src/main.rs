#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum::middleware;
    use leptos::*;
    pub use leptos_axum::{generate_route_list, LeptosRoutes};
    use yogurt_web::backend::model::ModelController;
    pub use yogurt_web::fileserv::file_and_error_handler;
    pub use yogurt_web::{
        app::*,
        backend::{middleware::mw_auth,routes::{routes_login, routes_static, routes_tickets}},
    };
    use tower_cookies::CookieManagerLayer;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let mc = ModelController::new().await.unwrap();
    let route_apis = routes_tickets::routes(mc).route_layer(middleware::from_fn(mw_auth::mw_require_auth)); 

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .merge(routes_login::routes())
        .nest("/api", route_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::routes());

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}


#[cfg(feature = "ssr")]
use axum::response::Response;
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    res
}

// #[cfg(feature = "ssr")]
// async fn main_response_mapper(
// 	ctx: Option<Ctx>,
// 	uri: Uri,
// 	req_method: Method,
// 	res: Response,
// ) -> Response {
// 	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
// 	let uuid = Uuid::new_v4();
//
// 	// -- Get the eventual response error.
// 	let service_error = res.extensions().get::<Error>();
// 	let client_status_error = service_error.map(|se| se.client_status_and_error());
//
// 	// -- If client error, build the new reponse.
// 	let error_response =
// 		client_status_error
// 			.as_ref()
// 			.map(|(status_code, client_error)| {
// 				let client_error_body = json!({
// 					"error": {
// 						"type": client_error.as_ref(),
// 						"req_uuid": uuid.to_string(),
// 					}
// 				});
//
// 				println!("    ->> client_error_body: {client_error_body}");
//
// 				// Build the new response from the client_error_body
// 				(*status_code, Json(client_error_body)).into_response()
// 			});
//
// 	// Build and log the server log line.
// 	let client_error = client_status_error.unzip().1;
// 	// TODO: Need to hander if log_request fail (but should not fail request)
// 	let _ =
// 		log_request(uuid, req_method, uri, ctx, service_error, client_error).await;
//
// 	println!();
// 	error_response.unwrap_or(res)
// }
