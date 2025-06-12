use exemplar::Model;
use hypertext::html_elements::a;
use micromap::Map;
use rusqlite::types::{FromSql, ToSqlOutput, Value};
use rusqlite::{
    params, types::ValueRef, CachedStatement, Connection, Error, OptionalExtension, Params, Result,
    Row, Rows, ToSql,
};
use serde::{Deserialize, Serialize};
use core::fmt;
use std::any::type_name;
use std::fmt::Debug;
use std::vec::Vec;
use std::path::Path;
use time::OffsetDateTime;

mod importer;


static DB_INIT_SQL: &'static str = include_str!("./init_db.sql");

pub fn init_db(db_loc: &str) -> Result<Connection, Error> {
    let conn = Connection::open(db_loc)?;
    conn.execute_batch(DB_INIT_SQL)?;
    return Ok(conn);
}

pub trait WithSQL {
    fn get_fetch_sql() -> &'static str {
        panic!("Type {} does not provide fetch sql", type_name::<Self>());
    }

    fn get_update_sql() -> Option<&'static str> {
        return None;
    }
}

pub trait Fetchable1<U: ToSql>: Model + WithSQL {
    fn get_from_id(conn: &Connection, id: U) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        Ok(conn
            .prepare_cached(Self::get_fetch_sql())?
            .query_row([id], Self::from_row)
            .optional()?)
    }

    fn check_exists(conn: &Connection, id: U) -> Result<bool, Error> {
        let fetch_sql = Self::get_fetch_sql();
        conn.prepare_cached(format!("select exists({fetch_sql}) as 'exists';").as_str())?
            .query_row([id], |r| {
                let foo: bool = r.get("exists")?;
                return Ok(foo);
            })
    }
}

pub trait Fetchable2<U1: ToSql, U2: ToSql>: Model + WithSQL {
    fn get_from_id(conn: &Connection, id1: U1, id2: U2) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        Ok(conn
            .prepare_cached(Self::get_fetch_sql())?
            .query_row(params![id1, id2], Self::from_row)
            .optional()?)
    }

    fn check_exists(conn: &Connection, id1: U1, id2: U2) -> Result<bool, Error> {
        let fetch_sql = Self::get_fetch_sql();
        conn.prepare_cached(format!("select exists({fetch_sql}) as 'exists';").as_str())?
            .query_row(params![id1, id2], |r| {
                let foo: bool = r.get("exists")?;
                return Ok(foo);
            })
    }
}

fn fetch_vec_of<ID: ToSql, THINGY: Model>(
    conn: &Connection,
    id: ID,
    sql: &str,
) -> Result<Vec<THINGY>, Error> {
    let mut stmt = conn.prepare_cached(&sql)?;
    let type_rows = stmt.query_map([id], THINGY::from_row)?;
    Ok(type_rows
        .map(|t| t.expect("No errors permitted here"))
        .collect())
}

fn fetch_specific_vec_of<ID1: ToSql, ID2: ToSql, THINGY: Model>(
    conn: &Connection,
    id1: ID1,
    id2: ID2,
    sql: &str,
) -> Result<Vec<THINGY>, Error> {
    let mut stmt = conn.prepare_cached(&sql)?;
    let type_rows = stmt.query_map(params![id1, id2], THINGY::from_row)?;
    Ok(type_rows
        .map(|t| t.expect("No errors permitted here"))
        .collect())
}

/*
pub trait DBQuickUpdatable<U: ToSql>: DBSimpleRecord + Serialize {
    fn get_update_sql() -> &'static str;
    fn get_insert_sql() -> &'static str;
    fn update(&self, conn: &Connection) -> Result<(), Error>{
        let binding = sr::to_params_named(self)
            .expect("Params building didn't work");
        let params = binding
            .to_slice()
            .as_slice();
        let mut stmt = conn.prepare_cached(Self::get_update_sql())?;
        let res = stmt.execute(params)?;
        Ok(())
    }
}*/

