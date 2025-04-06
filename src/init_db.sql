create table MediaCategories (
    media_category_id text primary key, 
    media_category_display_name text not null
);

insert into MediaCategories values ('DOCUMENT', 'Document');

create table MediaTypes (
    media_type_id text primary key,
    media_type_display_name text not null,
    media_category_id text not null,
    foreign key (media_category_id) references MediaCategories(media_category_id)
);

insert into MediaTypes values ('PLAINTEXT', 'Plain Text File', 'DOCUMENT');

create table FileExtensions (
    file_extension_tag text primary key,
    file_extension_description text not null,
);

insert into FileExtensions values ('TXT', 'Ordinary text file');

create table MediaTypesForFileExtensions (
    file_extension_tag blob not null,
    media_type_id text not null,
    primary key (file_extension_tag, media_type_id),
    foreign key (file_extension_tag) references FileExtension(file_extension_tag),
    foreign key (media_type_id) references MediaTypes(media_type_id)
);

insert into MediaTypesForFileExtensions values ('TXT', 'PLAINTEXT');

create table Files (
    file_uuid blob primary key,
    file_name text not null,
    file_size_bytes integer not null,
    file_hash text not null,
    file_path text not null,
    file_extension_tag text not null,
    file_encoding text,
    foreign key (file_extension_tag) references FileExtensions(file_extension_tag)
);

insert into Files values (
    X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    "welcome.txt",
    0,
    'a38bcfa83e52932d49e36146950db40423daeb89a2e1f1b9734401bc98f1c79e',
    '',
    'TXT',
    'UTF-8'
);

create table FileBlobs (
    file_uuid blob primary key,
    blob_value blob,
    foreign key (file_uuid) references Files(file_uuid)
);

insert into FileBlobs values (
    X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'Welcome to Oosikle!'
);

create table Plugins (
    plugin_package_name text primary key,
    plugin_display_name text not null,
    plugin_version integer not null
);

insert into Plugins values (
    'oosikle.manager.text',
    'Default Text File Manager',
    1);


create table MediaTypesForPlugins (
    plugin_package_name text not null,
    media_type_id text not null,
    primary key (plugin_package_name, media_type_id),
    foreign key (plugin_package_name) references Plugins(plugin_package_name),
    foreign key (media_type_id) references MediaTypes(media_type_id)
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.text',
    'PLAINTEXT'
);

create table Objects (
    object_uuid blob primary key,
    object_name text not null,
    plugin_package_name text not null,
    foreign key (plugin_package_name) references Plugins(plugin_package_name)
);

insert into Objects values (
    X'ABADCAFEABADCAFEABADCAFEABADCAFE',

    'Welcome File',
    'oosikle.manager.text'
);

create table ObjectAttributes (
    object_uuid blob not null,
    attribute_name text not null,
    attribute_data_type text not null,
    attribute_value blob,
    primary key (object_uuid, attribute_name),
    foreign key (object_uuid) references Objects(object_id)
);

create table FilesForObjects (
    object_uuid blob not null,
    file_uuid blob not null,
    file_priority integer not null,
    primary key (object_uuid, file_uuid),
    foreign key (object_uuid) references Objects(object_uuid),
    foreign key (file_uuid) references Files(file_uuid)
);

insert into FilesForObjects values (
    X'ABADCAFEABADCAFEABADCAFEABADCAFE',
    X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    0
);

create table Collections (
    collection_uuid blob primary key,
    collection_name text not null,
    collection_visible integer not null,
    collection_location text not null
);

insert into Collections values (
    X'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
    'Default Briefcase',
    TRUE,
    ''
);

create table MediaCategoriesForCollections (
    collection_uuid blob not null,
    media_category_id text not null,
    primary key ( collection_uuid, media_category_id),
    foreign key (collection_uuid) references Collections(collection_uuid)
    foreign key (media_category_id) references MediaCategories(media_category_id),
);

insert into MediaCategoriesForCollections values (
    X'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
    'DOCUMENT'
);

create table ObjectsInCollections (
    object_uuid blob not null,
    collection_uuid blob not null,
    primary key (object_uuid, collection_uuid),
    foreign key (object_uuid) references Objects(object_uuid),
    foreign key (collection_uuid) references Collections(collection_uuid)
);

insert into ObjectsInCollections values (
    X'ABADCAFEABADCAFEABADCAFEABADCAFE',
    X'BADC0FFEE0DDF00DBADC0FFEE0DDF00D'
);

create table Devices (
    device_uuid blob primary key,
    device_name text not null,
    device_description text not null,
    device_icon_path text
);

insert into Devices values (
    X'0DE2C3400DE2C3400DE2C3400DE2C340',
    'Example Flash Drive',
    'This is a flash drive that is used to test the program',
    NULL
);

create table DeviceSyncLists (
    device_uuid blob not null,
    plugin_package_name text not null,
    collection_uuid blob not null,
    dsl_directory_on_device text not null,
    primary key (device_uuid, plugin_package_name),
    foreign key (device_uuid) references Devices(device_uuid),
    foreign key (plugin_package_name) references Plugins(plugin_package_name),
    foreign key (collection_uuid) references Collections(collection_uuid)
);

insert into DeviceSyncLists (
    X'0DE2C3400DE2C3400DE2C3400DE2C340',
    'oosikle.manager.text',
    X'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
    'documents'
);

