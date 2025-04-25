create table if not exists MediaCategories (
    media_category_id text primary key, 
    media_category_display_name text not null
);

create table if not exists MediaTypes (
    media_type_id text primary key,
    media_type_display_name text not null,
    media_category_id text not null,
    foreign key (media_category_id) references MediaCategories(media_category_id)
);

create table if not exists FileExtensions (
    file_extension_tag text primary key,
    file_extension_description text not null
);

create table if not exists MediaTypesForFileExtensions (
    file_extension_tag text not null,
    media_type_id text not null,
    primary key (file_extension_tag, media_type_id),
    foreign key (file_extension_tag) references FileExtensions(file_extension_tag),
    foreign key (media_type_id) references MediaTypes(media_type_id)
);

create table if not exists Files (
    file_uuid text primary key,
    file_name text not null,
    file_size_bytes integer not null,
    file_hash text not null,
    file_dir_path text not null,
    file_extension_tag text not null,
    file_encoding text,
    media_type_override_id text,
    file_deleted integer,
    file_read_only integer,
    foreign key (file_extension_tag) references FileExtensions(file_extension_tag),
    foreign key (media_type_override_id) references MediaTypes(media_type_id)
);

create table if not exists FileBlobs (
    file_uuid text primary key,
    blob_value blob,
    foreign key (file_uuid) references Files(file_uuid)
);

create table if not exists Plugins (
    plugin_package_name text primary key,
    plugin_display_name text not null,
    plugin_version integer not null
);

create table if not exists MediaTypesForPlugins (
    plugin_package_name text not null,
    media_type_id text not null,
    primary key (plugin_package_name, media_type_id),
    foreign key (plugin_package_name) references Plugins(plugin_package_name),
    foreign key (media_type_id) references MediaTypes(media_type_id)
);

create table if not exists Objects (
    object_uuid text primary key,
    object_name text not null,
    plugin_package_name text not null,
    object_deleted integer,
    foreign key (object_uuid) references Files(file_uuid),
    foreign key (plugin_package_name) references Plugins(plugin_package_name)
);

create table if not exists ObjectAttributes (
    object_uuid text not null,
    attribute_name text not null,
    attribute_value blob,
    primary key (object_uuid, attribute_name),
    foreign key (object_uuid) references Objects(object_uuid)
);

create table if not exists ExtraFilesForObjects (
    object_uuid text not null,
    file_uuid text not null,
    file_note text not null,
    primary key (object_uuid, file_uuid),
    foreign key (object_uuid) references Objects(object_uuid),
    foreign key (file_uuid) references Files(file_uuid)
);

create table if not exists FileArtwork (
    file_uuid text not null,
    artwork_file_uuid text not null,
    artwork_role text not null,
    primary key (file_uuid, artwork_file_uuid, artwork_role),
    foreign key (file_uuid) references Files(file_uuid),
    foreign key (artwork_file_uuid) references Files(file_uuid)
);

create table if not exists Collections (
    collection_uuid text primary key,
    collection_name text not null,
    collection_visible integer not null,
    collection_location text not null,
    collection_deleted integer -- bool
);

create table if not exists CollectionHiddenColumns (
    collection_uuid text not null,
    column_name text not null,
    primary key (collection_uuid, column_name),
    foreign key (collection_uuid) references Collections(collection_uuid)
);

create table if not exists MediaCategoriesForCollections (
    collection_uuid text not null,
    media_category_id text not null,
    primary key ( collection_uuid, media_category_id),
    foreign key (collection_uuid) references Collections(collection_uuid),
    foreign key (media_category_id) references MediaCategories(media_category_id)
);

create table if not exists ObjectsInCollections (
    object_uuid text not null,
    collection_uuid text not null,
    idx integer,
    primary key (object_uuid, collection_uuid),
    foreign key (object_uuid) references Objects(object_uuid),
    foreign key (collection_uuid) references Collections(collection_uuid),
    unique(collection_uuid, idx)
);

create table if not exists Devices (
    device_uuid text primary key,
    device_name text not null,
    device_description text not null,
    device_icon_path text
);

create table if not exists DeviceSyncLists (
    device_uuid text not null,
    collection_uuid text not null,
    plugin_package_name text not null,
    dsl_directory_on_device text not null,
    last_sync_time integer not null,
    primary key (device_uuid, collection_uuid),
    foreign key (device_uuid) references Devices(device_uuid),
    foreign key (plugin_package_name) references Plugins(plugin_package_name),
    foreign key (collection_uuid) references Collections(collection_uuid)
);

/*
create view if not exists ObjectRecordView as
select
	file_uuid as uuid,
	object_name as name,
	plugin_package_name as manager,
	file_path,
	object_deleted as deleted,
	media_type_override_id as media_type_override
from Objects left join Files on Objects.object_uuid = Files.file_uuid
	where Files.file_deleted = 0 and Objects.object_deleted = 0;
    */
