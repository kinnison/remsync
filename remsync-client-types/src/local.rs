//! Types for the local disk format (see on-disk.md)

use remsync_api_types::NodeType;
use serde::{Deserialize, Serialize};

/// Metadata held locally for a node
#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataFile {
    /// The parent of the node, or "" for a top-level node
    parent: String,
    /// The type of this node
    #[serde(rename = "type")]
    node_type: NodeType,
    /// The version of the node
    version: usize,
    /// The name of the node
    #[serde(rename = "visibleName")]
    name: String,
    /// Whether or not the node is bookmarked
    #[serde(rename = "pinned")]
    bookmarked: bool,
    /// Whether or not this node was ever synced
    synced: bool,
    /// When this node was last modified locally.
    /// This is represented as a time_t in millisecond precision,
    /// rendered as a string.
    #[serde(rename = "lastModified")]
    last_modified: String,
    /// Whether this node has had its metadata changed since last sync
    #[serde(rename = "metadatamodified")]
    metadata_modified: bool,
    /// Whether this node has had its document data changed since last sync
    modified: bool,
    /// Whether this node was deleted since last sync
    deleted: bool,
}

impl MetadataFile {
    /// Create a new MetadataFile object.
    ///
    /// A new instance sets only the node type, parent, and name.
    /// Everything else is set as though the node were just created
    /// on a client. This means that it's ready to be sync'd.
    pub fn new(node_type: NodeType, parent: &str, name: &str) -> Self {
        Self {
            parent: parent.to_owned(),
            node_type,
            version: 0, // Zero until synced for the first time
            name: name.to_owned(),
            bookmarked: false,
            synced: false,
            last_modified: MetadataFile::get_now(),
            metadata_modified: false,
            modified: false,
            deleted: false,
        }
    }

    pub fn parent(&self) -> &str {
        &self.parent
    }

    pub fn set_parent(&mut self, parent: &str) {
        if self.parent != parent {
            self.parent = parent.to_owned();
            self.set_metadata_modified();
        }
    }

    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    pub fn version(&self) -> usize {
        self.version
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        if self.name != name {
            self.name = name.to_owned();
            self.set_metadata_modified();
        }
    }

    pub fn bookmarked(&self) -> bool {
        self.bookmarked
    }

    pub fn set_bookmarked(&mut self, bookmarked: bool) {
        if self.bookmarked != bookmarked {
            self.bookmarked = bookmarked;
            self.set_metadata_modified();
        }
    }

    pub fn synced(&self) -> bool {
        self.synced
    }

    pub fn last_modified(&self) -> &str {
        &self.last_modified
    }

    pub fn metadata_modified(&self) -> bool {
        self.metadata_modified
    }

    pub fn modified(&self) -> bool {
        self.modified
    }

    pub fn deleted(&self) -> bool {
        self.deleted
    }

    pub fn delete_node(&mut self) {
        if !self.deleted {
            self.deleted = true;
            self.set_metadata_modified();
        }
    }

    pub fn set_modified(&mut self) {
        self.modified = true;
        self.last_modified = MetadataFile::get_now();
    }

    fn set_metadata_modified(&mut self) {
        self.metadata_modified = true;
        self.last_modified = MetadataFile::get_now();
    }

    /// Get now, as a time_t in millisecond precision, as a string
    fn get_now() -> String {
        use std::time;
        let now = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("We're before epoch?")
            .as_millis();
        format!("{}", now)
    }
}

/// A (deliberately) incomplete implementation of the `.content` file
#[derive(Debug, Serialize, Deserialize)]
pub struct ContentFile {
    /// The number of pages in the document
    #[serde(rename = "pageCount")]
    page_count: usize,
}

impl ContentFile {
    pub fn page_count(&self) -> usize {
        self.page_count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{from_str, from_value, to_value, Value};

    fn round_trip<'de, T>(content: &'de str)
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let v: Value = from_str(content).expect("Unable to parse");
        let obj: T = from_value(v.clone()).expect("Unable to construct");
        let v2: Value = to_value(obj).expect("Unable to reserialize");
        assert_eq!(v, v2);
    }

    #[test]
    fn metadata() {
        round_trip::<MetadataFile>(
            r#"
{
    "deleted": false,
    "lastModified": "1567360844532",
    "metadatamodified": false,
    "modified": false,
    "parent": "15af7606-da75-4465-a769-5fb3c9a1ecdb",
    "pinned": false,
    "synced": true,
    "type": "DocumentType",
    "version": 11,
    "visibleName": "WiFi and USB local sync"
}
"#,
        )
    }
}
