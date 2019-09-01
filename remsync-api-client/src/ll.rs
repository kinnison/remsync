//! Low level API implementations for remsync-api-client

use hyper::{client::connect::Connect, Body, Client, Request, Uri};
use remsync_api_types::*;

use crate::{util::*, GenericResult};

pub async fn auth_get_device_bearer<C>(
    client: &Client<C, Body>,
    base: &Uri,
    req: &DeviceTokenRequest,
) -> GenericResult<String>
where
    C: Connect + Sync + 'static,
{
    let body = serde_json::to_string(req)?;
    let request = Request::builder()
        .method("POST")
        .uri(catenate_url_path(base, "/token/json/2/device/new")?)
        .body(Body::from(body))?;

    let response = client.request(request).await?;

    if !response.status().is_success() {
        // Failed to get something back
        return Err(format!("API:GetDeviceBearer:{:?}", response).into());
    }

    // The body if there is one, is our new bearer token, so return it
    let ret = hoover_body_to_vec(response.into_body()).await?;
    Ok(String::from_utf8(ret)?)
}

pub async fn auth_get_user_bearer<C>(
    client: &Client<C, Body>,
    base: &Uri,
    device_bearer: &str,
) -> GenericResult<String>
where
    C: Connect + Sync + 'static,
{
    let request = Request::builder()
        .header("Authorization", format!("Bearer {}", device_bearer))
        .header("Content-Length", "0") // No body
        .method("POST")
        .uri(catenate_url_path(base, "/token/json/2/user/new")?)
        .body(Body::empty())?;
    let response = client.request(request).await?;

    if !response.status().is_success() {
        return Err(format!("API:GetUserBearer:{:?}", response).into());
    }

    // The body if there is one, is our new bearer token, so return it
    let ret = hoover_body_to_vec(response.into_body()).await?;
    Ok(String::from_utf8(ret)?)
}

pub async fn discover_storage_service<C>(
    client: &Client<C, Body>,
    base: &Uri,
    user_bearer: &str,
) -> GenericResult<String>
where
    C: Connect + Sync + 'static,
{
    let token = jsonwebtoken::dangerous_unsafe_decode::<UserToken>(user_bearer)?;
    let group = token.claims.auth0_profile().user_id();
    let group = percent_encoding::utf8_percent_encode(&group, &percent_encoding::NON_ALPHANUMERIC);
    let request = Request::builder()
        .header("Authorization", format!("Bearer {}", user_bearer))
        .method("GET")
        .uri(catenate_url_path(
            base,
            &format!(
                "/service/json/1/document-storage?environment=production&apiVer=2&group={}",
                group
            ),
        )?)
        .body(Body::empty())?;
    let response = client.request(request).await?;

    if !response.status().is_success() {
        return Err(format!("API:DiscoverStorageService:{:?}", response).into());
    }

    let ret = hoover_body_to_vec(response.into_body()).await?;
    let ret: DiscoveryResponse = serde_json::from_slice(&ret)?;
    if ret.status() != "OK" {
        return Err("Unknown error during discovery".into());
    }
    Ok(ret.into_host())
}

pub async fn storage_fetch_all_docs<C>(
    client: &Client<C, Body>,
    base: &Uri,
    user_token: &str,
) -> GenericResult<Vec<DocsResponse>>
where
    C: Connect + Sync + 'static,
{
    let request = Request::builder()
        .header("Authorization", format!("Bearer {}", user_token))
        .method("GET")
        .uri(catenate_url_path(base, "/document-storage/json/2/docs")?)
        .body(Body::empty())?;
    let response = client.request(request).await?;

    if !response.status().is_success() {
        return Err(format!("API:GetDocsList:{:?}", response).into());
    }

    // The body is a JSON list of document nodes
    let ret = hoover_body_to_vec(response.into_body()).await?;
    Ok(serde_json::from_slice(&ret)?)
}