/*/
#[derive(Debug, Serialize, Deserialize, PartialEq, Model)]
#[table("Plugins")]
#[check("./init_db.sql")]
pub struct PluginRecord {
    #[column("plugin_package_name")]
    pub package_name: String,
    #[column("plugin_display_name")]
    pub display_name: String,
    #[column("plugin_version")]
    pub version: usize,
}

impl Fetchable1<&str> for PluginRecord {}
impl WithSQL for PluginRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from Plugins where Plugins.plugin_package_name = ?1 limit 1;"
    }
}

impl PluginRecord {
    pub fn get_associated_types(&self, conn: &Connection) -> Result<Vec<MediaTypeRecord>> {
        fetch_vec_of(
            conn,
            &self.package_name,
            "select MT.* from MediaTypesForPlugins FP
        inner join MediaTypes MT on FP.media_type_id = MT.media_type_id
        where FP.plugin_package_name = ?;",
        )
    }

    pub fn add_type(&self, conn: &Connection, type_record: MediaTypeRecord) -> Result<bool, Error> {
        let mut stmt = conn.prepare_cached(" insert into MediaTypesForPlugins values (?1, ?2);")?; 
        if !MediaTypeRecord::check_exists(conn, &type_record.id)? {
            type_record.insert(conn)?;
            if !MediaTypeRecord::check_exists(conn, &type_record.id)? {
                return Ok(false);
            }
        }
        let rows_effected = stmt.execute(params![self.package_name, type_record.id])?;
        return Ok(rows_effected == 1);
    }
}
    */

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("MediaCategories")]
#[check("./init_db.sql")]
pub struct MediaCategoryRecord {
    pub media_category_id: String,
    pub media_category_string_key: String,
}

impl Fetchable1<&str> for MediaCategoryRecord {}
impl WithSQL for MediaCategoryRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from MediaCategories where MediaCategories.media_category_id = ?1 limit 1;"
    }
}

impl MediaCategoryRecord {
    pub fn get_media_types(&self, conn: &Connection) -> Result<Vec<MediaTypeRecord>> {
        fetch_vec_of(
            conn,
            &self.media_category_id,
            "select * from MediaTypes where MediaTypes.media_category_id = ?",
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("MediaTypes")]
#[check("./init_db.sql")]
pub struct MediaTypeRecord {
    pub media_type_id: String,
    pub media_type_string_key: String,
    pub media_category_id: String,
}

impl Fetchable1<&str> for MediaTypeRecord {}
impl WithSQL for MediaTypeRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from MediaTypes where MediaTypes.media_type_id = ?1 limit 1;"
    }
}

impl MediaTypeRecord {
    pub fn get_category_record(&self, conn: &Connection) -> Result<Option<MediaCategoryRecord>> {
        MediaCategoryRecord::get_from_id(&conn, &self.media_category_id)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("FileExtensions")]
#[check("./init_db.sql")]
pub struct FileExtensionRecord {
    pub file_extension_tag: String,
    pub file_extension_desc_string_key: String,
}

impl Fetchable1<&str> for FileExtensionRecord {}
impl WithSQL for FileExtensionRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from FileExtensions where FileExtensions.file_extension_tag = ?1 limit 1;"
    }
}

impl FileExtensionRecord {
    pub fn get_media_types(&self, conn: &Connection) -> Result<Vec<MediaTypeRecord>> {
        fetch_vec_of(conn, &self.file_extension_tag, "
            select * from MediaTypesForFileExtensions
                inner join MediaTypes on MediaTypesForFileExtensions.media_type_id = MediaTypes.media_type_id
                where MediaTypesForFileExtensions.file_Extension_tag = ?;")
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("MediaTypesForFileExtensions")]
#[check("./init_db.sql")]
pub struct MediaTypeForFileExtensionsRecord {
    pub file_extension_tag: String,
    pub media_type_id: String,
}

impl Fetchable2<&str, &str> for MediaTypeForFileExtensionsRecord {}
impl WithSQL for MediaTypeForFileExtensionsRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from MediaTypesForFileExtensions MT where MT.file_extension_tag = ?1 and MT.media_type_id = ?2 limit 1;"
    }
}



/*
create table MediaTypes (
    media_type_id text primary key,
    media_type_display_name text not null,
    media_category_id text not null,
    foreign key (media_category_id) references MediaCategories(media_category_id)
);
 */

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("Files")]
#[check("./init_db.sql")]
pub struct FileRecord {
    pub file_uuid: String,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub file_hash: String,
    pub file_dir_path: String,
    pub file_extension_tag: String,
    pub file_encoding: String,
    pub media_type_override_id: Option<String>,
    pub file_deleted: bool,
    pub file_read_only: bool,
    pub file_vfs_path: String,
}

