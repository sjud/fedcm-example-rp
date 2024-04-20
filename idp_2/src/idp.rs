/*
https://developer.mozilla.org/en-US/docs/Web/API/FedCM_API/IDP_integration
https://fedidcg.github.io/FedCM/
https://developers.google.com/privacy-sandbox/3pcd/fedcm-developer-guide
*/

use axum::{
    Form,
    routing::{get,post},
    Router,
    Json,
    http::header::HeaderMap,
};
use axum_extra::{
    extract::cookie::CookieJar,
};
use leptos::LeptosOptions;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
pub struct WebIdentity{
    provider_urls:Vec<String>
}

pub async fn wellknown() -> Json<WebIdentity> {
    Json(
        WebIdentity{
            provider_urls:vec!["http://127.0.0.2:3001/idp/config".into()]
        } 
    )
}

pub fn idp_router() -> Router<LeptosOptions> {
    Router::<LeptosOptions>::new()
        .route("/tos",get(tos))
        .route("/config",get(config))
        .route("/privacy_policy",get(privacy_policy) )
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
    Json(IdentityProviderAPIConfig {
        accounts_endpoint: "/idp/accounts".into(),
        client_metadata_endpoint: "/idp/metadata".into(),
        id_assertion_endpoint: "/idp/assertion".into(),
        login_url:  "/idp".into(),
        disconnect_endpoint: Some("/idp/disconnect".into()),
        branding: Some(IdentityProviderBranding {
            background_color: "green".into(),
            color: "#FFEEAA".into(),
            icons: vec![
                IdentityProviderIcon {
                    url: "http://127.0.0.3:3002/favicon.ico".into(),
                    size: 25,
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
pub async fn accounts(
    // expecting Sec-Fetch-Dest: webidentity
    headers: HeaderMap,
    jar: CookieJar,
) -> Json<IdentityProviderAccountList> {

    Json(IdentityProviderAccountList {
        accounts: vec![
            IdentityProviderAccount {
                id: "123".into(),
                name: "Red Riding Hood".into(),
                email: "red_hood@riding.com".into(),
                given_name: Some("Red".into()),
                picture: Some("http://127.0.0.3:3002/red_riding_hood.png".into()),
                approved_clients: vec![],
                login_hints: vec![],
                domain_hints: vec![],
            },
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
    "No TOS".to_string()
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct IdentityProviderToken{
    pub token:String,
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct IdentityProviderAssertionForm{
    pub account_id:String,
    pub client_id:String,
    pub nonce:String,
    pub disclosure_text_shown:bool 
}
#[tracing::instrument(ret)]
pub async fn assertion(Form(f):Form<IdentityProviderAssertionForm>) -> Json<IdentityProviderToken> {
    Json(IdentityProviderToken{
            token:"asjo;asdfjadlsfsadpufdosafjsdkflkshgapf;".into()
    })
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct DisconnectedAccount  {
    pub account_id:String,
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct DisconnectForm{
    pub account_hint:String,
    pub client_id:String,
}
#[tracing::instrument(ret)]
pub async fn disconnect(
    Form(_):Form<DisconnectForm>,
) -> Json<DisconnectedAccount>  {
    Json(
        DisconnectedAccount{
            account_id:"123".into()
        }
    )
}

 


