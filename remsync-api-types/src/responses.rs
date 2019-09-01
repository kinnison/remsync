//! API types for the reMarkable Sync API response bodies

pub mod discovery {
    //! Discovery API responses

    use serde::{Deserialize, Serialize};
    /// A response from a service discovery request
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DiscoveryResponse {
        #[serde(rename = "Status")]
        /// The status of the response, almost universally "OK"
        status: String,
        #[serde(rename = "Host")]
        /// The hostname discovered for the given service
        /// Note: This is only a hostname, port will always be 443 for https
        host: String,
    }

    impl DiscoveryResponse {
        /// Create a new DiscoveryResponse object
        ///
        /// ```
        /// # use remsync_api_types::DiscoveryResponse;
        /// let discovery = DiscoveryResponse::new("OK", "foo.com");
        /// ```
        pub fn new(status: &str, host: &str) -> Self {
            Self {
                status: status.to_owned(),
                host: host.to_owned(),
            }
        }

        /// Look up the status of a DiscoveryResponse object
        ///
        /// ```
        /// # use remsync_api_types::DiscoveryResponse;
        /// # let discovery = DiscoveryResponse::new("OK", "foo.com");
        /// assert_eq!(discovery.status(), "OK");
        /// ```
        pub fn status(&self) -> &str {
            &self.status
        }

        /// Look up the status of a DiscoveryResponse object
        ///
        /// ```
        /// # use remsync_api_types::DiscoveryResponse;
        /// # let discovery = DiscoveryResponse::new("OK", "foo.com");
        /// assert_eq!(discovery.host(), "foo.com");
        /// ```
        pub fn host(&self) -> &str {
            &self.host
        }

        /// Pull the host out of this DiscoveryResponse
        ///
        /// ```
        /// # use remsync_api_types::DiscoveryResponse;
        /// # let discovery = DiscoveryResponse::new("OK", "foo.com");
        /// assert_eq!(discovery.into_host(), "foo.com");
        /// ```
        pub fn into_host(self) -> String {
            self.host
        }
    }
}

pub mod docs {
    //! Docs response types

    use serde::{Deserialize, Serialize};

    use crate::NodeType;