impl Fetchable1<&str> for FileRecord {}
impl WithSQL for FileRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from Files where Files.file_uuid = ?1 limit 1"
    }
}

impl FileRecord {
    pub fn get_object_record(&self, conn: &Connection) -> Result<Option<ObjectRecord>> {
        ObjectRecord::get_from_id(conn, &self.file_uuid)
    }
    
    pub fn as_object_attrs(self) -> Result<Vec<ObjectAttr>> {
        Ok(vec![
            ObjectAttr {object_uuid: self.file_uuid.clone(), attribute_name: "filename".to_string(), attribute_value: AttrValue::STRING(self.file_name)},
            ObjectAttr {object_uuid: self.file_uuid.clone(), attribute_name: "size".to_string(), attribute_value: AttrValue::INT(self.file_size_bytes as i64)},
            ObjectAttr {object_uuid: self.file_uuid.clone(), attribute_name: "hash".to_string(), attribute_value: AttrValue::STRING(self.file_hash)},
            ObjectAttr {object_uuid: self.file_uuid.clone(), attribute_name: "dir".to_string(), attribute_value: AttrValue::STRING(self.file_dir_path)},
            ObjectAttr {object_uuid: self.file_uuid.clone(), attribute_name: "extension".to_string(), attribute_value: AttrValue::STRING(self.file_extension_tag)},
            ObjectAttr {object_uuid: self.file_uuid.clone(), attribute_name: "encoding".to_string(), attribute_value: AttrValue::STRING(self.file_encoding)},
            ObjectAttr {object_uuid: self.file_uuid.clone(), attribute_name: "media_type".to_string(), attribute_value: match self.media_type_override_id {
                Some(s) => AttrValue::STRING(s),
                None => AttrValue::NONE
            }},
            ObjectAttr {object_uuid: self.file_uuid.clone(), attribute_name: "read_only".to_string(), attribute_value: AttrValue::INT(if self.file_read_only {1} else {0})},
            ObjectAttr {object_uuid: self.file_uuid.clone(), attribute_name: "id".to_string(), attribute_value: AttrValue::BYTES(self.file_uuid.into_bytes().to_vec())},
        ])
    }


    pub fn get_extension_record(&self, conn: &Connection) -> Result<Option<FileExtensionRecord>> {
        FileExtensionRecord::get_from_id(conn, &self.file_extension_tag)
    }

    pub fn get_override_media_type_record(
        &self,
        conn: &Connection,
    ) -> Result<Option<MediaTypeRecord>> {
        Ok(match &self.media_type_override_id {
            Some(typeid) => MediaTypeRecord::get_from_id(conn, &typeid)?,
            Option::None => None,
        })
    }

    pub fn get_artwork_records(&self, conn: &Connection) -> Result<Vec<FileArtworkRecord>> {
        fetch_vec_of(
            conn,
            &self.file_uuid,
            "select * from FileArtwork FA where FA.file_uuid = ?",
        )
    }

    pub fn get_art_by_role(&self, conn: &Connection, role: &str) -> Result<Option<FileArtworkRecord>> {
        FileArtworkRecord::get_art_by_role(conn, &self.file_uuid, role)
    }

    pub fn get_cover_art(&self, conn: &Connection) -> Result<Option<FileArtworkRecord>> {
        self.get_art_by_role(conn, "cover")
    }

    pub fn get_blob_contents(&self, conn: &Connection) -> Result<Option<Vec<u8>>> {
        let mut stmt = conn.prepare_cached(
            "select FileBlobs.blob_value from FileBlobs where FileBlobs.file_uuid = ?1 limit 1;",
        )?;
        let record = stmt
            .query_row(params![self.file_uuid], |row| {
                let thingy = row.get_ref("blob_value")?;
                Ok(match thingy {
                    // It would be really weird for these first three to be here
                    ValueRef::Null => vec![],
                    ValueRef::Integer(i) => i.to_be_bytes().to_vec(),
                    ValueRef::Real(f) => f.to_be_bytes().to_vec(),
                    // Sometimes it may be a string
                    ValueRef::Text(s) => s.to_vec(),
                    ValueRef::Blob(b) => b.to_vec(),
                })
            })
            .optional()?;
        return Ok(record);
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("FileArtwork")]
#[check("./init_db.sql")]
pub struct FileArtworkRecord {
    pub file_uuid: String,
    pub artwork_file_uuid: String,
    pub artwork_role: String,
}

impl Fetchable2<&str, &str> for FileArtworkRecord {}
impl WithSQL for FileArtworkRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from FileArtwork FA where FA.file_uuid = ?1 and FA.artwork_file_uuid = ?2 limit 1;"
    }
}

