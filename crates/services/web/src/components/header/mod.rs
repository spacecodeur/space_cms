use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <div class="header__content">
                <h1 class="header__title">"Kunkodio"</h1>
                <button class="header__burger" aria-label="Menu">
                    <span class="burger-line"></span>
                    <span class="burger-line"></span>
                    <span class="burger-line"></span>
                </button>
            </div>
        </header>
    }
}