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

pub async fn storage_fetch_blob<C>(
    client: &Client<C, Body>,
    base: &Uri,
    user_token: &str,
    id: &str,
    output: &mut dyn std::io::Write,
) -> GenericResult<usize>
where
    C: Connect + Sync + 'static,
{
    let doc = percent_encoding::utf8_percent_encode(id, &percent_encoding::NON_ALPHANUMERIC);
    let request = Request::builder()
        .header("Authorization", format!("Bearer {}", user_token))
        .method("GET")
        .uri(catenate_url_path(
            base,
            &format!("/document-storage/json/2/docs?withBlob=1&doc={}", doc),
        )?)
        .body(Body::empty())?;
    let response = client.request(request).await?;

    if !response.status().is_success() {
        return Err(format!("API:GetDocsList:{:?}", response).into());
    }

    // The body is a JSON list of document nodes
    let docs = hoover_body_to_vec(response.into_body()).await?;
    let docs: Vec<DocsResponse> = serde_json::from_slice(&docs)?;
    if docs.len() != 1 {
        return Err(format!("API:GetDocsList: Expected 1, got {} documents", docs.len()).into());
    }

    if docs[0].blob_url_get().is_empty() {
        return Err(format!("API:GetDocsList: Blob URL missing: {:?}", docs[0]).into());
    }

    let request = Request::builder()
        .method("GET")
        .uri(docs[0].blob_url_get())
        .body(Body::empty())?;
    let response = client.request(request).await?;

    if !response.status().is_success() {
        return Err(format!("API:GetBlob:{:?}", response).into());
    }

    let mut body = response.into_body();
    let mut written = 0;
    while let Some(next) = body.next().await {
        let chunk = next?;
        output.write_all(&chunk)?;
        written += chunk.len();
    }

    Ok(written)
}

pub async fn storage_delete_doc<C>(
    client: &Client<C, Body>,
    base: &Uri,
    user_token: &str,
    id: &str,
    version: usize,
) -> GenericResult<()>
where
    C: Connect + Sync + 'static,
{
    let req = DeleteRequest::new(id, version);
    let request = Request::builder()
        .method("PUT")
        .header("Authorization", format!("Bearer {}", user_token))
        .uri(catenate_url_path(base, "/document-storage/json/2/delete")?)
        .body(Body::from(serde_json::to_string(&req)?))?;
    let response = client.request(request).await?;

    if !response.status().is_success() {
        return Err(format!("API:DeleteDoc:{:?}", response).into());
    }

    let ret = hoover_body_to_vec(response.into_body()).await?;
    let ret: DeleteResponse = serde_json::from_slice(&ret)?;
    if !ret.success() {
        return Err(format!("API:DeleteDoc:{}", ret.message()).into());
    }
    Ok(())
}

pub async fn storage_update_doc<C>(
    client: &Client<C, Body>,
    base: &Uri,
    user_token: &str,
    id: &str,
    version: usize,
    parent: &str,
    node_type: NodeType,
    bookmarked: bool,
    current_page: usize,
    name: &str,
    modified_client: &str,
) -> GenericResult<()>
where
    C: Connect + Sync + 'static,
{
    let req = UpdateStatusRequest::new(
        id,
        parent,
        node_type,
        version,
        bookmarked,
        current_page,
        name,
        modified_client,
    );

    let request = Request::builder()
        .method("PUT")
        .header("Authorization", format!("Bearer {}", user_token))
        .uri(catenate_url_path(
            base,
            "/document-storage/json/2/upload/update-status",
        )?)
        .body(Body::from(serde_json::to_string(&[&req])?))?;
    let response = client.request(request).await?;

    if !response.status().is_success() {
        return Err(format!("API:UpdateStatus:{:?}", response).into());
    }

    let ret = hoover_body_to_vec(response.into_body()).await?;
    let ret: Vec<UpdateStatusResponse> = serde_json::from_slice(&ret)?;
    if ret.len() != 1 {
        return Err(format!("API:UpdateStatus:{} responses", ret.len()).into());
    }
    let ret = &ret[0];
    if !ret.success() {
        return Err(format!("API:UpdateStatus:{}", ret.message()).into());
    }

    Ok(())
}

pub async fn storage_upload_doc<C>(
    client: &Client<C, Body>,
    base: &Uri,
    user_token: &str,
    id: &str,
    version: usize,
    parent: &str,
    node_type: NodeType,
    bookmarked: bool,
    current_page: usize,
    name: &str,
    modified_client: &str,
    zipfile: Vec<u8>,
) -> GenericResult<usize>
where
    C: Connect + Sync + 'static,
{
    let req = UploadRequestRequest::new(id, parent, node_type, version);
    let request = Request::builder()
        .method("PUT")
        .header("Authorization", format!("Bearer {}", user_token))
        .uri(catenate_url_path(
            base,
            "/document-storage/json/2/upload/request",
        )?)
        .body(Body::from(serde_json::to_string(&[&req])?))?;
    let response = client.request(request).await?;

    if !response.status().is_success() {
        return Err(format!("API:UploadRequest:{:?}", response).into());
    }

    let ret = hoover_body_to_vec(response.into_body()).await?;
    let ret: Vec<UploadRequestResponse> = serde_json::from_slice(&ret)?;
    if ret.len() != 1 {
        return Err(format!("API:UpdateStatus:{} responses", ret.len()).into());
    }
    let ret = &ret[0];
    if !ret.success() {
        return Err(format!("API:UploadRequest:{}", ret.message()).into());
    }

    // We succeeded in requesting the upload, so put the blob
    let lenzip = zipfile.len();
    let request = Request::builder()
        .method("PUT")
        .uri(ret.blob_url_put())
        .body(Body::from(zipfile))?;
    let response = client.request(request).await?;

    if !response.status().is_success() {
        return Err(format!("API:UploadRequestBlobPut:{:?}", response).into());
    }

    // Now complete the update

    storage_update_doc(
        client,
        base,
        user_token,
        id,
        version,
        parent,
        node_type,
        bookmarked,
        current_page,
        name,
        modified_client,
    )
    .await?;

    Ok(lenzip)
}
