use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower::ServiceExt;
use tower_http::services::ServeDir;
use web::server::ServerConfig;

async fn render_app() -> impl IntoResponse {
    let html = r#"
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Space CMS</title>
                <style>
                    :root {
                        --color-primary: #2563eb;
                        --color-primary-dark: #1d4ed8;
                        --color-secondary: #64748b;
                        --color-background: #f8fafc;
                        --color-surface: #ffffff;
                        --color-text: #1e293b;
                        --color-text-light: #64748b;
                        --shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
                        --shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1);
                        --radius: 0.5rem;
                        --max-width: 1200px;
                    }
                    * { margin: 0; padding: 0; box-sizing: border-box; }
                    body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background-color: var(--color-background); color: var(--color-text); line-height: 1.6; }
                    .container { max-width: var(--max-width); margin: 0 auto; padding: 0 1rem; }
                    .site-header { background-color: var(--color-surface); box-shadow: var(--shadow-sm); position: sticky; top: 0; z-index: 100; }
                    .site-header .container { display: flex; justify-content: space-between; align-items: center; padding: 1rem; }
                    .logo { font-size: 1.5rem; font-weight: bold; color: var(--color-primary); }
                    .main-nav { display: flex; gap: 2rem; }
                    .main-nav a { color: var(--color-text); text-decoration: none; font-weight: 500; transition: color 0.2s; }
                    .main-nav a:hover { color: var(--color-primary); }
                    .hero { padding: 4rem 1rem; text-align: center; }
                    .hero-content h1 { font-size: 3rem; margin-bottom: 1rem; color: var(--color-primary); }
                    .hero-subtitle { font-size: 1.5rem; color: var(--color-text); margin-bottom: 1rem; }
                    .hero-description { font-size: 1.125rem; color: var(--color-text-light); max-width: 600px; margin: 0 auto 2rem; }
                    .hero-actions { display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap; }
                    .btn { display: inline-block; padding: 0.75rem 1.5rem; border-radius: var(--radius); font-weight: 500; text-decoration: none; transition: all 0.2s; cursor: pointer; border: none; }
                    .btn-primary { background-color: var(--color-primary); color: white; }
                    .btn-primary:hover { background-color: var(--color-primary-dark); transform: translateY(-1px); box-shadow: var(--shadow-md); }
                    .btn-secondary { background-color: var(--color-surface); color: var(--color-primary); border: 2px solid var(--color-primary); }
                    .btn-secondary:hover { background-color: var(--color-primary); color: white; }
                    .main-content { min-height: calc(100vh - 200px); }
                    .site-footer { background-color: var(--color-surface); padding: 2rem 0; margin-top: 4rem; border-top: 1px solid #e2e8f0; }
                    .site-footer p { text-align: center; color: var(--color-text-light); }
                </style>
            </head>
            <body>
                <div id="app">
                    <header class="site-header">
                        <div class="container">
                            <h1 class="logo">Space CMS</h1>
                            <nav class="main-nav">
                                <a href="/">Accueil</a>
                                <a href="/articles">Articles</a>
                                <a href="/practice">Pratique</a>
                                <a href="/appointments">Rendez-vous</a>
                            </nav>
                        </div>
                    </header>
                    <main class="main-content">
                        <section class="hero">
                            <div class="hero-content">
                                <h1>Space CMS</h1>
                                <p class="hero-subtitle">
                                    Votre système de gestion de contenu avec réservation de rendez-vous
                                </p>
                                <p class="hero-description">
                                    Partagez votre expertise à travers des articles et des ateliers pratiques,
                                    et permettez à vos lecteurs de réserver des consultations personnalisées.
                                </p>
                                <div class="hero-actions">
                                    <a href="/articles" class="btn btn-primary">Découvrir les articles</a>
                                    <a href="/appointments" class="btn btn-secondary">Réserver un rendez-vous</a>
                                </div>
                            </div>
                        </section>
                    </main>
                    <footer class="site-footer">
                        <div class="container">
                            <p>© 2025 Space CMS - Tous droits réservés</p>
                        </div>
                    </footer>
                </div>
            </body>
        </html>
    "#;

    Html(html)
}


#[tokio::main]
async fn main() {
    let config = ServerConfig::new();
    
    let app = Router::new()
        .route("/", get(render_app))
        .nest_service("/style", ServeDir::new("/app/crates/services/web/style"))
        .fallback(get(render_app));

    println!("🚀 Server running at http://{}", config.addr);
    println!("📦 Using modular Leptos structure with Axum");
    
    let listener = tokio::net::TcpListener::bind(config.addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}