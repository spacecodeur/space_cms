use leptos::prelude::*;
use leptos::ev;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use space_cms_shared::LoginRequest;

#[component]
pub fn LoginPage() -> impl IntoView {
    let navigate = use_navigate();
    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        let navigate = navigate.clone();
        let username_value = username.get();
        let password_value = password.get();

        if username_value.is_empty() || password_value.is_empty() {
            set_error.set(Some("Please fill in all fields".to_string()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            let login_data = LoginRequest {
                username: username_value,
                password: password_value,
            };

            match crate::api::login(login_data).await {
                Ok(response) => {
                    // Store the token in localStorage
                    #[cfg(feature = "hydrate")]
                    {
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("auth_token", &response.token);
                            }
                        }
                    }
                    
                    // Navigate to home page
                    navigate("/", Default::default());
                }
                Err(err) => {
                    set_error.set(Some(err.to_string()));
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="login-page">
            <h1>"Login"</h1>
            
            <form on:submit=on_submit>
                {move || error.get().map(|err| view! {
                    <div class="error">{err}</div>
                })}
                
                <div>
                    <label for="username">"Username"</label>
                    <input
                        type="text"
                        id="username"
                        value=username
                        on:input=move |ev| set_username.set(event_target_value(&ev))
                        disabled=loading
                    />
                </div>
                
                <div>
                    <label for="password">"Password"</label>
                    <input
                        type="password"
                        id="password"
                        value=password
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        disabled=loading
                    />
                </div>
                
                <button type="submit" disabled=loading>
                    {move || if loading.get() { "Logging in..." } else { "Login" }}
                </button>
            </form>
            
            <p>
                "Don't have an account? "
                <a href="/register">"Register here"</a>
            </p>
        </div>
    }
}