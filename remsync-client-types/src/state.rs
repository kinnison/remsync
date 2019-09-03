//! A Client state

use std::path::{Path, PathBuf};
// TODO: Do a better job of error handling
use crate::local::MetadataFile;
use serde_json::from_reader;
use std::collections::HashMap;
use std::error::Error;

/// The embodiment of a client state
#[derive(Debug)]
pub struct ClientState {
    base_path: PathBuf,
    nodes: HashMap<String, MetadataFile>,
}

impl ClientState {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self {
            base_path: base_path.as_ref().to_owned(),
            nodes: HashMap::new(),
        };

        ret.load_everything()?;

        Ok(ret)
    }

    fn load_everything(&mut self) -> Result<(), Box<dyn Error>> {
        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let full_path = entry.path();
            let basename = full_path.file_stem().ok_or("No file stem?")?;
            let basename = basename.to_str().ok_or("Odd, UUIDs are ASCII")?;
            let node_id = basename.to_owned();
            let metadata: MetadataFile = from_reader(std::fs::File::open(full_path)?)?;
            self.nodes.insert(node_id, metadata);
        }
        Ok(())
    }

    pub fn node_metadata(&self, node: &str) -> Option<&MetadataFile> {
        self.nodes.get(node)
    }

    pub fn node_metadata_mut(&mut self, node: &str) -> Option<&mut MetadataFile> {
        self.nodes.get_mut(node)
    }
}
