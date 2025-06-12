create table if not exists MediaCategories (
    media_category_id text primary key collate nocase, 
    media_category_string_key text not null collate nocase
);

create table if not exists MediaTypes (
    media_type_id text primary key collate nocase,
    media_type_string_key text not null collate nocase,
    media_category_id text not null collate nocase,
    foreign key (media_category_id) references MediaCategories(media_category_id)
);

create table if not exists FileExtensions (
    file_extension_tag text primary key collate nocase,
    file_extension_desc_string_key text not null collate nocase
);

create table if not exists MediaTypesForFileExtensions (
    file_extension_tag text not null collate nocase,
    media_type_id text not null collate nocase,
    primary key (file_extension_tag, media_type_id),
    foreign key (file_extension_tag) references FileExtensions(file_extension_tag),
    foreign key (media_type_id) references MediaTypes(media_type_id)
);

create table if not exists Files (
    file_uuid text primary key collate nocase,
    file_name text not null collate nocase,
    file_size_bytes integer not null,
    file_hash text not null collate nocase,
    file_dir_path text not null collate nocase,
    file_extension_tag text not null collate nocase,
    file_encoding text collate nocase,
    media_type_override_id text collate nocase,
    file_deleted integer,
    file_read_only integer,
    file_vfs_path text not null collate rtrim,
    unique (file_vfs_path, file_name),
    foreign key (file_extension_tag) references FileExtensions(file_extension_tag),
    foreign key (media_type_override_id) references MediaTypes(media_type_id)
);

create table if not exists FileBlobs (
    file_uuid text primary key collate nocase,
    blob_value blob,
    foreign key (file_uuid) references Files(file_uuid)
);

create table if not exists Objects (
    object_uuid text primary key collate nocase,
    object_name text not null collate nocase,
    plugin_package_name text not null collate nocase,
    object_deleted integer default 0,
    object_genre text default '' collate nocase,
    object_album_name text default '' collate nocase,
    object_album_position integer default 0,
    object_region text default 'w' collate nocase,
    object_language text default 'en' collate nocase,
    object_artist text default '' collate nocase,
    object_imprint text default '' collate nocase,
    object_publish_timestamp text default '1970-00-00T00:00:00' collate nocase,
    object_website text default '' collate nocase,
    foreign key (object_uuid) references Files(file_uuid)
);

create table if not exists ObjectAttributes (
    object_uuid text not null collate nocase,
    attribute_name text not null collate nocase,
    attribute_value blob,
    primary key (object_uuid, attribute_name),
    foreign key (object_uuid) references Objects(object_uuid)
);

create table if not exists ExtraFilesForObjects (
    object_uuid text not null collate nocase,
    file_uuid text not null collate nocase,
    file_note text not null collate nocase,
    primary key (object_uuid, file_uuid),
    foreign key (object_uuid) references Objects(object_uuid),
    foreign key (file_uuid) references Files(file_uuid)
);

create table if not exists FileArtwork (
    file_uuid text not null collate nocase,
    artwork_file_uuid text not null collate nocase,
    artwork_role text not null collate nocase,
    primary key (file_uuid, artwork_file_uuid, artwork_role),
    foreign key (file_uuid) references Files(file_uuid),
    foreign key (artwork_file_uuid) references Files(file_uuid)
);

create table if not exists Collections (
    collection_uuid text primary key collate nocase,
    collection_name text not null collate nocase,
    collection_visible integer not null collate nocase,
    collection_location text not null collate nocase,
    collection_deleted integer -- bool
);

create table if not exists CollectionHiddenColumns (
    collection_uuid text not null collate nocase,
    column_name text not null collate nocase,
    primary key (collection_uuid, column_name),
    foreign key (collection_uuid) references Collections(collection_uuid)
);

create table if not exists MediaCategoriesForCollections (
    collection_uuid text not null collate nocase,
    media_category_id text not null collate nocase,
    primary key (collection_uuid, media_category_id),
    foreign key (collection_uuid) references Collections(collection_uuid),
    foreign key (media_category_id) references MediaCategories(media_category_id)
);

create table if not exists MediaTypesForCollections (
    collection_uuid text not null collate nocase,
    media_type_id text not null collate nocase,
    primary key (collection_uuid, media_type_id),
    foreign key (collection_uuid) references Collections(collection_uuid),
    foreign key (media_type_id) references MediaTypes(media_type_id)
);

create table if not exists ObjectsInCollections (
    collection_uuid text not null collate nocase,
    index_in_collection integer,
    object_uuid text not null collate nocase,
    primary key (collection_uuid, index_in_collection),
    foreign key (object_uuid) references Objects(object_uuid),
    foreign key (collection_uuid) references Collections(collection_uuid),
    unique(collection_uuid, index_in_collection)
);

create table if not exists Devices (
    device_uuid text primary key collate nocase,
    device_name text not null collate nocase,
    device_description text not null collate nocase,
    device_icon_path text collate nocase
);

create table if not exists DeviceSyncLists (
    device_uuid text not null collate nocase,
    collection_uuid text not null collate nocase,
    plugin_package_name text not null collate nocase,
    dsl_directory_on_device text not null collate nocase,
    last_sync_time integer not null,
    primary key (device_uuid, collection_uuid),
    foreign key (device_uuid) references Devices(device_uuid),
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
