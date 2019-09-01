//! Utility functions for internal use only.

use hyper::Body;
use hyper::Uri;

use crate::GenericResult;

pub fn catenate_url_path(base: &Uri, path: &str) -> GenericResult<Uri> {
    let mut builder = Uri::builder();

    // Copy the scheme
    if let Some(scheme) = base.scheme_str() {
        builder.scheme(scheme);
    }
    // And the authority
    if let Some(authority) = base.authority_part() {
        builder.authority(authority.as_str());
    }

    // But catenate the path
    let mut base_path = base.path();

    if base_path.ends_with("/") {
        base_path = &base_path[..base_path.len() - 1];
    }

    builder.path_and_query(&*format!("{}{}", base_path, path));

    Ok(builder.build()?)
}

pub async fn hoover_body_to_vec(mut body: Body) -> GenericResult<Vec<u8>> {
    let mut ret = Vec::new();
    while let Some(next) = body.next().await {
        let chunk = next?;
        ret.extend_from_slice(&chunk);
    }
    Ok(ret)
}