impl FileArtworkRecord {
    pub fn get_artwork_file_record(&self, conn: &Connection) -> Result<Option<FileRecord>> {
        FileRecord::get_from_id(conn, &self.artwork_file_uuid)
    }
    pub fn get_art_by_role(conn: &Connection, file_uuid: &str, role: &str) -> Result<Option<FileArtworkRecord>> {
        conn.prepare_cached( "select * from FileArtwork FA where FA.file_uuid = ?1 and FA.artwork_role = ?2 limit 1;")?
        .query_row(params![file_uuid, role], FileArtworkRecord::from_row).optional()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum AttrValue {
    STRING(String),
    INT(i64),
    FLOAT(f64),
    BYTES(Vec<u8>),
    NONE,
}

impl fmt::Display for AttrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttrValue::STRING(t) => fmt::Display::fmt(t, f),
            AttrValue::INT(t) => fmt::Display::fmt(t, f),
            AttrValue::FLOAT(t) => fmt::Display::fmt(t, f),
            AttrValue::BYTES(t) => write!(f, "{:?}", t),
            AttrValue::NONE => write!(f, "")
        }
    }
}

impl FromSql for AttrValue {
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Ok(match value {
            ValueRef::Null => AttrValue::NONE,
            ValueRef::Integer(i) => AttrValue::INT(i),
            ValueRef::Real(f) => AttrValue::FLOAT(f),
            ValueRef::Text(s) => AttrValue::STRING(
                String::from_utf8(s.to_vec()).expect("A text string was not utf-8"),
            ),
            ValueRef::Blob(b) => AttrValue::BYTES(b.to_vec()),
        })
    }
}

