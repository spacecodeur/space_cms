use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="hero-content">
                <h1>"Space CMS"</h1>
                <p class="hero-subtitle">
                    "Votre système de gestion de contenu avec réservation de rendez-vous"
                </p>
                <p class="hero-description">
                    "Partagez votre expertise à travers des articles et des ateliers pratiques, "
                    "et permettez à vos lecteurs de réserver des consultations personnalisées."
                </p>
                <div class="hero-actions">
                    <a href="/articles" class="btn btn-primary">"Découvrir les articles"</a>
                    <a href="/appointments" class="btn btn-secondary">"Réserver un rendez-vous"</a>
                </div>
            </div>
        </section>
    }
}