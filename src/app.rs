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
        <Stylesheet id="leptos" href="/pkg/fedcm-example-rp.css"/>

        <Title text="RP App"/>

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
                    <Route path="rp" view=Rp/>
                    <Route path="idp" view=Idp/>
                </Routes>
            </main>
        </Router>
    }
}



/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div> Sign in With IDP first </div>
        <div>
        <A href="idp">"Click to go to the IDP"</A>
        </div>
        <div> Then sign in to the RP (it relies on the IDP)</div>
        <div>
        <A href="rp">"Click to go to the RP"</A>
        </div>
    }
}

// https://developer.mozilla.org/en-US/docs/Web/API/FedCM_API/RP_sign-in
#[component]
fn Rp() -> impl IntoView {
    // sign into the relying party by signing into the idp
    let rp_signin = create_rw_signal(false);
    let rp_show_user_info = create_rw_signal(false);

    view!{
        <div>
        <button on:click=move|_|rp_signin.set(!rp_signin.get_untracked())>{
            move || if rp_signin.get() {
                "Remove Login Script"
            } else {
                "Add Login Script"
            }
        }</button>
        <Show when=move||rp_signin.get()>
        <Script>r#"
        var paragraph = document.getElementById('user_msg');
        if ('IdentityCredential' in window) {
            navigator.credentials.get({
                identity: {
                    context:"continue",
                  providers: [{
                    configURL: "http://127.0.0.1:3000/idp/config",
                    clientId: "my_client_id",
                    nonce: "123",
                  }]
                }
              })
              .then((credential) => {
                paragraph.innerText = 'Token = ' + credential.token;
                console.log('Credential obtained:', credential);
              })
              .catch((error) => {
                paragraph.innerText = "You have to sign in with the identity provider first.";
                console.error('Error obtaining credentials: ', error.name + ' ' + error.message);
              });
        } else {
            paragraph.innerText = 'FedCM is not available on this browser.';
        }
        "#
        </Script>
        </Show>
        </div>
        <div>
        <button on:click=move|_|rp_show_user_info.set(!rp_show_user_info.get_untracked())>{
            move || if rp_show_user_info.get() {
                "Remove User Info Script"
            } else {
                "Add User Info Script"
            }
        }</button>
        <Show when=move||rp_show_user_info.get()>
     
        <iframe
        src="http://127.0.0.1:3000/idp"
        allow="identity-credentials-get"
        >
        <script>
        r#"
        var user_img = document.getElementById('user_img');
        var user_name = document.getElementById('user_name');
        var user_email = document.getElementById('user_email');
        var paragraph_2 =  document.getElementById('user_msg_2');
        IdentityProvider.getUserInfo({
            configUrl: "http://127.0.0.1:3000/idp/config",
            clientId: "my_client_id",
        }).then((user_info) => {
            if (user_info.length > 0) {
                // Returning accounts should be first, so the first account received
                // is guaranteed to be a returning account
                const name = user_info[0].name;
                const given_name = user_info[0].given_name;
                const display_name = given_name ? given_name : name;
                user_name.innerText = display_name;
                user_img.src = user_info[0].picture;
                user_email.innerText = user_info[0].email;
            } else {
                const paragraph = document.getElementById('user_msg_2');
                paragraph_2.innerText = 'No user info';
            }
        }).catch((err)=>{
            let message = err.name + ' ; ' + err.message + '  ; ' + err.code;
            console.log(message);
        });"#
        </script>
        </iframe>
        </Show>
        </div>
        <p id="user_msg"></p>
        <p id="user_msg_2"></p>
        <p id="user_name"></p>
        <p id="user_email"></p>
        <img id="user_img"/>
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
    // Sign into IDP
    let idp_sign_in = Action::<Signin,_>::server();
    let idp_sign_out = Action::<Signout,_>::server();

    view!{
        <button on:click=move|_|idp_sign_in.dispatch(Signin{})>"Sign In"</button>
        <button on:click=move|_|idp_sign_out.dispatch(Signout{})>"Sign Out"</button>
    }
}



/*
    let sign_up = create_rw_signal(false);
    let register = create_rw_signal(false);
    let authenticate = create_rw_signal(false);


        <div>
        <button on:click=move|_|sign_up.set(!sign_up.get_untracked())>{
            move || if sign_up.get() {
                "Remove Sign Up Script"
            } else {
                "Add Sign Up Script"
            }
        }</button>
        <Show when=move||sign_up.get()>
        <Script>r#"
        navigator.credentials.get({
            identity: {
              context: 'signup',
              providers: [{
                configURL: 'http://127.0.0.1:3000/idp/config.json',
                clientId: "my_client_id",
                nonce: 123
              }]
            }
          }) .then((credential) => {
            console.log('Credential obtained:', credential);
          })
          .catch((error) => {
            console.error('Error obtaining credentials:', error);
          });
          
        "#
        </Script>
        </Show>
        </div>
        <div>
        <button on:click=move|_|register.set(!register.get_untracked())>{
            move || if register.get() {
                "Remove Register Script"
            } else {
                "Add Register Script"
            }
        }</button>
        <Show when=move||register.get()>
        <Script>r#"
        IdentityProvider.register('http://127.0.0.1:3000/idp/config');
        "#
        </Script>
        </Show>
        </div>
        <div>
        <button on:click=move|_|sign_in.set(!sign_in.get_untracked())>{
            move || if sign_in.get() {
                "Remove Sign In Script"
            } else {
                "Add Sign In Script"
            }
        }</button>
        <Show when=move||sign_in.get()>
        <Script>r#"
        // what here?
        "#
        </Script>
        </Show>
        </div>
        <div>
        <button on:click=move|_|authenticate.set(!authenticate.get_untracked())>{
            move || if authenticate.get() {
                "Remove Authenticate Script"
            } else {
                "Add Authenticate Script"
            }
        }</button>
        <Show when=move||authenticate.get()>
        <Script>r#"
     
        "#
        </Script>
        </Show>
        </div>
*/