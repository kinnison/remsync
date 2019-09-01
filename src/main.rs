type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

mod cli;
use cli::{Command, Options};
use remsync_api_client::hyper::{self, Uri};
use remsync_api_client::ll as llapi;
use remsync_api_types as api;
use std::str::FromStr;

fn random_uuid() -> String {
    format!("{}", uuid::Uuid::new_v4())
}

fn https_capable_client(
) -> hyper::Client<hyper_tls::HttpsConnector<hyper::client::connect::HttpConnector>> {
    let connector = hyper_tls::HttpsConnector::new().expect("Unable to construct connector");
    hyper::Client::builder().build(connector)
}

async fn register_device(opt: &Options) -> Result<()> {
    let base_uri = Uri::from_str(&opt.auth_server)?;

    let (code, desc, id) = match opt.cmd {
        Command::Register {
            ref code,
            ref device_desc,
            ref device_id,
        } => (code, device_desc, device_id),
        _ => unreachable!(),
    };

    let id = if let Some(id) = id {
        id.clone()
    } else {
        random_uuid()
    };

    let req = api::DeviceTokenRequest::new(&code, &desc, &id);

    let client = https_capable_client();
    let token = llapi::auth_get_device_bearer(&client, &base_uri, &req).await?;

    println!("New device bearer: {}", token);

    Ok(())
}

async fn acquire_user_token(opt: &Options) -> Result<String> {
    let base_uri = Uri::from_str(&opt.auth_server)?;
    let client = https_capable_client();
    Ok(llapi::auth_get_user_bearer(&client, &base_uri, &opt.device_token).await?)
}

async fn list_server(opt: &Options) -> Result<()> {
    let user_token = acquire_user_token(opt).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Options::get();
    match &opt.cmd {
        Command::Register { .. } => register_device(&opt).await,
        Command::ListServer => list_server(&opt).await,
    }
}