    /// A response to a request to the docs api
    ///
    /// This always comes back in a list, and the blob URL stuff may not be
    /// valid unless requested.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DocsResponse {
        #[serde(rename = "Success")]
        /// Whether this docs request was successful
        success: bool,
        #[serde(rename = "Message")]
        /// If the request was unsuccessful, this is the reason
        message: String,
        #[serde(rename = "ID")]
        /// The ID of this node
        id: String,
        #[serde(rename = "Version")]
        /// The version of this node
        version: usize,
        #[serde(rename = "BlobURLGet")]
        /// The URL to `GET` to fetch the blob for this node.
        /// This will be the empty string unless it was requested
        blob_url_get: String,
        #[serde(rename = "BlobURLGetExpires")]
        /// When the blob URL will expire
        blob_url_get_expires: String,
        #[serde(rename = "ModifiedClient")]
        /// When this node was last modified on a client device
        modified_client: String,
        #[serde(rename = "Type")]
        /// The type of this node
        node_type: NodeType,
        #[serde(rename = "VissibleName")]
        /// The name of this node
        name: String,
        #[serde(rename = "CurrentPage")]
        /// The current page for this node
        current_page: usize,
        #[serde(rename = "Bookmarked")]
        /// Whether or not this node is bookmarked
        bookmarked: bool,
        #[serde(rename = "Parent")]
        /// The parent of this node
        parent: String,
    }

    impl DocsResponse {
        /// Create a new DocsResponse object
        ///
        /// These are returned in lists from docs requests.  The page is only
        /// valid for documents.  It's zero for collection nodes.
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// let doc = DocsResponse::new(
        ///     true, "", "some-id", 4, "some-url", "some-expiry",
        ///     "some-modified", NodeType::DocumentType, "some-name",
        ///     71, true, "some-parent-id"
        /// );
        /// ```
        pub fn new(
            success: bool,
            message: &str,
            id: &str,
            version: usize,
            blob_url_get: &str,
            blob_url_get_expires: &str,
            modified_client: &str,
            node_type: NodeType,
            name: &str,
            current_page: usize,
            bookmarked: bool,
            parent: &str,
        ) -> Self {
            Self {
                success,
                message: message.to_owned(),
                id: id.to_owned(),
                version,
                blob_url_get: blob_url_get.to_owned(),
                blob_url_get_expires: blob_url_get_expires.to_owned(),
                modified_client: modified_client.to_owned(),
                node_type: node_type,
                name: name.to_owned(),
                current_page,
                bookmarked,
                parent: parent.to_owned(),
            }
        }

        /// Retrieve the success of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.success(), true);
        /// ```
        pub fn success(&self) -> bool {
            self.success
        }

        /// Retrieve the message of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.message(), "");
        /// ```
        pub fn message(&self) -> &str {
            &self.message
        }

        /// Retrieve the ID of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.id(), "some-id");
        /// ```
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Retrieve the version of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.version(), 4);
        /// ```
        pub fn version(&self) -> usize {
            self.version
        }

        /// Retrieve the BlobURLGet of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.blob_url_get(), "some-url");
        /// ```
        pub fn blob_url_get(&self) -> &str {
            &self.blob_url_get
        }

        /// Retrieve the BlobURLGetExpires of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.blob_url_get_expires(), "some-expiry");
        /// ```
        pub fn blob_url_get_expires(&self) -> &str {
            &self.blob_url_get_expires
        }

        /// Retrieve the client modification time of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.modified_client(), "some-modified");
        /// ```
        pub fn modified_client(&self) -> &str {
            &self.modified_client
        }

        /// Retrieve the node type of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.node_type(), NodeType::DocumentType);
        /// ```
        pub fn node_type(&self) -> NodeType {
            self.node_type
        }

        /// Retrieve the name of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.name(), "some-name");
        /// ```
        pub fn name(&self) -> &str {
            &self.name
        }

        /// Retrieve the current page of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.current_page(), 71);
        /// ```
        pub fn current_page(&self) -> usize {
            self.current_page
        }

        /// Retrieve the bookmarked status of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.bookmarked(), true);
        /// ```
        pub fn bookmarked(&self) -> bool {
            self.bookmarked
        }

        /// Retrieve the parent ID of a DocsResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DocsResponse};
        /// # let doc = DocsResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// #     "some-modified", NodeType::DocumentType, "some-name",
        /// #     71, true, "some-parent-id"
        /// # );
        /// assert_eq!(doc.parent(), "some-parent-id");
        /// ```
        pub fn parent(&self) -> &str {
            &self.parent
        }
    }
}

pub mod upload {
    //! Response types for uploads
    use serde::{Deserialize, Serialize};

    /// A response to an UploadRequestRequest
    ///
    /// It's worth noting that these always come in lists even though the
    /// clients seem to only ever send one at a time
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UploadRequestResponse {
        #[serde(rename = "Success")]
        /// Whether this upload request was successful
        success: bool,
        #[serde(rename = "Message")]
        /// If the request was unsuccessful, this is the reason
        message: String,
        #[serde(rename = "ID")]
        /// The ID of this node
        id: String,
        #[serde(rename = "Version")]
        /// The version of this node
        version: usize,
        #[serde(rename = "BlobURLPut")]
        /// The URL to `PUT` to set the blob for this node.
        blob_url_put: String,
        #[serde(rename = "BlobURLPutExpires")]
        /// When the blob URL will expire
        blob_url_put_expires: String,
    }

    impl UploadRequestResponse {
        /// Create a new UploadRequestResponse object
        ///
        /// These are returned in lists from upload requests.
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UploadRequestResponse};
        /// let upload = UploadRequestResponse::new(
        ///     true, "", "some-id", 4, "some-url", "some-expiry",
        /// );
        /// ```
        pub fn new(
            success: bool,
            message: &str,
            id: &str,
            version: usize,
            blob_url_put: &str,
            blob_url_put_expires: &str,
        ) -> Self {
            Self {
                success,
                message: message.to_owned(),
                id: id.to_owned(),
                version,
                blob_url_put: blob_url_put.to_owned(),
                blob_url_put_expires: blob_url_put_expires.to_owned(),
            }
        }

        /// Retrieve the success of an UploadRequestResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UploadRequestResponse};
        /// # let upload = UploadRequestResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// # );
        /// assert_eq!(upload.success(), true);
        /// ```
        pub fn success(&self) -> bool {
            self.success
        }

        /// Retrieve the success of an UploadRequestResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UploadRequestResponse};
        /// # let upload = UploadRequestResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// # );
        /// assert_eq!(upload.success(), true);
        /// ```
        pub fn message(&self) -> &str {
            &self.message
        }

