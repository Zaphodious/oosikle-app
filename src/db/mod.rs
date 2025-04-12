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
    fn get_from_id(conn: &Connection, id: &U) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        let mut stmt = conn.prepare_cached(Self::get_fetch_sql())?;
        let record = stmt.query_row([id], Self::from_row).optional()?;
        return Ok(record);
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttrValue {
    STRING(String),
    INT(i64),
    FLOAT(f64),
    NONE,
}

#[derive(Debug, Serialize, Deserialize)]
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
                ValueRef::Blob(b) => {
                    AttrValue::STRING(String::from_utf8(b.to_vec()).expect("A blob went wrong idk"))
                }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRecord {
    pub uuid: Uuid,
    pub name: String,
    pub manager: String,
    pub file_path: String,
    pub deleted: bool,
    pub media_type_override: Option<String>,
}

impl DBSimpleRecord for ObjectRecord {
    fn from_row(row: &Row) -> Result<ObjectRecord, Error> {
        Ok(ObjectRecord {
            uuid: row.get("uuid")?,
            name: row.get("name")?,
            manager: row.get("manager")?,
            file_path: row.get::<&str, String>("file_path")?,
            deleted: row.get("deleted")?,
            media_type_override: row.get("media_type_override")?,
        })
    }
}

impl DBQuickGettable<Uuid> for ObjectRecord {
    fn get_fetch_sql() -> &'static str {
        "select * from ObjectRecordView where ObjectRecordView.uuid = ?1;"
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
}

#[derive(Debug, Serialize, Deserialize)]
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

impl DBQuickGettable<Uuid> for FileRecord {
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

/*
   file_uuid blob primary key,
   file_name text not null,
   file_size_bytes integer not null,
   file_hash text not null,
   file_path text not null,
   file_extension_tag text not null,
   file_encoding text,
   media_type_override_id text,
   file_deleted integer,
   read_only integer,
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsInCollection {
    collection_uuid: Uuid,
    pagesize: usize,
    pageno: usize,
    objects: Vec<ObjectRecord>,
    total_length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
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

impl DBQuickGettable<Uuid> for CollectionRecord {
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
        let mut obj_stmt = conn.prepare_cached("
            select * from ObjectsInCollections
                inner join ObjectRecordView on ObjectRecordView.uuid=ObjectsInCollections.object_uuid
                where ObjectsInCollections.collection_uuid = ?1
                order by ObjectRecordView.name
                limit ?2
                offset ?3;")?;
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
