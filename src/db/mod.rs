use micromap::Map;
use rusqlite::{
    params, types::ValueRef, CachedStatement, Connection, Error, OptionalExtension, Params, Result,
    Row, ToSql,
};
use serde::{Deserialize, Serialize};
use serde_rusqlite as sr;
use std::vec::Vec;
use uuid::{uuid, Uuid};

static DB_INIT_SQL: &'static str = include_str!("./init_db.sql");

pub fn init_db(db_loc: &str) -> Result<Connection, Error> {
    let conn = Connection::open(db_loc)?;
    conn.execute_batch(DB_INIT_SQL)?;
    return Ok(conn);
}

pub trait DBSimpleRecord {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait DBQuickGettable<U: ToSql>: DBSimpleRecord {
    fn get_fetch_sql() -> &'static str;
    fn get_from_id(conn: &Connection, id: U) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        let mut stmt = conn.prepare_cached(Self::get_fetch_sql())?;
        let record = stmt.query_row([id], Self::from_row).optional()?;
        return Ok(record);
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MediaCategoryRecord {
    id: String,
    display_name: String,
}

impl DBQuickGettable<&str> for MediaCategoryRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from MediaCategories where MediaCategories.media_category_id = ?1"
    }
}

impl DBSimpleRecord for MediaCategoryRecord {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(MediaCategoryRecord {
            id: row.get("media_category_id")?,
            display_name: row.get("media_category_display_name")?,
        })
    }
}

impl MediaCategoryRecord {
    pub fn get_media_types(&self, conn: &Connection) -> Result<Vec<MediaTypeRecord>> {
        let mut stmt =
            conn.prepare_cached("select * from MediaTypes where MediaTypes.media_category_id = ?")?;
        let type_rows = stmt.query_map([&self.id], MediaTypeRecord::from_row)?;
        Ok(type_rows.map(|t| t.expect("just for now")).collect())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MediaTypeRecord {
    id: String,
    display_name: String,
    media_category_id: String,
}

impl DBQuickGettable<&str> for MediaTypeRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from MediaTypes where MediaTypes.media_type_id = ?1;"
    }
}

impl DBSimpleRecord for MediaTypeRecord {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(MediaTypeRecord {
            id: row.get("media_type_id")?,
            display_name: row.get("media_type_display_name")?,
            media_category_id: row.get("media_category_id")?,
        })
    }
}

impl MediaTypeRecord {
    pub fn get_category_record(&self, conn: &Connection) -> Result<Option<MediaCategoryRecord>> {
        MediaCategoryRecord::get_from_id(&conn, &self.media_category_id)
    }
}

pub struct FileExtensionRecord {
    tag: String,
    description: String,
}

impl DBQuickGettable<&str> for FileExtensionRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from FileExtensions where FileExtensions.file_extension_tag = ?1;"
    }
}

impl DBSimpleRecord for FileExtensionRecord {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(FileExtensionRecord {
            tag: row.get("file_extension_tag")?,
            description: row.get("file_extension_description")?,
        })
    }
}

impl FileExtensionRecord {
    fn get_media_types(&self, conn: &Connection) -> Result<Vec<MediaTypeRecord>> {
        let mut stmt =
            conn.prepare_cached("
            select * from MediaTypesForFileExtensions
                inner join MediaTypes on MediaTypesForFileExtensions.media_type_id = MediaTypes.media_type_id
                where MediaTypesForFileExtensions.file_Extension_tag = ?;")?;
        let type_rows = stmt.query_map([&self.tag], MediaTypeRecord::from_row)?;
        Ok(type_rows.map(|t| t.expect("just for now")).collect())
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FileRecord {
    pub uuid: Uuid,
    pub name: String,
    pub size_bytes: usize,
    pub hash: String,
    pub path: String,
    pub extension_tag: String,
    pub encoding: String,
    pub media_type_override: Option<String>,
    pub deleted: bool,
    pub read_only: bool,
}

impl DBQuickGettable<&Uuid> for FileRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from Files where Files.file_uuid = ?1"
    }
}

impl DBSimpleRecord for FileRecord {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(FileRecord {
            uuid: row.get("file_uuid")?,
            name: row.get("file_name")?,
            size_bytes: row.get("file_size_bytes")?,
            hash: row.get("file_hash")?,
            path: row.get("file_path")?,
            extension_tag: row.get("file_extension_tag")?,
            encoding: row.get("file_encoding")?,
            media_type_override: row.get("media_type_override_id")?,
            deleted: row.get("file_deleted")?,
            read_only: row.get("read_only")?,
        })
    }
}

