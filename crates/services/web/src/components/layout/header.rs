use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="site-header">
            <div class="container">
                <h1 class="logo">"Space CMS"</h1>
                <nav class="main-nav">
                    <a href="/">"Accueil"</a>
                    <a href="/articles">"Articles"</a>
                    <a href="/practice">"Pratique"</a>
                    <a href="/appointments">"Rendez-vous"</a>
                </nav>
            </div>
        </header>
    }
}