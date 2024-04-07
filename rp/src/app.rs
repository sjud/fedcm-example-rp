
//https://github.com/achimschloss/fedcm-idp-typescript
//https://fedcm-rp-demo.glitch.me/
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
        <Stylesheet id="leptos" href="/pkg/rp.css"/>

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
                    <Route path="" view=Rp/>
                </Routes>
            </main>
        </Router>
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
        <a href="http://127.0.0.2:3001">IDP 1</a>
        </div>
        <div>
        <a href="http://127.0.0.3:3002">IDP 2</a>
        </div>
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
                  providers: [{
                    configURL: "any",
                    //registered:true,
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

        <p id="user_msg"></p>

    }
}





        /* https://github.com/fedidcg/FedCM/issues/554 ???
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
        <p id="user_msg_2"></p>
        <p id="user_name"></p>
        <p id="user_email"></p>
        <img id="user_img"/>*/