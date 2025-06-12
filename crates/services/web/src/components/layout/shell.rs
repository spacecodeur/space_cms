use leptos::prelude::*;

#[component]
pub fn Shell(children: Children) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"Space CMS"</title>
                <link rel="stylesheet" href="/style/main.css"/>
            </head>
            <body>
                <div id="app">
                    {children()}
                </div>
            </body>
        </html>
    }
}