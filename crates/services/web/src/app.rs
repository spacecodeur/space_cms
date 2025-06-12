use leptos::prelude::*;
use crate::components::layout::Shell;
use crate::pages::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Shell>
            <HomePage />
        </Shell>
    }
}