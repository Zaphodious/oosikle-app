## Entities: 

Objects are strong, and must have a name

Attributes for objects are weak, and must have a name, a data type, and a value

Media types are strong, and must have a name

Media categories are strong, and must have a name

File extensions are strong, and must have a description, and an extension tag

Collections are strong, and must have a name, a directory path, and a visibility flag 

Files are strong, and must have a name, a size, a hash, a path

Blobs are weak, and must have the binary contents of a file

Plugins are strong, and must have a package name, a display name, and a version

Devices are strong, and must have a name

Device-Plugin Mappings are weak, and must have a relative directory path 

## Relationships

Objects 1/1 - 0/m Attributes 

Objects 0/m - 1/m Files

Media Types 0/m - 1/1 Media Category

Objects 0/m - 1/1 Media Types

File Extensions 1/1 - 0/m Media Types

Collections 0/m - 0/m Media Categories 

Objects 0/m - 0/m Collections

Files 0/m - 1/1 File Extensions

Files 1/1 - 0/1 Blobs

Plugins 0/m - 1/m Media Types

Objects 0/m - 1/1 Plugins

Devices 1/1 - 0/m Device-Plugin Mappings

Plugins 1/1 - 0/m Device-Plugin Mappings

