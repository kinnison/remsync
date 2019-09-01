//! CLI definitions for remsync

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "remsync", about = "reMarkable sync tool")]
pub struct Options {
    #[structopt(
        long = "auth-server",
        name = "authentication server",
        default_value = "https://my.remarkable.com/"
    )]
    /// Authentication server to use to acquire bearer tokens
    pub auth_server: String,
    #[structopt(
        long = "device-token",
        name = "device token",
        env = "REMSYNC_DEVICE_TOKEN",
        hide_env_values = true
    )]
    pub device_token: String,
    #[structopt(subcommand)]
    pub cmd: Command,
}

impl Options {
    pub fn get() -> Options {
        Self::from_args()
    }
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "register")]
    /// Register a device and acquire a device bearer token
    Register {
        /// The code to register with
        code: String,
        /// The device descriptor
        #[structopt(long = "desc", name = "device-desc", default_value = "desktop-linux")]
        device_desc: String,
        /// The device ID, if not specified, a UUID will be generated
        #[structopt(long = "id", name = "device-id")]
        device_id: Option<String>,
    },
    #[structopt(name = "ls")]
    /// List the contents of the server
    ListServer,
}
