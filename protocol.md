# Protocol description of reMarkable sync

Most of this comes via reading the captures made by MITMing the reMarkable while
it sync'd. However some comes from reading [rmapi source][rmapi] and some comes
from the [splitbrain effort][splitbrain].

[rmapi]: https://github.com/juruen/rmapi
[splitbrain]: https://github.com/splitbrain/ReMarkableAPI/wiki

TODO: Authentication API is not yet verified. Currently this is the aggregation
of information from the two sources listed above.

TODO: Firmware update API has not yet been validated, though it appears to be
an independent protocol, using XML, talking to the server at
`get-updates.cloud.remarkable.engineering` which is a redirect to a hosted
service on `remarkable.auto-up.date`. It doesn't take the auth data that the
rest of this API concerns itself with.

TODO: The email API has not yet been explored

TODO: The handwriting recognition API has yet to be explored

# Authentication

**NOTE**: This has a fixed-by-code endpoint of `my.remarkable.com`

The reMarkable authenticates by acquiring a device token by way of a one-time
password supplied by the reMarkable website. This OTP is submitted to an API
on `my.remarkable.com` where it is transformed into a device token.

On each sync, the device token is given to another API on the same site and a
new token is retrieved. These tokens are JWT provided by auth0. It is quite
possible that the JWT is unpacked on the reMarkable because the discovery API
seems to consume `token["auth0-profile"]["UserID"]` at bare minimum. If we are
to create our own tokens, it's likely we'll have to construct a full Auth0 token
in terms of payload, even if we don't use auth0 as the provider. Another option
is to use Auth0 as the provider for our JWT and thereby be as compatible as
we might hope to be.

## Acquiring a device token

In brief we `POST` to <https://my.remarkable.com/token/json/2/device/new> with
an empty bearer token, and a JSON payload. The payload must contain a `code`,
a `deviceDesc` which is a description of the device kind, and a `deviceID` which
is most likely simply a UUIDv4 generated on the device itself.

Example payload from splitbrains captures...

```json
{
  "code": "gliuqtne",
  "deviceDesc": "desktop-windows",
  "deviceID": "701c3752-1025-4770-af43-5ddcfa4dabb2"
}
```

The payload of the response is the device JWT

Interestingly `rmapi` passes `desktop-linux` as the `deviceDesc` where `splitbrains`
chooses to continue to say `device-windows`.

If the descriptor is `remarkable` then it registers as a tablet (necessary to unlock
a new account) any `desktop-*` is a 'desktop' device. I do not yet know the
prefix for a mobile device.

## Acquiring a user token

User tokens are needed for all other non-blob operations. Both the splitbrains
and the rmapi codebases acquire a fresh user token on each run.

We `POST` to <https://my.remarkable.com/token/json/2/user/new> with a bearer
of the device token and an empty body.

It's important to send a `Content-Length: 0` header otherwise things fail.

The payload of the response is a user JWT which can be used for the foreseeable
future, though parsing it and knowing the expiry could be useful. A token I
captured during my tests had an `exp` 24 hours after its `iat` and the `iat` was
pretty much when the capture happened indicating the user token is refreshed
each time the tablet connects to the cloud.

# Service discovery API

**NOTE**: This has a fixed-by-code endpoint of `service-manager-production-dot-remarkable-production.appspot.com`

Service discovery uses two endpoints on the above host. One allows for the
discovery of a websocket endpoint, and the other the document storage endpoint.

The former provides notifications to the tablet when changes occur on the server
which allows the tablet to react to those changes if it so chooses. The latter
is the primary endpoint through which document management on the cloud occurs.

## Discovering the websocket

This is a `GET` request to `/service/json/1/notifications` passing query
string parameters of:

- `environment` which is always `production`
- `group` which appears to be the `UserID` from the `auth0-token` in the JWT
- `apiVer` which appears to always be `1`

The body of the response is a JSON document of the form:

```json
{
  "Status": "OK",
  "Host": "XXXX-notifications-production.cloud.remarkable.engineering"
}
```

The `XXXX` in the hostname varies from request to request and appears to be a
very rudimentary attempt at load-balancing across a number of endpoint servers.

I've never seen a failure from this API, and if the tablet doesn't get a good
response, it will **refuse** to operate at all in the cloud sense.

## Discovering the document storage

This is a `GET` request to `/service/json/1/document-storage` passing query
string parameters of:

- `environment` which is always `production`
- `group` which appears to be the `UserID` from the `auth0-token` in the JWT
- `apiVer` which appears to always be `2`

The body of the response is a JSON document of the form:

