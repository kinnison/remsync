//! types useful for dealing with server sync in a very dodgy manner

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{from_reader, to_writer_pretty};

use super::Result;
use remsync_api_types::DocsResponse;

#[derive(Debug)]
pub struct LocalState {
    base_path: PathBuf,
    docs: HashMap<String, DocsResponse>,
}

impl LocalState {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<LocalState> {
        let mut ret = Self {
            base_path: base_path.as_ref().to_owned(),
            docs: HashMap::new(),
        };

        ret.load_data()?;

        Ok(ret)
    }

    pub fn count_docs(&self) -> usize {
        self.docs.len()
    }

    pub fn remove_not_listed(&mut self, server_uuids: &HashSet<String>) -> Result<()> {
        let client_uuids: HashSet<String> = self.docs.keys().map(|k| k.to_owned()).collect();
        let to_delete = client_uuids.difference(server_uuids);
        for k in to_delete {
            println!("Removing local doc {}", k);
            self.docs.remove(k);
            fs::remove_file(self.doc_path(k))?;
            fs::remove_file(self.zip_path(k))?;
        }
        Ok(())
    }

    pub fn get_not_listed(&self, server_uuids: &HashSet<String>) -> HashSet<String> {
        let mut ret = HashSet::new();
        for server_uuid in server_uuids.iter() {
            if !self.docs.contains_key(server_uuid) {
                ret.insert(server_uuid.to_owned());
            }
        }
        ret
    }

    pub fn find_changed(&self, docs: &HashMap<String, DocsResponse>) -> Result<HashSet<String>> {
        let mut ret = HashSet::new();

        // At this point we know that we contain some subset of the docs response
        // so anything in docs which isn't known to us needs a fetch, and anything
        // which is known to us but isn't the same version needs a fetch
        for doc in docs.values() {
            match self.docs.get(doc.id()) {
                Some(localdoc) => {
                    if localdoc.version() != doc.version() {
                        ret.insert(doc.id().to_owned());
                    }
                }
                None => {
                    ret.insert(doc.id().to_owned());
                }
            }
        }

        Ok(ret)
    }

    pub fn find_locally_changed(
        &self,
        docs: &HashMap<String, DocsResponse>,
    ) -> Result<HashSet<String>> {
        let mut ret = HashSet::new();

        // We contain a superset of the non-deleted values in docs
        // as such we care about returning the set of things we have
        // which docs does not, and which we have where docs differs
        for doc in self.docs.values() {
            match docs.get(doc.id()) {
                Some(serverdoc) => {
                    if serverdoc.version() != doc.version() {
                        ret.insert(doc.id().to_owned());
                    }
                }
                None => {
                    ret.insert(doc.id().to_owned());
                }
            }
        }

        Ok(ret)
    }

    pub fn download_path(&self, uuid: &str) -> PathBuf {
        let mut ret = self.zip_path(uuid);
        ret.set_extension(".zip.tmp");
        ret
    }

    pub fn adopt_doc(&mut self, doc: &DocsResponse, zip: &Path) -> Result<()> {
        let outf = fs::File::create(self.doc_path(doc.id()))?;
        to_writer_pretty(outf, doc)?;
        fs::rename(zip, self.zip_path(doc.id()))?;
        self.docs.insert(doc.id().to_owned(), doc.clone());
        Ok(())
    }

    pub fn get_doc(&self, uuid: &str) -> Option<&DocsResponse> {
        self.docs.get(uuid)
    }

    // Private stuff here down

    fn doc_path(&self, uuid: &str) -> PathBuf {
        let mut ret = self.base_path.clone();
        ret.push(uuid);
        ret.set_extension("doc");
        ret
    }

    pub fn zip_path(&self, uuid: &str) -> PathBuf {
        let mut ret = self.base_path.clone();
        ret.push(uuid);
        ret.set_extension("zip");
        ret
    }

    fn load_data(&mut self) -> Result<()> {
        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?.path();
            if let Some(ext) = entry.extension() {
                if ext == "doc" {
                    self.load_doc(&entry)?;
                }
            }
        }
        Ok(())
    }

    fn load_doc(&mut self, entry: &Path) -> Result<()> {
        let uuid = entry
            .file_stem()
            .ok_or("Odd, no UUID")?
            .to_str()
            .ok_or("Odd, UUID not safe")?;
        let file = fs::File::open(entry)?;
        let doc: DocsResponse = from_reader(file)?;
        self.docs.insert(uuid.to_owned(), doc);
        Ok(())
    }
}
