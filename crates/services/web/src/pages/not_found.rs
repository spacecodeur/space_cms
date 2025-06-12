use leptos::prelude::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="not-found-page">
            <h1>"404"</h1>
            <p>"Page non trouvée"</p>
            <a href="/">"Retour à l'accueil"</a>
        </div>
    }
}