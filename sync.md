# Synchronisation heuristics

**NOTE**: This is entirely guesswork because we have no way of knowing what the
actual synchronisation decision mechanism is. As such, all this is speculation
and analysis and should not be taken as gospel. Coming out of this is how the
`reMsync` suite will operate, at least initially.

A first important observation is that the sync server itself appears to be
entirely subservient to any client connected to it. The server simply acts as
a remote object store with indexing and notification capability. This means
that a first-pass server can be very simplistic, not needing to look deeply into
the data at all.

A second useful observation is that, based on that, it's quite possible that
the synchronisation heuristics have several holes which users will need to be
aware of, in case of falling into them.

In this document we will talk about nodes and documents interchangeably. Since
in the on-disk structures, they both have metadatamodified as well as modified
markers we can assume that at least somewhat they are equivalent whether actual
documents, or simply collection nodes.

When we talk about the 'client' or 'device' we are talking about any synchronisation
client, such as a reMarkable tablet, a phone client, official desktop client,
or reMsync acting as a client. The only distinction which may be made is that
a 'device' is interested in the content of the data files, whereas a 'client'
_may_ simply be interested in synchronising and maintaining a blind copy.
Any of the above mentioned synchronisation clients may therefore be a 'device'
though it's likely that only 'reMsync' may be a pure 'client'.

When we talk about the server, we mean the partner in the synchronisation which
is acting as the sync server. The server is always subservient to the client
and does as it is told. The server may be serving multiple clients simultaneously
and clients have to treat the server as ever-changing. As such the clients have
a way to get regular updates from the server whenever a change occurs which they
can use to decide what to do next.

# The correct way for a client to establish a sync session

In order for a client to properly establish a sync session they need to do the
following steps. Before they begin, they must take stock of the current data
set that they are operating with, so that they know the status of all the nodes
that they are currently aware of.

1. Use their device token to communicate to the authentication endpoint to
   create a user token. This is typically useful for 24 hours and importantly
   it contains the auth0 ID needed for the next step
2. Query the service discovery endpoint for the hostnames of the notification
   server and the document storage server.
3. Establish a websocket with the notification server
4. Connect to the document storage server and begin the sync session

The session with the storage server seems to begin with a device first deleting
any nodes which it was aware of existing but deleting since the last time it
synchronised. Then the client will ask for the server's current node list and
begin the full synchronisation heuristic.

# Synchronisation data points

Assuming that all nodes the client knew about being deleted have been removed
from the server's view of the world before the node list is retrieved, then
the following starts...

For each node 'N' in the superset of nodes known to the client and nodes known
to the server, the following data is accumulated:

1.  Whether the client knows about N (cNe)
2.  Whether the server knows about N (sNe)
3.  If cNe, then:
    - The client's lastmodified (cNlm)
    - The client's version (cNv)
    - whether the client has sync'd before (cNs)
    - Whether the client has marked the metadata as modified (cNmd)
    - Whether the client has marked the data as modified (cNm)
4.  If sNe, then:
    - The server's (client) lastmodified (sNlm)
    - The server's version (sNv)

At this point there're a number of primary states to be dealt with:

1. Document known to client but not to server
2. Document known to server but not to client
3. Document known to both

Each of the above have sub-states, but we'll consider each in turn

## Document known to client, but not to server

Now we have three options to consider again

### Document has never been sync'd

This is the simplest -- the client has a node which has _never_ been sent to
the server, and so it bundles it up and sends it over to the server.

The process here will assign a version of 1 to new nodes sent to the server.
This version is stored locally as well.

As a useful checkpoint here, the client should verify that the parent chain for
the node is also present on the server. It's possible that an entire folder
heirarchy had been deleted, and that'd need to be recreated, so if anything in
the parent chain for the node is missing from the server's document list, then
it should be _immediately_ sent to the server as well.

Any node sent to the server is marked as 'synced' and has cNmd and cNm cleared
If any of the nodes considered in this process were in the 'to delete' list then
they are removed from that list at this time.

### Document has been sync'd but not locally modified

In this state, the document _as it currently is on the device_ was previously
sync'd to the server and has since been deleted. This node is put onto the
'to delete' list.

### Document has been sync'd but has been locally modified

In this state, the document _previously_ was on the server, but since then,
another client has deleted the document from the server _and_ the local device
has modified the document.

In this case, the client acts as though the document had never been sync'd,
(but does not reset its version to 1) and uploads it to the server, automatically
adding any missing parent nodes in the parent chain (again without resetting
any of the version numbers).

TODO: Confirm that this behaves as we suggest it ought here.

## Document known to server but not to client

Since the client sends all known deletes to the server before it begins this
synchronisation startup, this is a very simple case -- the document is new to
this client, and so it downloads it.

## Document known to both server and client

This is the potentially fraught situation and the correct synchronisation is
not entirely algorithmic. As such this is a heuristic determined in part by
watching real clients, and in part by thinking about the problem and defining
how we want reMsync to work.

There are a number of aspects at play here. The first is to compare the
versions. This leads to three basic situations:

1. Server is definitely out of date (sNv less than cNv)
2. Server and client _may_ be equivalent (sNv equals cNv)
3. Client is definitely out of date but may have local changes (cNv less than sNv)

Since the server cannot cause version increments on its own, and since each client
only increments version numbers when it uploads to the server, it's very likely
that the first case will never occur but might if the server had to restore from
backup or similar. In that case, the client may choose to download the server
content anyway, rename (new UUID) its copy, and upload that too. However, more
likely is that the client will simply upload its copy clobbering the server copy.

The other two cases need a little more thought

### sNv equals cNv

In this situation we need to look to the details. If there are changes on the
client (metadatamodified or modified) then the client can upload the new data
clobbering anything on the server.

Otherwise the client can use the data from the server to clobber metadata locally
such as parent, name, etc.

### cNv less than sNv

In this situation the client has an older version number than the server. If the
client has no local changes, it can simply download the server's version and clobber
its local copy.

If, on the other hand, there are also local changes, then it's up to the client
to decide if it will download and clobber, upload and clobber, or perform a dance
with renaming the local objects, downloading the server one, and uploading a fresh
one to the server as though it has never been sync'd before. This is a hard
decision to make.

TODO: Examine, if we can, what a real sync client would do

## Deleting the 'to delete' list

Once all the nodes have been considered as above, if there are any nodes in the
'to delete' list then they are removed from the local device.

# Ongoing sync

Until the sync session has been fully established by the above, notifications
should be queued up.

Once the sync session is fully established, queued notifications are processed
and then new changes may arrive via the notification socket. Any notifications
caused by the client can be ignored, but any caused by another sync client need
to have attention paid to them.

Notifications should be processed in-order since that way the client may reduce
its chance of ending up in an inconsistent state. Deletions are simply honoured
and additions are queued up to be processed with the above algorithm.

As and when the client commits changes to its local store (e.g. via document edits)
it queues a fresh upload to the server. It's possible that the client may
choose to only upload documents when they are closed, to reduce bandwidth consumed.

# Ending up with orphaned nodes

If a node exists on a device with no parent, and the initial sync has been
established and there are no queued notifications to deal with, a device may
choose to reparent that node to the root of the device (parent="") by performing
a metadata update which can be pushed in realtime to the cloud if connected.