        /// Retrieve the ID of an UploadRequestResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UploadRequestResponse};
        /// # let upload = UploadRequestResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// # );
        /// assert_eq!(upload.id(), "some-id");
        /// ```
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Retrieve the version of an UploadRequestResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UploadRequestResponse};
        /// # let upload = UploadRequestResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// # );
        /// assert_eq!(upload.version(), 4);
        /// ```
        pub fn version(&self) -> usize {
            self.version
        }

        /// Retrieve the PUT url of an UploadRequestResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UploadRequestResponse};
        /// # let upload = UploadRequestResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// # );
        /// assert_eq!(upload.blob_url_put(), "some-url");
        /// ```
        pub fn blob_url_put(&self) -> &str {
            &self.blob_url_put
        }

        /// Retrieve the expiry time of the PUT url of an UploadRequestResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UploadRequestResponse};
        /// # let upload = UploadRequestResponse::new(
        /// #     true, "", "some-id", 4, "some-url", "some-expiry",
        /// # );
        /// assert_eq!(upload.blob_url_put_expires(), "some-expiry");
        /// ```
        pub fn blob_url_put_expires(&self) -> &str {
            &self.blob_url_put_expires
        }
    }

    /// The response to an UpdateStatusRequest
    ///
    /// Note: this come in lists, despite the device only ever using single
    /// requests at a time
    ///
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UpdateStatusResponse {
        #[serde(rename = "Success")]
        /// Whether this upload request was successful
        success: bool,
        #[serde(rename = "Message")]
        /// If the request was unsuccessful, this is the reason
        message: String,
        #[serde(rename = "ID")]
        /// The ID of this node
        id: String,
        #[serde(rename = "Version")]
        /// The version of this node
        version: usize,
    }
    impl UpdateStatusResponse {
        /// Create a new UpdateStatusResponse object
        ///
        /// These are returned in lists from upload requests.
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UpdateStatusResponse};
        /// let update = UpdateStatusResponse::new(
        ///     true, "", "some-id", 4,
        /// );
        /// ```
        pub fn new(success: bool, message: &str, id: &str, version: usize) -> Self {
            Self {
                success,
                message: message.to_owned(),
                id: id.to_owned(),
                version,
            }
        }

        /// Retrieve the success of an UpdateStatusResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UpdateStatusResponse};
        /// # let update = UpdateStatusResponse::new(
        /// #     true, "", "some-id", 4,
        /// # );
        /// assert_eq!(update.success(), true);
        /// ```
        pub fn success(&self) -> bool {
            self.success
        }

        /// Retrieve the success of an UpdateStatusResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UpdateStatusResponse};
        /// # let update = UpdateStatusResponse::new(
        /// #     true, "", "some-id", 4,
        /// # );
        /// assert_eq!(update.success(), true);
        /// ```
        pub fn message(&self) -> &str {
            &self.message
        }

        /// Retrieve the ID of an UpdateStatusResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UpdateStatusResponse};
        /// # let update = UpdateStatusResponse::new(
        /// #     true, "", "some-id", 4,
        /// # );
        /// assert_eq!(update.id(), "some-id");
        /// ```
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Retrieve the version of an UpdateStatusResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, UpdateStatusResponse};
        /// # let update = UpdateStatusResponse::new(
        /// #     true, "", "some-id", 4,
        /// # );
        /// assert_eq!(update.version(), 4);
        /// ```
        pub fn version(&self) -> usize {
            self.version
        }
    }
}

pub mod delete {
    //! Deleting nodes

