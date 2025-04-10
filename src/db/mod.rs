use micromap::Map;
use rusqlite::{types::ValueRef, Connection, Error, OptionalExtension, Result, Row};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use uuid::{uuid, Uuid};

static DB_INIT_SQL: &'static str = include_str!("./init_db.sql");

pub fn init_db(db_loc: &str) -> Result<Connection, Error> {
    let conn = Connection::open(db_loc)?;
    conn.execute_batch(DB_INIT_SQL)?;
    return Ok(conn);
}

pub trait DBRecord<U, T: DBRecord<U,T>> {
    fn get_from_id(id: &U, conn: &Connection) -> Result<Option<T>, Error>;
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

impl DBRecord<(&Uuid, &String), ObjectAttr> for ObjectAttr {
    fn get_from_id(id: &(&Uuid, &String), conn: &Connection) -> Result<Option<ObjectAttr>, Error> {
        let mut stmt = conn.prepare_cached( "select * from ObjectAttributes where ObjectAttributes.object_uuid = ?1 and ObjectAttributes.attribute_name = ?2;")?;
        let record: Option<ObjectAttr> = stmt
            .query_row(*id, ObjectAttr::from_row)
            .optional()?;
        return Ok(record);
    }
}
/*
create table ObjectAttributes (
    object_uuid blob not null,
    attribute_name text not null,
    attribute_value blob,
    primary key (object_uuid, attribute_name),
    foreign key (object_uuid) references Objects(object_uuid)
);
 */

impl ObjectAttr {
    fn from_row(row: &Row) -> Result<ObjectAttr, Error> {
        Ok(ObjectAttr {
            name: row.get("attribute_name")?,
            data: match row.get_ref("attribute_value")? {
                ValueRef::Null => AttrValue::NONE,
                ValueRef::Integer(i) => AttrValue::INT(i),
                ValueRef::Real(f) => AttrValue::FLOAT(f),
                ValueRef::Text(s) => AttrValue::STRING(String::from_utf8(s.to_vec()).expect("A text string was not utf-8")),
                ValueRef::Blob(b) => AttrValue::STRING(String::from_utf8(b.to_vec()).expect("A blob went wrong idk"))
            },
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRecord {
    pub uuid: Uuid,
    pub name: String,
    pub manager: String,
    pub file_path: String,
    pub deleted: bool,
    pub media_type_override: Option<String>
}

impl DBRecord<Uuid, ObjectRecord> for ObjectRecord {
    fn get_from_id(id: &Uuid, conn: &Connection) -> Result<Option<ObjectRecord>, Error> {
        let mut stmt = conn.prepare_cached( "select * from ObjectRecordView where ObjectRecordView.uuid = ?1;")?;
        let record = stmt
            .query_row([id], ObjectRecord::from_view)
            .optional()?;
        return Ok(record);
    }
}

impl ObjectRecord {
    fn from_view(row: &Row) -> Result<ObjectRecord, Error> {
        Ok( ObjectRecord {
            uuid: row.get("uuid")?,
            name: row.get("name")?,
            manager: row.get("manager")?,
            file_path: row.get::<&str, String>("file_path")?,
            deleted: row.get("deleted")?,
            media_type_override: row.get("media_type_override")?
        })
    }

    fn get_attributes (&self, conn: &Connection) -> Result<Vec<ObjectAttr>> {
        let mut stmt = conn.prepare_cached("select * from ObjectAttributes where ObjectAttributes.object_uuid = ?")?;
        let attr_rows = stmt.query_map([&self.uuid], ObjectAttr::from_row)?;
        Ok(attr_rows.map(|t|t.expect("just for now")).collect())
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
    pub read_only: bool

}

impl FileDescription {
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
pub struct CollectionRecord {
    uuid: Uuid,
    name: String,
    objects: Vec<ObjectRecord>,
}

pub fn get_object_by_uuid(
    conn: &Connection,
    the_uuid: &Uuid,
) -> Result<Option<ObjectRecord>, Error> {
    let mut stmt = conn.prepare_cached( "select * from ObjectRecordView where ObjectRecordView.uuid = ?1;")?;
    /*
     *  ##deserializing using query() and from_rows(), the most efficient way
        let mut statement = connection.prepare("SELECT * FROM example").unwrap();
        let mut res = from_rows::<Example>(statement.query([]).unwrap());
        assert_eq!(res.next().unwrap().unwrap(), row1);
        assert_eq!(res.next().unwrap().unwrap(), Example { id: 2, name: "second name".into() });
     */
    let record = stmt
        .query_row([the_uuid], ObjectRecord::from_view)
        .optional()?;
    return Ok(record);
}

pub fn get_collection_by_uuid(conn: &Connection, the_uuid: &Uuid) -> Result<Option<CollectionRecord>> {
    let mut coll_stmt = conn.prepare_cached("select * from Collections where Collections.collection_uuid = ?1;")?;
    let mut obj_stmt = conn.prepare_cached("select * from ObjectRecordView left join ObjectsInCollections on ObjectsInCollections.object_uuid = ObjectRecordView.uuid where ObjectsInCollections.collection_uuid = ?1;")?;
    let coll_rec = coll_stmt.query_row([the_uuid], |row| {
        let objects = obj_stmt.query_map([the_uuid], ObjectRecord::from_view)?;
        Ok(CollectionRecord {
            uuid: row.get("collection_uuid")?,
            name: row.get("collection_name")?,
            objects: objects.map(|t|t.expect("for now")).collect()
        })
    }).optional()?;
    return Ok(coll_rec);
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
        let obj = ObjectRecord::get_from_id(&uuid!("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"), &conn)?
            .expect("There is no entity here");
        assert!(obj.name == "Welcome File");
        return Ok(());
    }

    #[test]
    fn doesnt_get_an_object_that_doesnt_exist() -> Result<(), Error> {
        let conn = init()?;
        let no_obj = get_object_by_uuid(&conn, &uuid!("ABADCAFEABADCAFEABADCAFEABADCAF1"))?;
        if no_obj.is_some() {
            assert!(false, "There should not be an entity with this fake UUID")
        };
        return Ok(());
    }

    #[test]
    fn gets_attributes_for_an_object() -> Result<(), Error> {
        let conn = init()?;
        let attrs = ObjectRecord::get_from_id(&uuid!("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"), &conn)?.expect("There should be an object here").get_attributes(&conn)?;
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
        let attr = ObjectAttr::get_from_id(&(&uuid!("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"), &"author".to_string()), &conn)?.expect("There should be an attribute here");
        if let AttrValue::STRING(_) = attr.data {} else {
            assert!(false);
        }
        return Ok(());
    }

    #[test]
    fn gets_a_collection_by_uuid() -> Result<(), Error> {
        let conn = init()?;
        let coll = get_collection_by_uuid(&conn, &uuid!("BADC0FFEE0DDF00DBADC0FFEE0DDF00D"))?
            .expect("There is no collection here");
        assert!(coll.name == "Default Briefcase");
        return Ok(());
    }

    #[test]
    fn getting_a_collection_by_uuid_gets_objects_in_it() -> Result<(), Error> {
        let conn = init()?;
        let coll = get_collection_by_uuid(&conn, &uuid!("BADC0FFEE0DDF00DBADC0FFEE0DDF00D"))?
            .expect("There is no collection here");
        assert!(coll.objects[0].name == "Welcome File");
        return Ok(());
    }

}