impl ToSql for AttrValue {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(match self {
            AttrValue::NONE => ToSqlOutput::Owned(Value::Null),
            AttrValue::INT(i) => ToSqlOutput::Owned(Value::Integer(*i)),
            AttrValue::FLOAT(f) => ToSqlOutput::Owned(Value::Real(*f)),
            AttrValue::STRING(s) => ToSqlOutput::Borrowed(ValueRef::Text(s.as_bytes())),
            AttrValue::BYTES(b) => ToSqlOutput::Borrowed(ValueRef::Blob(b)),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("ObjectAttributes")]
#[check("./init_db.sql")]
pub struct ObjectAttr {
    pub object_uuid: String,
    pub attribute_name: String,
    pub attribute_value: AttrValue,
}

impl Fetchable2<&str, &str> for ObjectAttr {}
impl WithSQL for ObjectAttr {
    fn get_fetch_sql() -> &'static str {
        "select * from ObjectAttributes OA where OA.object_uuid = ?1 and OA.attribute_name = ?2 limit 1;"
    }
}

impl ObjectAttr {
    pub fn get_attributes_for_object_uuid(conn: &Connection, object_uuid: &str) -> Result<Vec<ObjectAttr>> {
        fetch_vec_of(
            conn,
            object_uuid,
            "select * from ObjectAttributes where ObjectAttributes.object_uuid = ?",
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("ObjectAttributes")]
pub struct ObjectExtraFileRecord {
    pub object_uuid: String,
    pub file_uuid: String,
    pub file_note: String,
}

impl Fetchable2<&str, &str> for ObjectExtraFileRecord {}
impl WithSQL for ObjectExtraFileRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from ExtraFilesForObjects EF where EF.object_uuid = ?1 and EF.file_uuid = ?2 limit 1;"
    }
}
impl ObjectExtraFileRecord {
    fn get_file_record(&self, conn: &Connection) -> Result<Option<FileRecord>> {
        FileRecord::get_from_id(conn, &self.file_uuid)
    }
}
/*
impl DBSimpleRecord for ObjectAttr {
    fn from_row(row: &Row) -> Result<ObjectAttr, Error> {
        Ok(ObjectAttr {
            name: row.get("attribute_name")?,
            data: match row.get_ref("attribute_value")? {
                ValueRef::Null => AttrValue::NONE,
                ValueRef::Integer(i) => AttrValue::INT(i),
                ValueRef::Real(f) => AttrValue::FLOAT(f),
                ValueRef::Text(s) => AttrValue::STRING(
                    String::from_utf8(s.to_vec()).expect("A text string was not utf-8"),
                ),
                ValueRef::Blob(b) => AttrValue::BYTES(b.to_vec()),
            },
        })
    }
}*/

/*
impl DBFlatRecord<(&Uuid, &str)> for ObjectAttr {
    fn get_from_id(conn: &Connection, id: &(&Uuid, &str)) -> Result<Option<ObjectAttr>, Error> {
        let mut stmt = conn.prepare_cached( "select * from ObjectAttributes where ObjectAttributes.object_uuid = ?1 and ObjectAttributes.attribute_name = ?2;")?;
        let record: Option<ObjectAttr> = stmt
            .query_row(*id, ObjectAttr::from_row)
            .optional()?;
        return Ok(record);
    }
    fn get_fetch_sql() -> &'static str {
         "select * from ObjectAttributes where ObjectAttributes.object_uuid = ?1 and ObjectAttributes.attribute_name = ?2;"
    }
}*/
/*
create table ObjectAttributes (
    object_uuid blob not null,
    attribute_name text not null,
    attribute_value blob,
    primary key (object_uuid, attribute_name),
    foreign key (object_uuid) references Objects(object_uuid)
);
 */


#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("Objects")]
#[check("./init_db.sql")]
pub struct ObjectRecord {
    pub object_uuid: String,
    pub object_name: String,
    pub plugin_package_name: String,
    pub object_deleted: bool,
    pub object_genre: String,
    pub object_album_name: String,
    pub object_album_position: u32,
    pub object_region: String,
    pub object_language: String,
    pub object_artist: String,
    pub object_imprint: String,
    pub object_publish_timestamp: OffsetDateTime,
    pub object_website: String,
}

/*
impl DBSimpleRecord for ObjectRecord {
    fn from_row(row: &Row) -> Result<ObjectRecord, Error> {
        Ok(ObjectRecord {
            uuid: row.get("object_uuid")?,
            name: row.get("object_name")?,
            manager: row.get("plugin_package_name")?,
            deleted: row.get("object_deleted")?,
        })
    }
}*/

impl Fetchable1<&str> for ObjectRecord {}
impl WithSQL for ObjectRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from Objects where Objects.object_uuid = ?1 limit 1;"
    }
}

