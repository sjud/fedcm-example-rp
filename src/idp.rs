/*
https://developer.mozilla.org/en-US/docs/Web/API/FedCM_API/IDP_integration
https://fedidcg.github.io/FedCM/
https://developers.google.com/privacy-sandbox/3pcd/fedcm-developer-guide
*/

use axum::{
    extract::Query,
    routing::{get,post},
    Router,
    Json,
};
use leptos::LeptosOptions;
use serde::{Serialize,Deserialize};

pub fn idp_router() -> Router<LeptosOptions> {
    Router::<LeptosOptions>::new()
        .route("/config",get(config))
        .route("/tos",get(tos))
        .route("/privacy_policy",get(privacy_policy))
        .route("/metadata",get(metadata))
        .route("/assertion",post(assertion))
        .route("/disconnect",post(disconnect))
        .route("/accounts",get(accounts))
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct IdentityProviderBranding {
    pub background_color: String,
    pub color: String,
    pub icons:Vec<IdentityProviderIcon>,
    pub  name:String,
}
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct IdentityProviderIcon{
    pub url: String,
    pub size: usize,
}
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct IdentityProviderAPIConfig{
    pub accounts_endpoint: String,
    pub client_metadata_endpoint: String,
    pub id_assertion_endpoint:String,
    pub login_url:String,
    pub disconnect_endpoint:Option<String>,
    pub branding:Option<IdentityProviderBranding>,
}

#[tracing::instrument(ret)]
pub async fn config() -> Json<IdentityProviderAPIConfig> {
    tracing::trace!("hello");

    Json(IdentityProviderAPIConfig {
        accounts_endpoint: "/idp/accounts".into(),
        client_metadata_endpoint: "/idp/metadata".into(),
        id_assertion_endpoint: "/idp/assertion".into(),
        login_url:  "/idp_login".into(),
        disconnect_endpoint: Some("/idp/disconnect".into()),
        branding: Some(IdentityProviderBranding {
            background_color: "green".into(),
            color: "#FFEEAA".into(),
            icons: vec![
                IdentityProviderIcon {
                    url: "http://127.0.0.1:3000/favicon.ico".into(),
                    size: 25, // Consider changing to a numeric type if applicable
                },
            ],
            name: "Leptos IDP Example".into(),
        }),
    })
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct IdentityProviderAccount {
    pub id:String,
    pub name:String,
    pub email:String,
    pub given_name:Option<String>,
    pub picture:Option<String>,
    pub approved_clients:Vec<String>,
    pub login_hints:Vec<String>,
    pub domain_hints:Vec<String>,
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct IdentityProviderAccountList{
    pub accounts:Vec<IdentityProviderAccount>
}

#[tracing::instrument(ret)]
pub async fn accounts() -> Json<IdentityProviderAccountList> {
    Json(IdentityProviderAccountList {
        accounts: vec![
            IdentityProviderAccount {
                id: "123".into(),
                name: "Jane Doe".into(),
                email: "jane.doe@example.com".into(),
                given_name: Some("Jane".into()),
                picture: Some("http://127.0.0.1:3000/app_user.png".into()),
                approved_clients: vec!["client1".into(), "client2".into()],
                login_hints: vec!["hint1".into(), "hint2".into()],
                domain_hints: vec!["example.com".into()],
            },
            // Add more accounts as needed
        ],
    })
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct IdentityProviderClientMetadata {
    pub privacy_policy_url:String,
    pub terms_of_service_url:String,
}

#[tracing::instrument(ret)]
pub async fn metadata() -> Json<IdentityProviderClientMetadata>{
    Json(IdentityProviderClientMetadata{
        privacy_policy_url:"/idp/privacy".into(),
        terms_of_service_url:"/idp/tos".into(),
    })
}

#[tracing::instrument(ret)]
pub async fn privacy_policy() -> String {
    "No privacy sorry.".to_string()
}
#[tracing::instrument(ret)]
pub async fn tos() -> String {
    "No TOS oopsies".to_string()
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct IdentityProviderToken{
    pub token:String,
}

//account_id=123&client_id=client1234&nonce=Ct60bD&disclosure_text_shown=true
#[tracing::instrument(ret)]
pub async fn assertion(account_id:Query<String>,client_id:Query<String>,nonce:Query<String>,disclosure_text_shown:Query<bool>) -> Json<IdentityProviderToken> {
    Json(IdentityProviderToken{
            token:"idk_a_token_i_guess".into()
    })
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct DisconnectedAccount  {
    pub account_id:String,
}

#[tracing::instrument(ret)]
pub async fn disconnect() -> Json<DisconnectedAccount>  {
    Json(
        DisconnectedAccount{
            account_id:"123".into()
        }
    )
}