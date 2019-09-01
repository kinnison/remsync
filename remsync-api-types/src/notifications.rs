//! Types for the Notifications

use crate::NodeType;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The type of a notification event
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum NotificationEventType {
    /// A document/collection was added/modified
    DocAdded,
    /// A document/collection was removed
    DocDeleted,
}

/// Attributes for a notification message.
///
/// The attributes actually carry all the useful data about a notification event.
/// For example, the device which made the change, and which node it happened to.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationMessageAttributes {
    #[serde(rename = "auth0UserID")]
    /// The User ID of the actor which made the change
    auth0_user_id: String,
    /// Whether or not the node was/is bookmarked
    #[serde(
        deserialize_with = "bool_from_string",
        serialize_with = "display_to_string"
    )]
    bookmarked: bool,
    /// The event kind
    event: NotificationEventType,
    /// The ID of the node the event is about
    id: String,
    /// The parent of the node the event is about
    parent: String,
    #[serde(rename = "sourceDeviceDesc")]
    /// The device descriptor registered to the user ID which made the change
    source_device_desc: String,
    #[serde(rename = "sourceDeviceID")]
    /// The device ID registered to the user ID which made the change
    source_device_id: String,
    #[serde(rename = "type")]
    /// The type of the node which was changed
    node_type: NodeType,
    /// The version of the node which was changed
    #[serde(
        deserialize_with = "usize_from_string",
        serialize_with = "display_to_string"
    )]
    version: usize,
    #[serde(rename = "vissibleName")]
    /// The name of the node after the change
    name: String,
}

impl NotificationMessageAttributes {
    /// Create a new notification message attributes object.
    ///
    /// The attributes of the mesage tell us what happened, to what node.
    ///
    /// ```
    /// # use remsync_api_types::{NodeType, NotificationEventType, NotificationMessageAttributes};
    /// let attrs = NotificationMessageAttributes::new(
    ///     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    ///     "some-parent-id", "some-device-desc", "some-device-id",
    ///     NodeType::CollectionType, 7, "My Shiny Node"
    /// );
    /// ```
    pub fn new(
        auth0_user_id: &str,
        bookmarked: bool,
        event: NotificationEventType,
        id: &str,
        parent: &str,
        source_device_desc: &str,
        source_device_id: &str,
        node_type: NodeType,
        version: usize,
        name: &str,
    ) -> Self {
        Self {
            auth0_user_id: auth0_user_id.to_owned(),
            bookmarked,
            event,
            id: id.to_owned(),
            parent: parent.to_owned(),
            source_device_desc: source_device_desc.to_owned(),
            source_device_id: source_device_id.to_owned(),
            node_type,
            version,
            name: name.to_owned(),
        }
    }

    /// Retrieve the auth0 userid from an attributes object.
    ///
    /// ```
    /// # use remsync_api_types::{NodeType, NotificationEventType, NotificationMessageAttributes};
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// assert_eq!(attrs.auth0_user_id(), "some-user-id");
    /// ```
    pub fn auth0_user_id(&self) -> &str {
        &self.auth0_user_id
    }

    /// Retrieve the bookmarked status from an attributes object.
    ///
    /// ```
    /// # use remsync_api_types::{NodeType, NotificationEventType, NotificationMessageAttributes};
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// assert_eq!(attrs.bookmarked(), false);
    /// ```
    pub fn bookmarked(&self) -> bool {
        self.bookmarked
    }

    /// Retrieve the event type from an attributes object.
    ///
    /// ```
    /// # use remsync_api_types::{NodeType, NotificationEventType, NotificationMessageAttributes};
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// assert_eq!(attrs.event(), NotificationEventType::DocAdded);
    /// ```
    pub fn event(&self) -> NotificationEventType {
        self.event
    }

    /// Retrieve the node ID from an attributes object.
    ///
    /// ```
    /// # use remsync_api_types::{NodeType, NotificationEventType, NotificationMessageAttributes};
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// assert_eq!(attrs.id(), "some-id");
    /// ```
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Retrieve the node's parent ID from an attributes object.
    ///
    /// ```
    /// # use remsync_api_types::{NodeType, NotificationEventType, NotificationMessageAttributes};
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// assert_eq!(attrs.parent(), "some-parent-id");
    /// ```
    pub fn parent(&self) -> &str {
        &self.parent
    }