impl ObjectRecord {
    pub fn get_attributes(&self, conn: &Connection) -> Result<Vec<ObjectAttr>> {
        fetch_vec_of(
            conn,
            &self.object_uuid,
            "select * from ObjectAttributes where ObjectAttributes.object_uuid = ?",
        )
    }
    pub fn get_attribute(&self, conn: &Connection, name: &str) -> Result<Option<ObjectAttr>> {
        ObjectAttr::get_from_id(conn, &self.object_uuid, name)
    }
    pub fn get_file_record(&self, conn: &Connection) -> Result<Option<FileRecord>> {
        FileRecord::get_from_id(conn, &self.object_uuid)
    }
    pub fn get_override_media_type_record(
        &self,
        conn: &Connection,
    ) -> Result<Option<MediaTypeRecord>> {
        let mut stmt = conn.prepare_cached(
            "
            select MediaTypes.* from MediaTypes
            inner join Files on MediaTypes.media_type_id = Files.media_type_override_id
            where Files.file_uuid = ?1",
        )?;
        let record = stmt
            .query_row(params![self.object_uuid], MediaTypeRecord::from_row)
            .optional()?;
        return Ok(record);
    }
    /*
    pub fn get_manager_plugin_record(
        &self,
        conn: &Connection,
    ) -> Result<Option<PluginRecord>, Error> {
        PluginRecord::get_from_id(conn, &self.manager)
    } */
    pub fn get_extra_files(&self, conn: &Connection) -> Result<Vec<ObjectExtraFileRecord>> {
        fetch_vec_of(
            conn,
            &self.object_uuid,
            "select * from ExtraFilesForObjects EF where EF.object_uuid = ?",
        )
    }
    pub fn get_artwork_records(&self, conn: &Connection) -> Result<Vec<FileArtworkRecord>> {
        fetch_vec_of(
            conn,
            &self.object_uuid,
            "select * from FileArtwork FA where FA.file_uuid = ?",
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("ObjectsInCollections")]
#[check("./init_db.sql")]
pub struct ObjectInCollection {
    pub collection_uuid: String,
    pub index_in_collection: i32,
    pub object_uuid: String,
}

impl Fetchable2<&str, &str> for ObjectInCollection {}
impl WithSQL for ObjectInCollection {
    fn get_fetch_sql() -> &'static str {
        "select * from ObjectsInCollctions OC where OC.collection_uuid = ?1 and OC.object_uuid = ?2 limit 1;"
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PageOfObjectsInCollection {
    pub collection_uuid: String,
    pub pagesize: i64,
    pub pageno: i64,
    pub objects: Vec<ObjectRecord>,
    pub total_length: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("Collections")]
#[check("./init_db.sql")]
pub struct CollectionRecord {
    #[column("collection_uuid")]
    pub uuid: String,
    #[column("collection_name")]
    pub name: String,
    #[column("collection_visible")]
    pub visible: bool,
    #[column("collection_location")]
    pub location: String,
    #[column("collection_deleted")]
    pub deleted: bool,
}

impl Fetchable1<&str> for CollectionRecord {}
impl WithSQL for CollectionRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from Collections where Collections.collection_uuid = ?1 limit 1;"
    }
}

impl CollectionRecord {
    fn get_objects(
        &self,
        conn: &Connection,
        pagesize: i64,
        pageno: i64,
    ) -> Result<PageOfObjectsInCollection, Error> {
        return PageOfObjectsInCollection::get_object_page(conn, &self.uuid, pagesize, pageno);
    }
}

impl PageOfObjectsInCollection {
    pub fn get_next_page(&mut self, conn: &Connection) -> Result<PageOfObjectsInCollection> {
        return PageOfObjectsInCollection::get_object_page(
            conn,
            &self.collection_uuid,
            self.pagesize,
            self.pageno + 1,
        );
    }

    pub fn get_object_page(
        conn: &Connection,
        collection_id: &str,
        pagesize: i64,
        pageno: i64,
    ) -> Result<PageOfObjectsInCollection, Error> {
        let mut obj_stmt = conn.prepare_cached(
            "
            select * from ObjectsInCollections OC
                inner join Objects on Objects.object_uuid=OC.object_uuid
                inner join Files on Files.file_uuid=Objects.object_uuid
                where OC.collection_uuid = ?1
                and Objects.object_deleted=0
                and Files.file_deleted=0
                order by OC.index_in_collection, Objects.object_name
                limit ?2
                offset ?3;",
        )?;
        let mut total_length_stmt = conn.prepare_cached("select count(*) from ObjectsInCollections where ObjectsInCollections.collection_uuid = ?1")?;
        let total_length = total_length_stmt.query_row([collection_id], |r| Ok(r.get(0)?))?;
        let objects = obj_stmt
            .query_map(
                params![collection_id, pagesize, pagesize * pageno],
                ObjectRecord::from_row,
            )?
            .map(|t| t.expect("Should be an object here"))
            .collect();
        Ok(PageOfObjectsInCollection {
            collection_uuid: collection_id.to_string(),
            objects,
            pagesize,
            pageno,
            total_length,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("Devices")]
#[check("./init_db.sql")]
pub struct DeviceRecord {
    pub device_uuid: String,
    pub device_name: String,
    pub device_description: String,
    pub device_icon_path: Option<String>,
}

impl Fetchable1<&str> for DeviceRecord {}
impl WithSQL for DeviceRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from Devices where Devices.device_uuid = ?1 limit 1;"
    }
}

impl DeviceRecord {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Model, Clone)]
#[table("DeviceSyncLists")]
#[check("./init_db.sql")]
pub struct DeviceSyncListRecord {
    pub device_uuid: String,
    pub collection_uuid: String,
    pub plugin_package_name: String,
    pub dsl_directory_on_device: String,
    pub last_sync_time: i32,
}

impl Fetchable2<&str, &str> for DeviceSyncListRecord {}
impl WithSQL for DeviceSyncListRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from DeviceSyncLists DSL where DSL.device_uuid = ?1 and DSL.collection_uuid = ?2 limit 1;"
    }
}

