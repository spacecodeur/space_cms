#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{Router, routing::get};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use space_cms::app::*;
    use space_cms_backend::{api_routes, create_pool, run_migrations};
    use space_cms_frontend::api::*;

    // Load environment variables
    dotenvy::dotenv().ok();
    
    // Setup database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:space_cms.db".to_string());
    
    let pool = create_pool(&database_url)
        .await
        .expect("Failed to create database pool");
    
    // Run migrations
    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .merge(api_routes(pool.clone()))
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
