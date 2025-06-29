use leptos::prelude::*;
use leptos::ev;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use space_cms_shared::RegisterRequest;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let navigate = use_navigate();
    let (username, set_username) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (password_confirm, set_password_confirm) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        let navigate = navigate.clone();
        let username_value = username.get();
        let email_value = email.get();
        let password_value = password.get();
        let password_confirm_value = password_confirm.get();

        // Validation
        if username_value.is_empty() || email_value.is_empty() || password_value.is_empty() {
            set_error.set(Some("Please fill in all fields".to_string()));
            return;
        }

        if password_value != password_confirm_value {
            set_error.set(Some("Passwords do not match".to_string()));
            return;
        }

        if password_value.len() < 6 {
            set_error.set(Some("Password must be at least 6 characters".to_string()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            let register_data = RegisterRequest {
                username: username_value,
                email: email_value,
                password: password_value,
            };

            match crate::api::register(register_data).await {
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
        <div class="register-page">
            <h1>"Create Account"</h1>
            
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
                    <label for="email">"Email"</label>
                    <input
                        type="email"
                        id="email"
                        value=email
                        on:input=move |ev| set_email.set(event_target_value(&ev))
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
                
                <div>
                    <label for="password_confirm">"Confirm Password"</label>
                    <input
                        type="password"
                        id="password_confirm"
                        value=password_confirm
                        on:input=move |ev| set_password_confirm.set(event_target_value(&ev))
                        disabled=loading
                    />
                </div>
                
                <button type="submit" disabled=loading>
                    {move || if loading.get() { "Creating account..." } else { "Register" }}
                </button>
            </form>
            
            <p>
                "Already have an account? "
                <a href="/login">"Login here"</a>
            </p>
        </div>
    }
}