/*
create table DeviceSyncLists (
    device_uuid blob not null,
    collection_uuid blob not null,
    plugin_package_name text not null,
    dsl_directory_on_device text not null,
    last_sync_time integer not null,
    primary key (device_uuid, collection_uuid),
    foreign key (device_uuid) references Devices(device_uuid),
    foreign key (plugin_package_name) references Plugins(plugin_package_name),
    foreign key (collection_uuid) references Collections(collection_uuid)
);
     */

#[cfg(test)]
mod upsert_tests {
    static TESTING_VALUES: &'static str = include_str!("../testing_data/sql/testing_values.sql");

    use super::*;

    fn init() -> Result<Connection, Error> {
        //let conn = init_db("./tmp/test_generated_db.sqlite")?;
        let conn = init_db(":memory:")?;
        conn.execute_batch(TESTING_VALUES)?;
        return Ok(conn);
    }

    #[test]
    fn foo() -> Result<(), Error> {
        let conn = init()?;
        return Ok(());
    }
}

#[cfg(test)]
mod simple_read_tests {
    static TESTING_VALUES: &'static str = include_str!("../testing_data/sql/testing_values.sql");

    use super::*;

    fn init() -> Result<Connection, Error> {
        //let conn = init_db("./tmp/test_generated_db.sqlite")?;
        let conn = init_db(":memory:")?;
        conn.execute_batch(TESTING_VALUES)?;
        return Ok(conn);
    }

    #[test]
    fn plain_sql_works() -> Result<(), Error> {
        let conn = init()?;
        let the_query = "select * from Objects where Objects.object_uuid='DEADBEEFDEADBEEFDEADBEEFDEADBEEF';";
        let mut stmt = conn.prepare_cached(the_query)?;
        let mut res1 = stmt.query([])?;
        res1.next().unwrap();
        return Ok(());
    }

    #[test]
    fn gets_media_category_by_id() -> Result<(), Error> {
        let conn = init()?;
        let mcat = MediaCategoryRecord::get_from_id(&conn, "DOCUMENT")?
            .expect("Document category should exsit");
        assert!(mcat.media_category_string_key == "media_category_document");
        return Ok(());
    }

    #[test]
    fn gets_media_type_by_id() -> Result<(), Error> {
        let conn = init()?;
        let mtype =
            MediaTypeRecord::get_from_id(&conn, "PLAINTEXT")?.expect("Plaintext type should exsit");
        assert!(mtype.media_type_string_key == "media_type_plain_text");
        return Ok(());
    }

    #[test]
    fn media_category_and_type_gets_each_other() -> Result<(), Error> {
        let conn = init()?;
        let mtype =
            MediaTypeRecord::get_from_id(&conn, "PLAINTEXT")?.expect("Plaintext type should exsit");
        let mcat = &mtype
            .get_category_record(&conn)?
            .expect("category should exist");
        let mtype_vec = &mcat.get_media_types(&conn)?;
        assert!(mtype_vec.contains(&mtype)); 
        return Ok(());
    }

