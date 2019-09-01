//! API types for the reMarkable Sync API request bodies

pub mod auth {
    //! Authentication types

    use serde::{Deserialize, Serialize};

    /// Establish new device bearer token
    ///
    /// This request is sent as a body when requesting a new device bearer
    /// token.  The response is a JWT as a plain body.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeviceTokenRequest {
        /// The OTP code for the device token request.
        code: String,
        #[serde(rename = "deviceDesc")]
        /// Device descriptor such as `desktop-windows`
        device_desc: String,
        #[serde(rename = "deviceID")]
        /// Device ID - a UUID as a string, or perhaps a serial number.  Unclear
        device_id: String,
    }

    impl DeviceTokenRequest {
        /// Create a new request
        ///
        /// ```
        /// # use resync_api_types::DeviceTokenRequest;
        ///
        /// let token = DeviceTokenRequest::new("abcdefg",
        ///                                     "device-resync",
        ///                                     "a1acac82-cc20-11e9-bd13-c3262b9895b0");
        /// ```
        pub fn new(code: &str, device_desc: &str, device_id: &str) -> Self {
            Self {
                code: code.to_owned(),
                device_desc: device_desc.to_owned(),
                device_id: device_id.to_owned(),
            }
        }

        /// Retrieve the code from a request
        ///
        /// ```
        /// # use resync_api_types::DeviceTokenRequest;
        /// # let token = DeviceTokenRequest::new("abcdefg",
        /// #                                     "device-resync",
        /// #                                     "a1acac82-cc20-11e9-bd13-c3262b9895b0");
        /// assert_eq!(token.code(), "abcdefg");
        /// ```
        pub fn code(&self) -> &str {
            &self.code
        }

        /// Retrieve the device descriptor from a request
        ///
        /// ```
        /// # use resync_api_types::DeviceTokenRequest;
        /// # let token = DeviceTokenRequest::new("abcdefg",
        /// #                                     "device-resync",
        /// #                                     "a1acac82-cc20-11e9-bd13-c3262b9895b0");
        /// assert_eq!(token.device_desc(), "device-resync");
        /// ```
        pub fn device_desc(&self) -> &str {
            &self.device_desc
        }

        /// Retrieve the device ID from a request
        ///
        /// ```
        /// # use resync_api_types::DeviceTokenRequest;
        /// # let token = DeviceTokenRequest::new("abcdefg",
        /// #                                     "device-resync",
        /// #                                     "a1acac82-cc20-11e9-bd13-c3262b9895b0");
        /// assert_eq!(token.device_id(), "a1acac82-cc20-11e9-bd13-c3262b9895b0");
        /// ```
        pub fn device_id(&self) -> &str {
            &self.device_id
        }
    }
}

pub mod upload {
    //! Requests for document/node uploading

    use crate::NodeType;
    use serde::{Deserialize, Serialize};

