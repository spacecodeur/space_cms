use leptos::prelude::*;
use crate::components::{Header, Search, Contents};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <Header />
            <Search />
            <Contents />
        </div>
    }
}