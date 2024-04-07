use std::str::FromStr;

//https://github.com/achimschloss/fedcm-idp-typescript
//https://fedcm-rp-demo.glitch.me/
use crate::error_template::{AppError, ErrorTemplate};
use http::{HeaderName, HeaderValue};
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
        <Stylesheet id="leptos" href="/pkg/idp_1.css"/>

        <Title text="IDP 1 App"/>

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
                    <Route path="" view=Idp/>
                </Routes>
            </main>
        </Router>
    }
}


#[server]
pub async fn signin() -> Result<(),ServerFnError> {
    let opts = expect_context::<leptos_axum::ResponseOptions>();
    opts.insert_header(HeaderName::from_str("set-login")?,HeaderValue::from_static("logged-in"));
    Ok(())
}
#[server]
pub async fn signout() -> Result<(),ServerFnError> {
    let opts = expect_context::<leptos_axum::ResponseOptions>();
    opts.insert_header(HeaderName::from_str("set-login")?,HeaderValue::from_static("logged-out"));
    Ok(())
}

// https://developer.mozilla.org/en-US/docs/Web/API/FedCM_API/IDP_integration
#[component]
fn Idp() -> impl IntoView {
    let idp_sign_in = Action::<Signin,_>::server();
    let idp_sign_out = Action::<Signout,_>::server();
    let idp_register = create_rw_signal(false);
    create_effect(move|_|if idp_sign_in.value().get().is_some() {
        leptos::logging::log!("sign-in header set to logged in");
    });
    view!{
        <button on:click=move|_|idp_sign_in.dispatch(Signin{})>"Sign In"</button>
        <button on:click=move|_|idp_sign_out.dispatch(Signout{})>"Sign Out"</button>
        <div>
        <button on:click=move|_|idp_register.set(!idp_register.get_untracked())>{
            move || if idp_register.get() {
                "Remove Register Script"
            } else {
                "Add Register Script"
            }
        }</button>
        <Show when=move||idp_register.get()>
        <Script>r#"
        IdentityProvider.register('http://127.0.0.2:3001/idp/config')
            .then((thing) => {
                console.log(thing);
            })
            .catch((err) => {
                console.log(err);
            });
        "#
        </Script>
        </Show>
        </div>
    }
}