```json
{
  "Status": "OK",
  "Host": "document-storage-production-dot-remarkable-production.appspot.com"
}
```

Currently that host seems to be statically defined, though this API suggests
that the tablet might react usefully if it were returned as something else.

# The notifications websocket

The notifications websocket _MUST_ be established or the tablet will refuse to
continue with its synchronisation efforts. The websocket is handled via an
endpoint on a selection of servers one of which is returned during discovery.
It does not appear to be relevant which you get.

So far, the websocket appears to be used unidirectionally as a structured message
queue from the cloud to the tablet. It only sends events which occur WHILE the
socket is established, there's no queueing on the server side when not connected,
and each websocket message appears to be a JSON object.

The websocket's endpoint is `/notifications/ws/json/1` and each packet is a text
frame in websocket protocol. The payload looks something like:

```json
{
  "message": {
    "attributes": {
      "auth0UserID": "auth0|XXXXXXXXXXXXXXXXXXXXXXXX",
      "bookmarked": "false",
      "event": "DocAdded",
      "id": "092fd1cc-df38-4fc5-8633-3a8a15a2a316",
      "parent": "e0c1c79f-b491-45e7-a431-a46fe1ec8a66",
      "sourceDeviceDesc": "remarkable",
      "sourceDeviceID": "RMXXX-XXX-XXXXX",
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
```

or

```json
{
  "message": {
    "attributes": {
      "auth0UserID": "auth0|XXXXXXXXXXXXXXXXXXXXXXXX",
      "bookmarked": "false",
      "event": "DocDeleted",
      "id": "0676a521-c548-4ad4-984e-87b875139063",
      "parent": "e0c1c79f-b491-45e7-a431-a46fe1ec8a66",
      "sourceDeviceDesc": "remarkable",
      "sourceDeviceID": "RMXXX-XXX-XXXXX",
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
```

The `subscription` is a pubsub identifier which includes the name of the server
which was connected to. This is fairly uninteresting in the end.

From within the `message` field, the `messageId` and `message_id` fields,
`publishTime` and `publish_time` are all fairly obvious.

The interesting content comes in the `attributes` part of the `message` field.

- The `auth0UserID` is, quite clearly, the same as we have discussed before.
- The `event` is the field we want to switch our handling based on.
- The `sourceDeviceDesc` and `sourceDeviceID` fields likely match the fields used
  when registering the device which created the token in use
- All other fields seem to be basically the metadata of an item on the reMarkable
  data storage. `id` being the obvious one to look out for and `version` if needed.

Interesting things to note: The event will always be `DocAdded` even if what was
added was in fact a folder (`CollectionType`). Also it seems to be `DocAdded`
for an updated node too, though I suppose it's a new version of that node, but
the old version doesn't get a `DocDeleted`

The event is `DocDeleted` for a deletion event, whatever the document type might be.

# The document storage API

The storage API mimics the flat nature of the reMarkable's format quite directly.

The storage API server name is provided via the discovery protocol (see above).

The tablet appears to send delete operations to the storage API _before_ it
establishes the websocket (or at least cotemporaneously with that) since in
at least one trace we see it delete things before even attempting to discover
the websocket endpoint server.

Interestingly, folders appear to have a `.content` and thus a blob when created
on the mobile phone app or the tablet. As a result, new folders push/pull blobs
when created/detected. It's possible that this is done to keep things orthogonal
in that all nodes in the tree have blobs, even if the folder blobs are basically
vestigial.

## Retrieving the document list

This is the primary endpoint and works in one of two ways. Either it returns
the full list of all the nodes in the storage tree (documents?) or else it
returns a single node which was requested by the client. In both cases it
gives back a list of json objects, where each object is one such node. The
endpoint can also be told to please provide a limited-use (one time?) download
URL to retrieve the content for this node.

This api is at `/document-storage/json/2/docs` and a plain GET of that will
give back a full list of all the nodes...

```json
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
```

Each entry carries a `Success` and `Message` field which are related to the
protocol in general, if `Success` is `false` then something odd has occurred
and the `Message` will be relevant.

The `BlobURLGet` and `BlobURLGetExpires` fields are related to retrieving of
blobs, and default to the empty string and a zero time respectively. See below.

The `ModifiedClient` is a rendered version of the time the document was last
modified on a client. This is rendered by the client not the server (see below)
and is likely used along with `Version` for sync/clobber purposes.

The rest of the fields (and in fact `ModifiedClient` too) come from the client
and pretty much match stuff you'd expect to see in the `.metadata` file for the
node on the filesystem. Obviously the field names and value rendering is not
exactly the same as the `.metadata` file.

