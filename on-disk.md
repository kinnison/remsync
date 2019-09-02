# On Disk storage format for reMarkable

The on-disk storage format for the reMarkable, at least for the tablet itself,
is a flat structure based out of `~/.local/share/remarkable/xochitl` where
each node is represented by `SOME-UUID-STRING.metadata` and associated files
each sharing the UUID prefix.

Each node has a `.metadata` and a `.content` file, both a JSON documents whose
content is described below. The `.content` for collection nodes (folders) is
simply an empty list, whereas it's an object for document nodes.

If the `.content` indicates a filetype other than `""` then there will be a `.pdf`
or similar too.

Finally there is the `SOME-UUID-STRING/` directory which contains an `.rm` file
and `-metadata.json` file for each page, named after the UUIDs in the `.content`
file's `"pages"` field.

## The `.metadata` file

This is the core file for any node. This file basically indicates the node
state for the application. When a storage directory is enumerated and an index
built, it is these files which are read to build the index. The `.metadata`
files are similar but not identical to the document-storage docs response format.

Example:

```json
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
```

The `parent`, `type`, `version`, `visibleName` (note, not misspelled) should
be obvious. `pinned` is the bookmark flag, `lastModified` is when the node was
last changed, `deleted` indicates the node has been deleted from the local
device and so is not part of the index (but may need deleting from the sync
server). `synced` is whether or not the data is synchronised, `metadatamodifed`
is whether or not the metadata needs to be synchronised. For example, deleting
a node sets `metadatamodified` but does not unset `synced` nor does it increment
the `version` or update `lastModified`.

Editing a document sets `lastModified`, `metadatamodified`, `modified`, but
does not alter the `version`, nor unset `synced`. After sync, `version` is
incremented, `metadatamodified` and `modified` are unset.

A brand new document has `modified` true, `synced` false, and `version` zero.
Once synchronised for the first time, `modified` is unset, `synced` is set, and
version is set to `1`.

If a device is offline when a node is deleted, the node is marked deleted, has
`metadatamodified` set and this is dealt with when the device next is able to
sync with the server.

## The `.content` file

The exact content of this file varies from document to document. Collection
nodes are an empty list `[\n]\n`.

Document nodes, when fresh, contain approx:

```json
{
  "extraMetadata": {},
  "fileType": "pdf",
  "fontName": "",
  "lastOpenedPage": 0,
  "lineHeight": -1,
  "margins": 100,
  "orientation": "portrait",
  "pageCount": 4,
  "pages": [
    "ab559086-18e4-4332-88a0-1b5453e78ce4",
    "b90f1f01-6a3b-474e-8269-7d708c8ad10b",
    "8cedeb13-6a55-479c-8653-41022bab8fa1",
    "6eb1193f-4b21-4b80-a75a-d81316573685"
  ],
  "textScale": 1,
  "transform": {
    "m11": 1,
    "m12": 0,
    "m13": 0,
    "m21": 0,
    "m22": 1,
    "m23": 0,
    "m31": 0,
    "m32": 0,
    "m33": 1
  }
}
```

When a document has been opened at least once, the `extraMetadata` is filled out
thusly:

```json
{
  "extraMetadata": {
    "LastBrushColor": "Black",
    "LastBrushThicknessScale": "2",
    "LastColor": "Black",
    "LastEraserThicknessScale": "2",
    "LastEraserTool": "Eraser",
    "LastPen": "Ballpoint",
    "LastPenColor": "Black",
    "LastPenThicknessScale": "2",
    "LastPencil": "SharpPencil",
    "LastPencilColor": "Black",
    "LastPencilThicknessScale": "2",
    "LastTool": "SharpPencil",
    "ThicknessScale": "2"
  }
}
```

Dealing with that first, the colours are one of "Black", "Gray", "White",
the tools are, from the top to the bottom on the screen, the Pen, Pencil,
Paintbrush, Highlighter, Eraser, SelectionTool. Other tools are not persistent
in their selection.