    /// Retrieve the author device descriptor from an attributes object.
    ///
    /// ```
    /// # use remsync_api_types::{NodeType, NotificationEventType, NotificationMessageAttributes};
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// assert_eq!(attrs.source_device_desc(), "some-device-desc");
    /// ```
    pub fn source_device_desc(&self) -> &str {
        &self.source_device_desc
    }

    /// Retrieve the author device ID from an attributes object.
    ///
    /// ```
    /// # use remsync_api_types::{NodeType, NotificationEventType, NotificationMessageAttributes};
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// assert_eq!(attrs.source_device_id(), "some-device-id");
    /// ```
    pub fn source_device_id(&self) -> &str {
        &self.source_device_id
    }

    /// Retrieve the node type from an attributes object.
    ///
    /// ```
    /// # use remsync_api_types::{NodeType, NotificationEventType, NotificationMessageAttributes};
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// assert_eq!(attrs.node_type(), NodeType::CollectionType);
    /// ```
    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    /// Retrieve the node version from an attributes object.
    ///
    /// ```
    /// # use remsync_api_types::{NodeType, NotificationEventType, NotificationMessageAttributes};
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// assert_eq!(attrs.version(), 7);
    /// ```
    pub fn version(&self) -> usize {
        self.version
    }

    /// Retrieve the name of the node from an attributes object.
    ///
    /// ```
    /// # use remsync_api_types::{NodeType, NotificationEventType, NotificationMessageAttributes};
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// assert_eq!(attrs.name(), "My Shiny Node");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A Notification message
///
/// This structure encapsulates the message attributes along with a message ID
/// and a publication time.  The same message might be sent to many endpoints
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationMessage {
    /// The attributes of this event message
    attributes: NotificationMessageAttributes,
    #[serde(rename = "messageId")]
    /// The ID of the message
    message_id_: String,
    /// The ID of the message
    message_id: String,
    #[serde(rename = "publishTime")]
    /// The publishing time of the message
    publish_time_: String,
    /// The publishing time of the message
    publish_time: String,
}

impl NotificationMessage {
    /// Create a new notification message.
    ///
    /// Notification messages consist of an ID, a time, and a set of attributes.
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// let msg = NotificationMessage::new(attrs, "some-message-id", "some-publish-time");
    /// ```
    pub fn new(
        attributes: NotificationMessageAttributes,
        message_id: &str,
        publish_time: &str,
    ) -> Self {
        Self {
            attributes,
            message_id_: message_id.to_owned(),
            message_id: message_id.to_owned(),
            publish_time_: publish_time.to_owned(),
            publish_time: publish_time.to_owned(),
        }
    }

    /// Retrieve the message attributes from a notification message.
    ///
    /// Note, this is only a borrow of the data
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// # let msg = NotificationMessage::new(attrs, "some-message-id", "some-publish-time");
    /// assert_eq!(msg.attributes().id(), "some-id");
    /// ```
    pub fn attributes(&self) -> &NotificationMessageAttributes {
        &self.attributes
    }

    /// Retrieve the message ID from a notification message.
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// # let msg = NotificationMessage::new(attrs, "some-message-id", "some-publish-time");
    /// assert_eq!(msg.message_id(), "some-message-id");
    /// ```
    pub fn message_id(&self) -> &str {
        &self.message_id
    }

    /// Retrieve the publication time from a notification message.
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// # let msg = NotificationMessage::new(attrs, "some-message-id", "some-publish-time");
    /// assert_eq!(msg.publish_time(), "some-publish-time");
    /// ```
    pub fn publish_time(&self) -> &str {
        &self.publish_time
    }
}

/// An actual notification event
///
/// The event consists of a message (potentially sent to several endpoints)
/// and a subscription channel name which is likely unique to this endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationEvent {
    /// The message in the event
    message: NotificationMessage,
    /// The string name of the subscription channel
    subscription: String,
}