    use serde::{Deserialize, Serialize};
    /// The response to a DeleteRequest
    ///
    /// These always come in lists as do the requests, though devices only ever
    /// seem to submit one a a time in the lists.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeleteResponse {
        #[serde(rename = "Success")]
        /// Whether this upload request was successful
        success: bool,
        #[serde(rename = "Message")]
        /// If the request was unsuccessful, this is the reason
        message: String,
        #[serde(rename = "ID")]
        /// The ID of this node
        id: String,
        #[serde(rename = "Version")]
        /// The version of this node
        version: usize,
    }
    impl DeleteResponse {
        /// Create a new DeleteResponse object
        ///
        /// These are returned in lists from upload requests.
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DeleteResponse};
        /// let delete = DeleteResponse::new(
        ///     true, "", "some-id", 4,
        /// );
        /// ```
        pub fn new(success: bool, message: &str, id: &str, version: usize) -> Self {
            Self {
                success,
                message: message.to_owned(),
                id: id.to_owned(),
                version,
            }
        }

        /// Retrieve the success of an DeleteResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DeleteResponse};
        /// # let delete = DeleteResponse::new(
        /// #     true, "", "some-id", 4,
        /// # );
        /// assert_eq!(delete.success(), true);
        /// ```
        pub fn success(&self) -> bool {
            self.success
        }

        /// Retrieve the success of an DeleteResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DeleteResponse};
        /// # let delete = DeleteResponse::new(
        /// #     true, "", "some-id", 4,
        /// # );
        /// assert_eq!(delete.success(), true);
        /// ```
        pub fn message(&self) -> &str {
            &self.message
        }

        /// Retrieve the ID of an DeleteResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DeleteResponse};
        /// # let delete = DeleteResponse::new(
        /// #     true, "", "some-id", 4,
        /// # );
        /// assert_eq!(delete.id(), "some-id");
        /// ```
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Retrieve the version of an DeleteResponse object
        ///
        /// ```
        /// # use remsync_api_types::{NodeType, DeleteResponse};
        /// # let delete = DeleteResponse::new(
        /// #     true, "", "some-id", 4,
        /// # );
        /// assert_eq!(delete.version(), 4);
        /// ```
        pub fn version(&self) -> usize {
            self.version
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use serde_json::{from_str, to_string_pretty};

    fn round_trip<'de, T>(content: &'de str)
    where
        T: serde::Serialize + serde::Deserialize<'de>,
    {
        let obj: T = from_str(content.trim()).expect("Unable to parse");
        let s = to_string_pretty(&obj).expect("Unable to reserialize");
        assert_eq!(content.trim(), s.trim());
    }

    #[test]
    fn discovery_response() {
        round_trip::<DiscoveryResponse>(
            r#"
{
  "Status": "OK",
  "Host": "document-storage-production-dot-remarkable-production.appspot.com"
}
"#,
        )
    }

    #[test]
    fn docs_response() {
        // It's useful to note that docs always returns a list, even for a single
        // requested document
        round_trip::<Vec<DocsResponse>>(
            r#"
[
  {
    "Success": true,
    "Message": "",
    "ID": "20d581ff-4507-4fc3-a7fb-07daf6c4bc86",
    "Version": 33,
    "BlobURLGet": "",
    "BlobURLGetExpires": "0001-01-01T00:00:00Z",
    "ModifiedClient": "2019-08-31T14:34:43.664664Z",
    "Type": "DocumentType",
    "VissibleName": "Quick sheets",
    "CurrentPage": 2,
    "Bookmarked": false,
    "Parent": ""
  },
  {
    "Success": true,
    "Message": "",
    "ID": "67a6f6c7-d4aa-430e-8c27-f40af171b135",
    "Version": 13,
    "BlobURLGet": "",
    "BlobURLGetExpires": "0001-01-01T00:00:00Z",
    "ModifiedClient": "2019-08-30T12:45:51Z",
    "Type": "DocumentType",
    "VissibleName": "Rust raytracing",
    "CurrentPage": 2,
    "Bookmarked": false,
    "Parent": ""
  }
]
"#,
        )
    }

    #[test]
    fn upload_request() {
        round_trip::<Vec<UploadRequestResponse>>(
            r#"
[
  {
    "Success": true,
    "Message": "",
    "ID": "092fd1cc-df38-4fc5-8633-3a8a15a2a316",
    "Version": 3,
    "BlobURLPut": "SOMEHUGEURLWHICHEXPIRESASPERBELOW",
    "BlobURLPutExpires": "2019-08-31T15:52:52.186395996Z"
  }
]
"#,
        )
    }

    #[test]
    fn update_status() {
        round_trip::<Vec<UpdateStatusResponse>>(
            r#"
[
  {
    "Success": true,
    "Message": "",
    "ID": "092fd1cc-df38-4fc5-8633-3a8a15a2a316",
    "Version": 3
  }
]
"#,
        )
    }

    #[test]
    fn delete_response() {
        round_trip::<Vec<DeleteResponse>>(
            r#"
[
  {
    "Success": true,
    "Message": "",
    "ID": "092fd1cc-df38-4fc5-8633-3a8a15a2a316",
    "Version": 3
  }
]
"#,
        )
    }
}
