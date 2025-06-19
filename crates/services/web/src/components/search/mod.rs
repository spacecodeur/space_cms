use leptos::{ev::{SubmitEvent}, prelude::*};

#[component]
pub fn Search() -> impl IntoView {
    let (is_loading, set_is_loading) = signal(false);
    let (search_query, set_search_query) = signal(String::new());

    let on_search = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_is_loading.set(true);
        // TODO: Implement actual search logic
        leptos::logging::log!("Searching for: {}", search_query.get());
    };

    view! {
        <section class="search">
            <form class="search__form" on:submit=on_search>
                <div class="search__icon-ai">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                </div>
                <input 
                    type="text"
                    class="search__input"
                    placeholder="Rechercher..."
                    value=search_query
                    on:input=move |ev| set_search_query.set(event_target_value(&ev))
                />
                <div class="search__loading">
                    <Show
                        when=move || is_loading.get()
                        fallback=|| view! { <div></div> }
                    >
                        <div class="loading-spinner">
                            <svg class="spinner" width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                <circle class="spinner-circle" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none"/>
                            </svg>
                        </div>
                    </Show>
                </div>
            </form>
        </section>
    }
}