    /// A request to be permitted to upload a node
    ///
    /// This request is sent when a client wishes to upload a node to the API
    /// and needs to be provided with a blob put URL etc.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UploadRequestRequest {
        #[serde(rename = "ID")]
        /// The ID of the node (UUID)
        id: String,
        #[serde(rename = "Parent")]
        /// The ID of the node's parent (UUID)
        parent: String,
        #[serde(rename = "Type")]
        /// The type of this node
        node_type: NodeType,
        #[serde(rename = "Version")]
        /// The version of this node
        version: usize,
    }

    impl UploadRequestRequest {
        /// Create a new UploadRequestRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UploadRequestRequest;
        /// let upload = UploadRequestRequest::new("some-id",
        ///                                        "some-parent-id",
        ///                                        NodeType::CollectionType,
        ///                                        1);
        /// ```
        pub fn new(id: &str, parent: &str, node_type: NodeType, version: usize) -> Self {
            Self {
                id: id.to_owned(),
                parent: parent.to_owned(),
                node_type,
                version,
            }
        }

        /// Retrieve the ID of an UploadRequestRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UploadRequestRequest;
        /// # let upload = UploadRequestRequest::new("some-id",
        /// #                                        "some-parent-id",
        /// #                                        NodeType::CollectionType,
        /// #                                        1);
        /// assert_eq!(upload.id(), "some-id");
        /// ```
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Retrieve the Parent ID of an UploadRequestRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UploadRequestRequest;
        /// # let upload = UploadRequestRequest::new("some-id",
        /// #                                        "some-parent-id",
        /// #                                        NodeType::CollectionType,
        /// #                                        1);
        /// assert_eq!(upload.parent(), "some-parent-id");
        /// ```
        pub fn parent(&self) -> &str {
            &self.parent
        }

        /// Retrieve the node type of an UploadRequestRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UploadRequestRequest;
        /// # let upload = UploadRequestRequest::new("some-id",
        /// #                                        "some-parent-id",
        /// #                                        NodeType::CollectionType,
        /// #                                        1);
        /// assert_eq!(upload.node_type(), NodeType::CollectionType);
        /// ```
        pub fn node_type(&self) -> NodeType {
            self.node_type
        }

        /// Retrieve the version of an UploadRequestRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UploadRequestRequest;
        /// # let upload = UploadRequestRequest::new("some-id",
        /// #                                        "some-parent-id",
        /// #                                        NodeType::CollectionType,
        /// #                                        1);
        /// assert_eq!(upload.version(), 1);
        /// ```
        pub fn version(&self) -> usize {
            self.version
        }
    }

    /// A request to update the status of a node
    ///
    /// This is sent to update the metadata about a node.  It must match the
    /// id, parent, type, and version of the UploadRequestRequest associated
    /// with the update.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UpdateStatusRequest {
        #[serde(rename = "ID")]
        /// The ID of the node to update
        id: String,
        #[serde(rename = "Parent")]
        /// The Parent of that node
        parent: String,
        #[serde(rename = "Type")]
        /// The type of this node
        node_type: NodeType,
        #[serde(rename = "Version")]
        /// The version of this node
        version: usize,
        #[serde(rename = "Bookmarked")]
        /// Whether or not this node is bookmarked
        bookmarked: bool,
        #[serde(rename = "CurrentPage")]
        /// The current page number (0 based)
        current_page: usize,
        #[serde(rename = "VissibleName")]
        /// The name of the node
        name: String,
        #[serde(rename = "ModifiedClient")]
        /// When this node was last modified on a client
        modified_client: String,
    }

    impl UpdateStatusRequest {
        /// Create a new UpdateStatusRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UpdateStatusRequest;
        /// let upload = UpdateStatusRequest::new("some-id",
        ///                                       "some-parent-id",
        ///                                       NodeType::CollectionType,
        ///                                       1,
        ///                                       false,
        ///                                       0,
        ///                                       "My Nice Folder",
        ///                                       "2019-08-31T14:49:51.302302Z");
        /// ```
        pub fn new(
            id: &str,
            parent: &str,
            node_type: NodeType,
            version: usize,
            bookmarked: bool,
            current_page: usize,
            name: &str,
            modified_client: &str,
        ) -> Self {
            Self {
                id: id.to_owned(),
                parent: parent.to_owned(),
                node_type,
                version,
                bookmarked,
                current_page,
                name: name.to_owned(),
                modified_client: modified_client.to_owned(),
            }
        }

        /// Retrieve the ID of an UpdateStatusRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UpdateStatusRequest;
        /// # let upload = UpdateStatusRequest::new("some-id",
        /// #                                       "some-parent-id",
        /// #                                       NodeType::CollectionType,
        /// #                                       1,
        /// #                                       false,
        /// #                                       0,
        /// #                                       "My Nice Folder",
        /// #                                       "2019-08-31T14:49:51.302302Z");
        /// assert_eq!(upload.id(), "some-id");
        /// ```
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Retrieve the Parent ID of an UpdateStatusRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UpdateStatusRequest;
        /// # let upload = UpdateStatusRequest::new("some-id",
        /// #                                       "some-parent-id",
        /// #                                       NodeType::CollectionType,
        /// #                                       1,
        /// #                                       false,
        /// #                                       0,
        /// #                                       "My Nice Folder",
        /// #                                       "2019-08-31T14:49:51.302302Z");
        /// assert_eq!(upload.parent(), "some-parent-id");
        /// ```
        pub fn parent(&self) -> &str {
            &self.parent
        }

        /// Retrieve the node type of an UpdateStatusRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UpdateStatusRequest;
        /// # let upload = UpdateStatusRequest::new("some-id",
        /// #                                       "some-parent-id",
        /// #                                       NodeType::CollectionType,
        /// #                                       1,
        /// #                                       false,
        /// #                                       0,
        /// #                                       "My Nice Folder",
        /// #                                       "2019-08-31T14:49:51.302302Z");
        /// assert_eq!(upload.node_type(), NodeType::CollectionType);
        /// ```
        pub fn node_type(&self) -> NodeType {
            self.node_type
        }

        /// Retrieve the version of an UpdateStatusRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UpdateStatusRequest;
        /// # let upload = UpdateStatusRequest::new("some-id",
        /// #                                       "some-parent-id",
        /// #                                       NodeType::CollectionType,
        /// #                                       1,
        /// #                                       false,
        /// #                                       0,
        /// #                                       "My Nice Folder",
        /// #                                       "2019-08-31T14:49:51.302302Z");
        /// assert_eq!(upload.version(), 1);
        /// ```
        pub fn version(&self) -> usize {
            self.version
        }

        /// Retrieve the bookmark status of an UpdateStatusRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UpdateStatusRequest;
        /// # let upload = UpdateStatusRequest::new("some-id",
        /// #                                       "some-parent-id",
        /// #                                       NodeType::CollectionType,
        /// #                                       1,
        /// #                                       false,
        /// #                                       0,
        /// #                                       "My Nice Folder",
        /// #                                       "2019-08-31T14:49:51.302302Z");
        /// assert_eq!(upload.bookmarked(), false);
        /// ```
        pub fn bookmarked(&self) -> bool {
            self.bookmarked
        }

        /// Retrieve the current page of an UpdateStatusRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UpdateStatusRequest;
        /// # let upload = UpdateStatusRequest::new("some-id",
        /// #                                       "some-parent-id",
        /// #                                       NodeType::CollectionType,
        /// #                                       1,
        /// #                                       false,
        /// #                                       0,
        /// #                                       "My Nice Folder",
        /// #                                       "2019-08-31T14:49:51.302302Z");
        /// assert_eq!(upload.current_page(), 0);
        /// ```
        pub fn current_page(&self) -> usize {
            self.current_page
        }

        /// Retrieve the node name of an UpdateStatusRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UpdateStatusRequest;
        /// # let upload = UpdateStatusRequest::new("some-id",
        /// #                                       "some-parent-id",
        /// #                                       NodeType::CollectionType,
        /// #                                       1,
        /// #                                       false,
        /// #                                       0,
        /// #                                       "My Nice Folder",
        /// #                                       "2019-08-31T14:49:51.302302Z");
        /// assert_eq!(upload.name(), "My Nice Folder");
        /// ```
        pub fn name(&self) -> &str {
            &self.name
        }

        /// Retrieve the modification time of an UpdateStatusRequest
        ///
        /// ```
        /// # use resync_api_types::NodeType;
        /// # use resync_api_types::UpdateStatusRequest;
        /// # let upload = UpdateStatusRequest::new("some-id",
        /// #                                       "some-parent-id",
        /// #                                       NodeType::CollectionType,
        /// #                                       1,
        /// #                                       false,
        /// #                                       0,
        /// #                                       "My Nice Folder",
        /// #                                       "2019-08-31T14:49:51.302302Z");
        /// assert_eq!(upload.modified_client(), "2019-08-31T14:49:51.302302Z");
        /// ```
        pub fn modified_client(&self) -> &str {
            &self.modified_client
        }
    }
}