impl FileRecord {
    fn get_object_record(&self, conn: &Connection) -> Result<Option<ObjectRecord>> {
        ObjectRecord::get_from_id(conn, &self.uuid)
    }

    fn get_extension_record(&self, conn: &Connection) -> Result<Option<FileExtensionRecord>> {
        FileExtensionRecord::get_from_id(conn, &self.extension_tag)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AttrValue {
    STRING(String),
    INT(i64),
    FLOAT(f64),
    BYTES(Vec<u8>),
    NONE,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ObjectAttr {
    name: String,
    data: AttrValue,
}

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
}

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

impl ObjectAttr {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ObjectRecord {
    pub uuid: Uuid,
    pub name: String,
    pub manager: String,
    pub deleted: bool,
}

impl DBSimpleRecord for ObjectRecord {
    fn from_row(row: &Row) -> Result<ObjectRecord, Error> {
        Ok(ObjectRecord {
            uuid: row.get("object_uuid")?,
            name: row.get("object_name")?,
            manager: row.get("plugin_package_name")?,
            deleted: row.get("object_deleted")?,
        })
    }
}

impl DBQuickGettable<&Uuid> for ObjectRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from Objects where Objects.object_uuid = ?1;"
    }
}

impl ObjectRecord {
    fn get_attributes(&self, conn: &Connection) -> Result<Vec<ObjectAttr>> {
        let mut stmt = conn.prepare_cached(
            "select * from ObjectAttributes where ObjectAttributes.object_uuid = ?",
        )?;
        let attr_rows = stmt.query_map([&self.uuid], ObjectAttr::from_row)?;
        Ok(attr_rows.map(|t| t.expect("just for now")).collect())
    }
    fn get_attribute(&self, conn: &Connection, name: &str) -> Result<Option<ObjectAttr>> {
        let mut stmt = conn.prepare_cached("select * from ObjectAttributes where ObjectAttributes.object_uuid = ?1 and ObjectAttributes.attribute_name = ?2")?;
        let record = stmt
            .query_row(params![self.uuid, name], ObjectAttr::from_row)
            .optional()?;
        return Ok(record);
    }
    fn get_file_record(&self, conn: &Connection) -> Result<Option<FileRecord>> {
        FileRecord::get_from_id(conn, &self.uuid)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ObjectsInCollection {
    collection_uuid: Uuid,
    pagesize: usize,
    pageno: usize,
    objects: Vec<ObjectRecord>,
    total_length: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CollectionRecord {
    uuid: Uuid,
    name: String,
}

impl DBSimpleRecord for CollectionRecord {
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(CollectionRecord {
            uuid: row.get("collection_uuid")?,
            name: row.get("collection_name")?,
        })
    }
}

impl DBQuickGettable<&Uuid> for CollectionRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from Collections where Collections.collection_uuid = ?1;"
    }
}

impl CollectionRecord {
    fn get_objects(
        &self,
        conn: &Connection,
        pagesize: usize,
        pageno: usize,
    ) -> Result<ObjectsInCollection, Error> {
        return ObjectsInCollection::get_object_page(conn, &self.uuid, pagesize, pageno);
    }
}

impl ObjectsInCollection {
    fn get_next_page(&mut self, conn: &Connection) -> Result<ObjectsInCollection> {
        return ObjectsInCollection::get_object_page(
            conn,
            &self.collection_uuid,
            self.pagesize,
            self.pageno + 1,
        );
    }

    fn get_object_page(
        conn: &Connection,
        collection_id: &Uuid,
        pagesize: usize,
        pageno: usize,
    ) -> Result<ObjectsInCollection, Error> {
        let mut obj_stmt = conn.prepare_cached(
            "
            select * from ObjectsInCollections
                inner join Objects on Objects.object_uuid=ObjectsInCollections.object_uuid
                where ObjectsInCollections.collection_uuid = ?1
                order by Objects.object_name
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
        Ok(ObjectsInCollection {
            collection_uuid: *collection_id,
            objects,
            pagesize,
            pageno,
            total_length,
        })
    }
}

#[cfg(test)]
mod tests {
    static DB_INIT_SQL: &'static str = include_str!("./testing_values.sql");

    use super::*;

    fn init() -> Result<Connection, Error> {
        //let conn = init_db("./tmp/test_generated_db.sqlite")?;
        let conn = init_db(":memory:")?;
        conn.execute_batch(DB_INIT_SQL)?;
        return Ok(conn);
    }

