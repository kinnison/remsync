///! Common types used in both requests and responses
use serde::{Deserialize, Serialize};

/// A node's type
///
/// Every node that exists in the system has a type.
///
/// Documents, be they epubs, notebooks, or pdfs, are one type,
/// and folders are another.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    /// Folders have this type
    CollectionType,
    /// Documents have this type
    DocumentType,
}
