#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use cloud_class::app::*;
    use cloud_class::fileserv::file_and_error_handler;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::compression::{
        predicate::{NotForContentType, SizeAbove},
        CompressionLayer, CompressionLevel, Predicate,
    };

    use cloud_class::state::AppState;
    use cloud_class::utils::db::*;
    use cloud_class::utils::regex::*;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let regex = InputValidationRegex::get_regex();

    let db_pool = match create_pool().await {
        Ok(p) => p,
        Err(e) => panic!("{}", e.to_string()),
    };

    let app_state = AppState {
        leptos_options,
        pool: db_pool.clone(),
        validation_regex: regex,
    };

    // files smaller than 1501 bytes are not compressed, since
    // the MTU (Maximum Transmission Unit) of a TCP packet is 1500 bytes
    let predicate = SizeAbove::new(1500)
        .and(NotForContentType::GRPC)
        .and(NotForContentType::IMAGES)
        // prevent compressing assets that are already statically compressed
        .and(NotForContentType::const_new("application/javascript"))
        .and(NotForContentType::const_new("application/wasm"))
        .and(NotForContentType::const_new("text/css"));

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&app_state, routes, App)
        .layer(
            CompressionLayer::new()
                .quality(CompressionLevel::Fastest)
                .compress_when(predicate),
        )
        .fallback(file_and_error_handler)
        .with_state(app_state);

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