    #[test]
    fn gets_an_object_by_uuid() -> Result<(), Error> {
        let conn = init()?;
        let obj = ObjectRecord::get_from_id(&conn, ("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?
            .expect("There is no entity here");
        assert!(obj.object_name == "Welcome File");
        return Ok(());
    }

    #[test]
    fn doesnt_get_an_object_that_doesnt_exist() -> Result<(), Error> {
        let conn = init()?;
        let no_obj = ObjectRecord::get_from_id(&conn, ("ABADCAFEABADCAFEABADCAFEABADCAF1"))?;
        if no_obj.is_some() {
            assert!(false, "There should not be an entity with this fake UUID")
        };
        return Ok(());
    }

    #[test]
    fn gets_attributes_for_an_object() -> Result<(), Error> {
        let conn = init()?;
        let attrs = ObjectRecord::get_from_id(&conn, ("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?
            .expect("There should be an object here")
            .get_attributes(&conn)?;
        //let attrs = get_attributes_for_object(&conn, &uuid!("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?;
        assert!(attrs[0].attribute_name.len() != 0);
        assert!(attrs[1].attribute_name.len() != 0);
        assert!(attrs[2].attribute_name.len() != 0);
        return Ok(());
    }

    #[test]
    fn gets_a_spcific_attribute_for_an_object() -> Result<(), Error> {
        let conn = init()?;
        let attr = ObjectRecord::get_from_id(&conn, ("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?
            .expect("There should be an object here")
            .get_attribute(&conn, "REVISION")?
            .expect("There should be an attribute here");
        if let AttrValue::INT(_) = attr.attribute_value {
        } else {
            assert!(false);
        }
        return Ok(());
    }

    #[test]
    fn gets_type_override_for_object() -> Result<(), Error> {
        let conn = init()?;
        let mt = ObjectRecord::get_from_id(&conn, "DEADBEEFDEADBEEFDEADBEEFDEADBEEF")?
            .expect("There should be an object here")
            .get_override_media_type_record(&conn)?;
        assert!(mt == None);
        Ok(())
    }

    #[test]
    fn gets_a_collection_by_uuid() -> Result<(), Error> {
        let conn = init()?;
        let coll =
            CollectionRecord::get_from_id(&conn, ("BADC0FFEE0DDF00DBADC0FFEE0DDF00D"))?
                .expect("There is no collection here");
        assert!(coll.name == "Default Briefcase");
        return Ok(());
    }

    #[test]
    fn gets_objects_in_collection() -> Result<(), Error> {
        let conn = init()?;
        let objcol =
            CollectionRecord::get_from_id(&conn, ("BADC0FFEE0DDF00DBADC0FFEE0DDF00D"))?
                .expect("There is no collection here")
                .get_objects(&conn, 10, 0)?;
        assert!(objcol.total_length == 1);
        assert!(objcol.objects[0].object_name == "Welcome File");
        return Ok(());
    }

    #[test]
    fn gets_a_file_by_uuid() -> Result<(), Error> {
        let conn = init()?;
        let fr = FileRecord::get_from_id(&conn, "DEADBEEFDEADBEEFDEADBEEFDEADBEEF")?
            .expect("There is no entity here");
        assert!(fr.file_name == "welcome.txt");
        return Ok(());
    }

    #[test]
    fn file_gets_blob() -> Result<(), Error> {
        let conn = init()?;
        let fr = FileRecord::get_from_id(&conn, ("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?
            .expect("There is no entity here");
        let blob = fr.get_blob_contents(&conn)?;
        assert!(blob == Some("Welcome to Oosikle!".as_bytes().to_vec()));
        return Ok(());
    }

    #[test]
    fn file_gets_extension_gets_types() -> Result<(), Error> {
        let conn = init()?;
        let fr = FileRecord::get_from_id(&conn, ("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?
            .expect("There is no entity here");
        let rec = fr
            .get_extension_record(&conn)?
            .expect("There should be an extension record here");
        assert!(rec.file_extension_desc_string_key == "file_ext_txt");
        let types = rec.get_media_types(&conn)?;
        assert!(types[1].media_type_string_key != "");
        return Ok(());
    }

    /*
    #[test]
    fn gets_plugin_record_gets_types() -> Result<(), Error> {
        let conn = init()?;
        let fr = PluginRecord::get_from_id(&conn, "oosikle.adapter.text")?
            .expect("There is no entity here");
        assert!(fr.display_name == "Default Text File Manager");
        let types = fr.get_associated_types(&conn)?;
        assert!(types[1].display_name != "");
        return Ok(());
    } */

    #[test]
    fn gets_device_record() -> Result<(), Error> {
        let conn = init()?;
        let dr = DeviceRecord::get_from_id(&conn, ("0DE2C3400DE2C3400DE2C3400DE2C340"))?
            .expect("There is no entity here");
        assert!(dr.device_name == "Example Flash Drive");
        Ok(())
    }

    /*
    #[test]
    fn getting_a_collection_by_uuid_gets_objects_in_it() -> Result<(), Error> {
        let conn = init()?;
        let coll =
            CollectionRecord::get_from_id(&conn, &uuid!("BADC0FFEE0DDF00DBADC0FFEE0DDF00D"))?
                .expect("There is no collection here");
        assert!(coll.objects[0].name == "Welcome File");
        return Ok(());
    } */
}