pub mod delete {
    //! Deletion types
    use serde::{Deserialize, Serialize};

    /// Request to delete a node from the API
    ///
    /// The ID and Version must match a node currently in the index for
    /// this to work.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeleteRequest {
        #[serde(rename = "ID")]
        /// The ID of the node to delete
        id: String,
        #[serde(rename = "Version")]
        /// The version of the node to delete
        version: usize,
    }

    impl DeleteRequest {
        /// Create a new DeleteRequest
        ///
        /// ```
        /// # use resync_api_types::DeleteRequest;
        /// let delete = DeleteRequest::new("some-id", 4);
        /// ```
        pub fn new(id: &str, version: usize) -> Self {
            Self {
                id: id.to_owned(),
                version,
            }
        }

        /// Retrieve the ID of a new DeleteRequest
        ///
        /// ```
        /// # use resync_api_types::DeleteRequest;
        /// # let delete = DeleteRequest::new("some-id", 4);
        /// assert_eq!(delete.id(), "some-id");
        /// ```
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Retrieve the version of a new DeleteRequest
        ///
        /// ```
        /// # use resync_api_types::DeleteRequest;
        /// # let delete = DeleteRequest::new("some-id", 4);
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
    fn auth_device() {
        round_trip::<DeviceTokenRequest>(
            r#"
{
  "code": "gliuqtne",
  "deviceDesc": "desktop-windows",
  "deviceID": "701c3752-1025-4770-af43-5ddcfa4dabb2"
}
"#,
        )
    }

    #[test]
    fn upload_request() {
        // It's worth noting that upload requests are always in lists
        round_trip::<Vec<UploadRequestRequest>>(
            r#"
[
  {
    "ID": "092fd1cc-df38-4fc5-8633-3a8a15a2a316",
    "Parent": "e0c1c79f-b491-45e7-a431-a46fe1ec8a66",
    "Type": "DocumentType",
    "Version": 3
  }
]
"#,
        );
    }

    #[test]
    fn update_status() {
        // It's worth noting that update status requests are always in lists
        round_trip::<Vec<UpdateStatusRequest>>(
            r#"
[
  {
    "ID": "092fd1cc-df38-4fc5-8633-3a8a15a2a316",
    "Parent": "e0c1c79f-b491-45e7-a431-a46fe1ec8a66",
    "Type": "DocumentType",
    "Version": 3,
    "Bookmarked": false,
    "CurrentPage": 2,
    "VissibleName": "WiFi and USB local sync",
    "ModifiedClient": "2019-08-31T14:49:51.302302Z"
  }
]
"#,
        );
    }

    #[test]
    fn delete_request() {
        // It's worth noting that delete requests are always in lists
        round_trip::<Vec<DeleteRequest>>(
            r#"
[
  {
    "ID": "5109f2cc-5559-4239-8364-dbd709ca9126",
    "Version": 1
  }
]
"#,
        )
    }
}
