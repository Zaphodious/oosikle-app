## Entities: 

Objects are strong, and must have a name

Attributes for objects are weak, and must have a name, a data type, and a value

Media types are strong, and must have a string ID and a name

Media categories are strong, and must have a string ID and a name

File extensions are strong, and must have a description, and an extension tag

Collections are strong, and must have a name, a directory path, a visibility flag, and a location (not-path)

Files are strong, and must have a name, a size, a hash, a path

FileBlobs are weak, and must have the binary contents of a file

Plugins are strong, and must have a package name, a display name, and a version

Devices are strong, and must have a name, a UUID, a description, and a path to an icon

DeviceSyncLists are weak, and must have a relative directory path 

## Relationships

.Objects 1/1 - 0/m Attributes (Identifying)

.Objects 0/m - 1/m Files

.Media Types 0/m - 1/1 Media Category

.File Extensions 0/m - 1/1 Media Types

.Collections 0/m - 0/m Media Categories 

.Objects 0/m - 0/m Collections

.Files 0/m - 1/1 File Extensions

.Files 1/1 - 0/1 FileBlobs

.Plugins 0/m - 0/m Media Types

/Objects 0/m - 1/1 Plugins

.Devices 1/1 - 0/m DeviceSyncLists (identifying)

.Plugins 1/1 - 0/m DeviceSyncLists (identifying)

.Collections 1/1 - 0/m DeviceSyncLists

