use crate::components::Counter;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Space CMS!"</h1>
        <Counter />
        
        <nav>
            <p>
                <a href="/login">"Login"</a>
                " | "
                <a href="/register">"Register"</a>
            </p>
        </nav>
    }
}
