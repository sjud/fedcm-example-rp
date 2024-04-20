#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use tower_http::cors::{CorsLayer};
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use idp_1::app::*;
    use idp_1::fileserv::file_and_error_handler;
    use http::HeaderValue;
    use http::Method;

    tracing_subscriber::fmt()
    .pretty()
    // enable everything
    .with_env_filter("trace,fedcm-example-rp=trace")
    // sets this to be the default, global collector for this application.
    .init();
    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
    .leptos_routes(&leptos_options, routes, App)
    .route("/.well-known/web-identity",axum::routing::get(idp_1::idp::wellknown))
    .nest("/idp",idp_1::idp::idp_router())
    .fallback(file_and_error_handler)
    .with_state(leptos_options)
    .layer(tower_http::trace::TraceLayer::new_for_http())
    .layer(
        CorsLayer::very_permissive()
                //.allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap())
                //.allow_credentials(true)
                //.allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                //.allow_headers([http::header::CONTENT_TYPE])
    );

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