    #[test]
    fn gets_media_category_by_id() -> Result<(), Error> {
        let conn = init()?;
        let mcat = MediaCategoryRecord::get_from_id(&conn, "DOCUMENT")?
            .expect("Document category should exsit");
        assert!(mcat.display_name == "Document");
        return Ok(());
    }

    #[test]
    fn gets_media_type_by_id() -> Result<(), Error> {
        let conn = init()?;
        let mtype = MediaTypeRecord::get_from_id(&conn, "PLAINTEXT")?
            .expect("Plaintext type should exsit");
        assert!(mtype.display_name == "Plain Text File");
        return Ok(());
    }

    #[test]
    fn media_category_and_type_gets_each_other() -> Result<(), Error> {
        let conn = init()?;
        let mtype = MediaTypeRecord::get_from_id(&conn, "PLAINTEXT")?
            .expect("Plaintext type should exsit");
        let mcat = &mtype.get_category_record(&conn)?.expect("category should exist");
        let mtype2 = &mcat.get_media_types(&conn)?[0];
        assert!(mtype == *mtype2);
        return Ok(());
    }

    #[test]
    fn gets_an_object_by_uuid() -> Result<(), Error> {
        let conn = init()?;
        let obj = ObjectRecord::get_from_id(&conn, &uuid!("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?
            .expect("There is no entity here");
        assert!(obj.name == "Welcome File");
        return Ok(());
    }

    #[test]
    fn doesnt_get_an_object_that_doesnt_exist() -> Result<(), Error> {
        let conn = init()?;
        let no_obj = ObjectRecord::get_from_id(&conn, &uuid!("ABADCAFEABADCAFEABADCAFEABADCAF1"))?;
        if no_obj.is_some() {
            assert!(false, "There should not be an entity with this fake UUID")
        };
        return Ok(());
    }

    #[test]
    fn gets_attributes_for_an_object() -> Result<(), Error> {
        let conn = init()?;
        let attrs = ObjectRecord::get_from_id(&conn, &uuid!("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?
            .expect("There should be an object here")
            .get_attributes(&conn)?;
        //let attrs = get_attributes_for_object(&conn, &uuid!("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?;
        assert!(attrs[0].name.len() != 0);
        assert!(attrs[1].name.len() != 0);
        assert!(attrs[2].name.len() != 0);
        assert!(attrs[3].name.len() != 0);
        return Ok(());
    }

    #[test]
    fn gets_a_spcific_attribute_for_an_object() -> Result<(), Error> {
        let conn = init()?;
        let attr = ObjectRecord::get_from_id(&conn, &uuid!("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?
            .expect("There should be an object here")
            .get_attribute(&conn, "author")?
            .expect("There should be an attribute here");
        if let AttrValue::STRING(_) = attr.data {
        } else {
            assert!(false);
        }
        return Ok(());
    }

    #[test]
    fn gets_a_collection_by_uuid() -> Result<(), Error> {
        let conn = init()?;
        let coll =
            CollectionRecord::get_from_id(&conn, &uuid!("BADC0FFEE0DDF00DBADC0FFEE0DDF00D"))?
                .expect("There is no collection here");
        assert!(coll.name == "Default Briefcase");
        return Ok(());
    }

    #[test]
    fn gets_objects_in_collection() -> Result<(), Error> {
        let conn = init()?;
        let objcol =
            CollectionRecord::get_from_id(&conn, &uuid!("BADC0FFEE0DDF00DBADC0FFEE0DDF00D"))?
                .expect("There is no collection here")
                .get_objects(&conn, 10, 0)?;
        assert!(objcol.total_length == 1);
        assert!(objcol.objects[0].name == "Welcome File");
        return Ok(());
    }

    #[test]
    fn gets_a_file_by_uuid() -> Result<(), Error> {
        let conn = init()?;
        let fr = FileRecord::get_from_id(&conn, &uuid!("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?
            .expect("There is no entity here");
        assert!(fr.name == "welcome.txt");
        return Ok(());
    }

    #[test]
    fn file_gets_extension_gets_types() -> Result<(), Error> {
        let conn = init()?;
        let fr = FileRecord::get_from_id(&conn, &uuid!("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?
            .expect("There is no entity here");
        let rec = fr.get_extension_record(&conn)?.expect("There should be an extension record here");
        assert!(rec.description == "Ordinary text file");
        let types = rec.get_media_types(&conn)?;
        assert!(types[0].display_name == "Plain Text File");
        return Ok(());
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
