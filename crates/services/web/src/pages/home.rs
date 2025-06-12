use leptos::prelude::*;
use crate::components::layout::{Header, Footer};
use crate::components::home::Hero;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Header />
        <main class="main-content">
            <Hero />
        </main>
        <Footer />
    }
}