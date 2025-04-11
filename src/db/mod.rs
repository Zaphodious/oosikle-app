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
    /*
    fn get_from_id(conn: &Connection, id: &Uuid)  -> Result<Option<ObjectRecord>, Error> {
        let mut stmt =
        let record = stmt
            .query_row([id], ObjectRecord::from_view)
            .optional()?;
        return Ok(record);
    }*/
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
pub struct FileDescription {
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

impl FileDescription {}
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
    orderby: String,
    pagesize: usize,
    pageno: usize,
    objects: Vec<ObjectRecord>,
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
    /*
    fn get_from_id(conn: &Connection, id: &Uuid) -> Result<Option<CollectionRecord>, Error> {
        let mut coll_stmt = conn.prepare_cached()?;
        let mut obj_stmt = conn.prepare_cached("select * from ObjectRecordView left join ObjectsInCollections on ObjectsInCollections.object_uuid = ObjectRecordView.uuid where ObjectsInCollections.collection_uuid = ?1;")?;
        let coll_rec = coll_stmt.query_row([id], |row| {
            let objects = obj_stmt.query_map([id], ObjectRecord::from_view)?;
            Ok(CollectionRecord {
                uuid: row.get("collection_uuid")?,
                name: row.get("collection_name")?,
                objects: objects.map(|t|t.expect("for now")).collect()
            })
        }).optional()?;
        return Ok(coll_rec);
    }*/
}

impl CollectionRecord {

    fn get_objects(conn: &Connection, id: &Uuid, orderby: &str, pagesize: usize, pageno: usize) -> Result<ObjectsInCollection, Error> {
        let mut obj_stmt = conn.prepare_cached("
            select * from ObjectsInCollections
                inner join ObjectRecordView on ObjectRecordView.uuid=ObjectsInCollections.object_uuid
                where ObjectsInCollections.collection_uuid = ?1
                order by ObjectRecordView.?2
                limit ?3
                offset ?4;")?;
        let coll_rec = obj_stmt.query_map(params![id, orderby, pagesize, pagesize*pageno], ObjectRecord::from_row)?
            .map(|t|t.expect("Should be an object here")).collect();
        Ok(ObjectsInCollection {
            collection_uuid: *id,
            objects: coll_rec,
            orderby: orderby.to_string(),
            pagesize: pagesize,
            pageno: pageno,
        })

    }
}

/*
pub fn get_file_by_uuid(conn: &Connection, the_uuid: &Uuid) -> Result<Option<FileDescription>> {
    let mut coll_stmt = conn.prepare_cached("select * from Collections where Collections.collection_uuid = ?1;")?;

}
*/

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