impl NotificationEvent {
    /// Create a new notification event.
    ///
    /// Notification events consist of a subscription name and a message.
    /// ```
    /// # use remsync_api_types::*;
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// # let msg = NotificationMessage::new(attrs, "some-message-id", "some-publish-time");
    /// let evt = NotificationEvent::new(msg, "some-subscription-name");
    /// ```
    pub fn new(message: NotificationMessage, subscription: &str) -> Self {
        Self {
            message,
            subscription: subscription.to_owned(),
        }
    }

    /// Access the message for this Notification event
    ///
    /// Note, this is only a borrow of the data.
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// # let msg = NotificationMessage::new(attrs, "some-message-id", "some-publish-time");
    /// # let evt = NotificationEvent::new(msg, "some-subscription-name");
    /// assert_eq!(evt.message().message_id(), "some-message-id");
    /// ```
    pub fn message(&self) -> &NotificationMessage {
        &self.message
    }

    /// Access the subscription channel name for this Notification event
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let attrs = NotificationMessageAttributes::new(
    /// #     "some-user-id", false, NotificationEventType::DocAdded, "some-id",
    /// #     "some-parent-id", "some-device-desc", "some-device-id",
    /// #     NodeType::CollectionType, 7, "My Shiny Node"
    /// # );
    /// # let msg = NotificationMessage::new(attrs, "some-message-id", "some-publish-time");
    /// # let evt = NotificationEvent::new(msg, "some-subscription-name");
    /// assert_eq!(evt.subscription(), "some-subscription-name");
    /// ```
    pub fn subscription(&self) -> &str {
        &self.subscription
    }
}

/// Deserialize bool from String with custom value mapping
fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(other),
            &"true or false",
        )),
    }
}

/// Deserialize usize from String
fn usize_from_string<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let val: String = String::deserialize(deserializer)?;
    let ret: usize = val.parse().map_err(|_| {
        serde::de::Error::invalid_type(serde::de::Unexpected::Str(&val), &"Valid positive integer")
    })?;

    Ok(ret)
}

/// Serialize any displayable type as a string
fn display_to_string<T, S>(val: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: std::fmt::Display,
{
    let val: String = format!("{}", val);
    serializer.serialize_str(&val)
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
    fn deleted() {
        round_trip::<NotificationEvent>(
            r#"
{
  "message": {
    "attributes": {
      "auth0UserID": "auth0|5d67c7af9584340e0f1ec3d5",
      "bookmarked": "false",
      "event": "DocDeleted",
      "id": "0676a521-c548-4ad4-984e-87b875139063",
      "parent": "e0c1c79f-b491-45e7-a431-a46fe1ec8a66",
      "sourceDeviceDesc": "remarkable",
      "sourceDeviceID": "RM102-928-57210",
      "type": "DocumentType",
      "version": "1",
      "vissibleName": "Notebook"
    },
    "messageId": "701046888181767",
    "message_id": "701046888181767",
    "publishTime": "2019-08-31T15:36:45.576Z",
    "publish_time": "2019-08-31T15:36:45.576Z"
  },
  "subscription": "projects/remarkable-production/subscriptions/sub-gm1h-notifications-production"
}
"#,
        )
    }

    #[test]
    fn added_or_changed() {
        round_trip::<NotificationEvent>(
            r#"
{
  "message": {
    "attributes": {
      "auth0UserID": "auth0|5d67c7af9584340e0f1ec3d5",
      "bookmarked": "false",
      "event": "DocAdded",
      "id": "092fd1cc-df38-4fc5-8633-3a8a15a2a316",
      "parent": "e0c1c79f-b491-45e7-a431-a46fe1ec8a66",
      "sourceDeviceDesc": "remarkable",
      "sourceDeviceID": "RM102-928-57210",
      "type": "DocumentType",
      "version": "3",
      "vissibleName": "WiFi and USB local sync"
    },
    "messageId": "700982536103223",
    "message_id": "700982536103223",
    "publishTime": "2019-08-31T14:52:54.158Z",
    "publish_time": "2019-08-31T14:52:54.158Z"
  },
  "subscription": "projects/remarkable-production/subscriptions/sub-gm1h-notifications-production"
}
"#,
        )
    }
}