Nominally all those fields are only of interest to the client, but if we want to
perform any kind of split-sync in a new server implementation, the `ID` and `Parent`
fields will be of great interest since they help form the heirarchy of nodes.

### Retrieving a document's blob

If the client wishes to retrieve the blob associated with a document (node) then
it requests a `GET` of the same `/document-storage/json/2/docs` endpoint but
passing it an additional set of parameters:

- `withBlob` - a boolean parameter and with any trueish value (`1`, `true`, `yes`)
  the `BlobURLGet` and `BlobURLGetExpires` fields will be valid.
- `doc` - the UUID (ID) of the document you want. Typically you set this when
  requesting `withBlob` since there's likely some expense server-side when you
  request a blob, so this limits the effect

If a blob URL is requested, then it's valid until the expiry time, or until it
is retrieved once. The URL is a _complete_ url including authentication information
and is retrieved with a simple `GET`. Currently the cloud API will return URLs
inside `storage.googleapis.com`.

## Uploading to the cloud

Uploading is a three phase process. First an upload request is sent to prepare
the ground, the result of this is that the blob can be uploaded. Then the blob
is sent to the storage API via whatever URL was specified. Finally the update
is completed, which makes the change visible to other clients including sending
a websocket notification to all connected parties.

It's worth noting that these APIs seem to take lists, but the device only ever
seems to transact a single node at a time in this manner, and doesn't seem
to pipeline them. A server implementation might want to assert these behaviours
or may want to be nice and allow multiple at once.

Even more fascinatingly, the protocol seems fine with uploading stuff whose
parents do not exist yet. It's not clear if the tablet would download something
uploaded before its parent folder was available too, but it may be worth experimenting
in the future.

### Preparing the ground - request upload URL

With a `PUT` to `/document-storage/json/2/upload/request` whose payload looks
like:

```json
[
  {
    "ID": "092fd1cc-df38-4fc5-8633-3a8a15a2a316",
    "Parent": "e0c1c79f-b491-45e7-a431-a46fe1ec8a66",
    "Type": "DocumentType",
    "Version": 3
  }
]
```

A response will be provided of the form:

```json
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
```

For brand new things, `Version` should be `1`, it's possible this could fail
for example if a quota is exceeded or somesuch.

### Uploading the blob

As with the blob download - this is a simple operation using the URL provided
in the first phase of uploading/updating a document node. The blob doesn't
seem to be transferred for renaming a folder, so I think the tablet maintains
some concept of whether or not it needs to send a blob.

TODO: Check if the no-transfer holds if a notebook is renamed only.

### Completing the update

Once the blob (if any) is transferred, then the update is completed by means of
a `PUT` to `/document-storage/json/2/upload/update-status` with a payload of:

```json
[
  {
    "Bookmarked": false,
    "CurrentPage": 2,
    "ID": "092fd1cc-df38-4fc5-8633-3a8a15a2a316",
    "ModifiedClient": "2019-08-31T14:49:51.302302Z",
    "Parent": "e0c1c79f-b491-45e7-a431-a46fe1ec8a66",
    "Type": "DocumentType",
    "Version": 3,
    "VissibleName": "WiFi and USB local sync"
  }
]
```

As you can see, this is essentially the data which would come via the `.../docs`
API, minus status, message, and blob URL stuff.

The response to such a request is of the form:

```json
[
  {
    "Success": true,
    "Message": "",
    "ID": "092fd1cc-df38-4fc5-8633-3a8a15a2a316",
    "Version": 3
  }
]
```

Only a success/message pair, and the ID/Version pair is returned here.

## Deleting a node

When a node is deleted on the client, as noted above, the tablet seems to send
the deletion request immediately on cloud connect, before establishing the
notification socket. Though again, as noted, it doesn't _have_ to be the case
since once established if an interactive delete occurs, then the websocket is
notified anyway. It's likely this behaviour is a housekeeping behaviour of the
client rather than something which MUST be done.

To delete a node, a `PUT` to `/document-storage/json/2/delete` with a body of:

```json
[
  {
    "ID": "5109f2cc-5559-4239-8364-dbd709ca9126",
    "Version": 1
  }
]
```

Again, a list, though the tablet only ever seems to send one at a time, it would
behoove a server implementation to accept many. Only the ID and Version are sent
and must match the server-side stored data or the delete will fail.

The response is of the form:

```json
[
  {
    "Success": true,
    "Message": "",
    "ID": "5109f2cc-5559-4239-8364-dbd709ca9126",
    "Version": 1
  }
]
```

Where success and message are as before, and id/version will match the request.
