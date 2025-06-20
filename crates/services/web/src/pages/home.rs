use leptos::prelude::*;
use crate::components::{Header, Search, Contents};
use crate::config::SiteConfig;

#[component]
pub fn HomePage() -> impl IntoView {
    let site_config = expect_context::<SiteConfig>();
    
    view! {
        <div class="home-page">
            <Header />
            <div class="tagline">
                <p class="tagline__text">{site_config.tagline}</p>
            </div>
            <Search />
            <Contents />
        </div>
    }
}