The `LastTool` entry however, specifically references the exact pen, pencil,
or eraser which was in use if that's the entry. So, for example, if the pen
tool was on `Fineliner` then that's the value `LastTool` will have if the pen
was selected last.

- Valid pen names are `Ballpoint`, `Marker`, `Fineliner`.
- Valid pencil names are `SharpPencil`, `Pencil`.
- Valid eraser names are `Eraser`, `EraseSection`. (The erase page isn't selectable)
- Otherwise the `Paintbrush`, `Highlighter`, and `SelectionTool` might be
  the most recent tools

Finally, thickness can be 1, 2, or 3.

Returning to the rest of the content data...

- `fileType` if non-empty indicates the type of the underlying data. This
  can be `pdf` and probably `epub`.
- `fontName` - only ever seen this be the empty string
- `lastOpenedPage` - this corresponds to the `CurrentPage` in the sync metadata
  I believe.
- `lineHeight` - only ever seen `-1`
- `margins` - so far only seen `100`
- `orientation` - can be `portrait` or `landscape`
- `pageCount` - is the count of pages in the document
- `pages` - An array of UUIDs, one for each page, used to find metadata in
  the metadata directory. This is done so pages don't have to be renumbered
  when reordering them in the document.
- `textScale` - So far only seen `1`
- `transform` - A set of nine transformation values `m11` `m12` .. `m33`
  whose values appear to be `0` except for `m11` `m22` and `m33` which are 1
  which suggests this is some kind of scale/rotation matrix.

TODO: Try and flesh out more of the above.

## The `.pagedata` file

This file, if present, has one line (`\n` separated) per page, indicating the
template used for that page as a simple string (e.g. `P Lines medium`)

## The pages themselves, and their metadata

Each page is in `SOME-LONG-UUID/` named after the UUID in the `pages` entry
described above in the `.content` file. There are two files, one `.rm` file
and one `-metadata.json` file.

The JSON document is of the form:

```json
{
  "layers": [
    {
      "name": "Layer 1"
    }
  ]
}
```

Layers can be manipulated in the document, with the layers button, there can be
several layers on a page, and they can be reordered at whim.

The `.rm` file contains the stroke information for the layers. It is described
in more detail online.

TODO: Incorporate description of the `.rm` file.

## Other directories

There is a `.cache` directory which appears to be cached PNGs of the rendered
pages, presumably to speed up changing page in the document.

There is a `.thumbnails` directory which appears to contain JPG images of thumbnails
of the pages. These are 280x374 pixels and if absent will be regenerated by
the device, so it's fine to lose them.

I've seen `.highlights` and `.textconversion` directories though so far nothing
within those.

# Mechanism for creating the transfer zip files

The transfer zip files consist of all the primary data for a document, without
the cache data, with the thumbnails and pages renamed to be numbered instead.

At a guess, this is because initially they were just numbered and over time the
UUID based pages were introduced because that way they could reorder/insert/delete
pages with more ease. But the sync protocol was semi-fixed by then.

Given a full understanding of the node, the zip file can be built by creating
a zip file, inserting the `.content` and the `.pagedata` if available. Then
creating the two directories (the base one and the `.thumbnails` one) in the zip
file before, for each page, adding the `.rm` the `-metadata.json` and the
thumbnail if it exists.

The zip ordering of the official zips appears to be the `.content` then the page
directory and data files, then the `.pagedata`, and then any thumbnails. As
such it may behoove us to do it that way instead, though it's unlikely to matter
it may make sense to maintain the ordering.

# Mechanism for unpacking the transfer zip files

In order to not be dependent on the ordering, in case it changes in the future,
we first scan the zip index for the `.content` which we unpack and then load in.
That teaches us the page UUIDs which means we can then extract the `.rm` the
`-metadata.json` and the `.thumbnails` as available. Obviously any missing data
should be treated as simply "there is nothing for that page" which I suppose might
be possible.
