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

fn render_token(token: &str) -> Result<String> {
    let jwt = jsonwebtoken::dangerous_unsafe_decode::<serde_json::Value>(token)?;
    Ok(format!("{:#}", jwt.claims))
}

async fn discover_storage_base(opt: &Options, user_token: &str) -> Result<hyper::Uri> {
    let base_uri = Uri::from_str(&opt.discovery_server)?;
    let client = https_capable_client();
    let host = llapi::discover_storage_service(&client, &base_uri, user_token).await?;
    Ok(Uri::from_str(&format!("https://{}/", host))?)
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

fn print_docs(docs: &[api::DocsResponse], container: &str, prefix: usize) {
    for doc in docs.iter().filter(|d| d.parent() == container) {
        for _ in 0..prefix {
            print!("| ");
        }
        let container = doc.node_type() == api::NodeType::CollectionType;
        if container {
            print!("+-📁");
        } else {
            print!("+-🗎");
        }
        println!(" {} ({})", doc.name(), doc.id());
        if container {
            print_docs(docs, doc.id(), prefix + 1);
        }
    }
}

async fn list_server(opt: &Options) -> Result<()> {
    let user_token = acquire_user_token(opt).await?;
    let storage_base_uri = discover_storage_base(opt, &user_token).await?;
    let client = https_capable_client();
    let docs = llapi::storage_fetch_all_docs(&client, &storage_base_uri, &user_token).await?;
    print_docs(&docs, "", 0);
    Ok(())
}

async fn show_tokens(opt: &Options) -> Result<()> {
    println!("Device token:\n{}", render_token(&opt.device_token)?);
    let user_token = acquire_user_token(opt).await?;
    println!("User token:\n{}", render_token(&user_token)?);
    Ok(())
}

async fn fetch_blob(opt: &Options) -> Result<()> {
    let user_token = acquire_user_token(opt).await?;
    let storage_base_uri = discover_storage_base(opt, &user_token).await?;
    let client = https_capable_client();
    let (id, out) = match &opt.cmd {
        Command::FetchBlob { id, out } => (id, out),
        _ => unreachable!(),
    };
    use std::fs::File;
    use std::io::BufWriter;
    let mut outbuf = BufWriter::new(File::create(out)?);
    println!(
        "Written {} bytes",
        llapi::storage_fetch_blob(&client, &storage_base_uri, &user_token, id, &mut outbuf).await?
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Options::get();
    match &opt.cmd {
        Command::Register { .. } => register_device(&opt).await,
        Command::ListServer => list_server(&opt).await,
        Command::ShowTokens => show_tokens(&opt).await,
        Command::FetchBlob { .. } => fetch_blob(&opt).await,
    }
}
