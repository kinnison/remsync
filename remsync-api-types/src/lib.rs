//! API types for the reMarkable Sync API
//!

mod common;
mod notifications;
mod requests;
mod responses;
mod tokens;

// The basic concept is that we'll import these types at the top level
// rather than expecting users to use the deeper paths
pub use common::*;
pub use notifications::*;
pub use requests::auth::DeviceTokenRequest;
pub use requests::delete::DeleteRequest;
pub use requests::upload::{UpdateStatusRequest, UploadRequestRequest};
pub use responses::delete::DeleteResponse;
pub use responses::discovery::DiscoveryResponse;
pub use responses::docs::DocsResponse;
pub use responses::upload::{UpdateStatusResponse, UploadRequestResponse};
pub use tokens::Auth0Profile;
pub use tokens::DeviceToken;
pub use tokens::UserToken;
