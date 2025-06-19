use leptos::prelude::*;
use crate::config::SiteConfig;

#[component]
pub fn Header() -> impl IntoView {
    let site_config = expect_context::<SiteConfig>();
    
    view! {
        <header class="header">
            <div class="header__content">
                <h1 class="header__title">{site_config.name}</h1>
                <button class="header__burger" aria-label="Menu">
                    <span class="burger-line"></span>
                    <span class="burger-line"></span>
                    <span class="burger-line"></span>
                </button>
            </div>
        </header>
    }
}