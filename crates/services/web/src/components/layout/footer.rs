use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="container">
                <p>"© 2025 Space CMS - Tous droits réservés"</p>
            </div>
        </footer>
    }
}