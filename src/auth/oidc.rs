use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::{
    ClientId, ClientSecret, EndpointMaybeSet, EndpointNotSet, EndpointSet, IssuerUrl, RedirectUrl,
    reqwest,
};
use reqwest::Client;
use std::env;

struct OidcSettings {
    client_id: String,
    client_secret: String,
    issuer_url: String,
    redirect_url: String,
}

fn get_oidc_client_settings() -> Result<OidcSettings, String> {
    let client_id =
        env::var("OIDC_CLIENT_ID").map_err(|_| "OIDC_CLIENT_ID environment variable missing")?;
    let client_secret = env::var("OIDC_CLIENT_SECRET")
        .map_err(|_| "OIDC_CLIENT_SECRET environment variable missing")?;
    let issuer_url =
        env::var("OIDC_ISSUER_URL").map_err(|_| "OIDC_ISSUER_URL environment variable missing")?;
    let redirect_url = env::var("OIDC_REDIRECT_URL")
        .map_err(|_| "OIDC_REDIRECT_URL environment variable missing")?;

    Ok(OidcSettings {
        client_id: client_id,
        client_secret: client_secret,
        issuer_url: issuer_url,
        redirect_url: redirect_url,
    })
}

pub async fn get_oidc_client(http_client: &Client) -> Result<
    CoreClient<
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointMaybeSet,
        EndpointMaybeSet,
    >,
    String,
> {
    let settings = get_oidc_client_settings()?;

    let issuer_url =
        IssuerUrl::new(settings.issuer_url).map_err(|_| "Could not construct issuer URL")?;
    let client_id = ClientId::new(settings.client_id);
    let client_secret = ClientSecret::new(settings.client_secret);
    let redirect_url =
        RedirectUrl::new(settings.redirect_url).map_err(|_| "Could not construct redirect URL")?;

    let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, http_client)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            "Could not discover OIDC provider metadata"
        })?;

    let result =
        CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
            .set_redirect_uri(redirect_url);

    Ok(result)
}
