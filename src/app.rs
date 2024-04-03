use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/fedcm-example-rp.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="idp_login" view=IdpLogin/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let authenticate = create_rw_signal(false);
    view! {
        <button on:click=move|_|authenticate.set(!authenticate.get_untracked())>{
            move || if authenticate.get() {
                "Remove Authenticate Script"
            } else {
                "Add Authenticate Script"
            }
        }</button>
        <Show when=move||authenticate.get()>
        <Script>r#"
        navigator.credentials.get({
            identity: {
              providers: [{
                configURL: "http://127.0.0.1:3000/idp/config",
                clientId: "http://127.0.0.1:3000",
                nonce: "123",
              }]
            }
          });
        "#
        </Script>
        </Show>
    }
}

#[component]
pub fn IdpLogin() -> impl IntoView{
    view!{
        <div>
            <button value="Login"/>
        </div>
    }
}
