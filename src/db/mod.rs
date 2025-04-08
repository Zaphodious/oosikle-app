use micromap::Map;
use rusqlite::{types::ValueRef, Connection, OptionalExtension, Result};
use serde::{Deserialize, Serialize};
use serde_rusqlite::*;
use std::vec::Vec;
use uuid::{uuid, Uuid};

static DB_INIT_SQL: &'static str = include_str!("./init_db.sql");

pub fn init_db(db_loc: &str) -> Result<Connection, Error> {
    let conn = Connection::open(db_loc)?;
    conn.execute_batch(DB_INIT_SQL)?;
    return Ok(conn);
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRecord {
    uuid: Uuid,
    name: String,
    manager: String,
    file_path: String,
    deleted: bool,
    set_media_type: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    uuid: Uuid,
    name: String,
}

pub fn get_object_by_uuid(
    conn: &Connection,
    the_uuid: &Uuid,
) -> Result<Option<ObjectRecord>, Error> {
    let mut stmt = conn.prepare_cached( "
select 
	file_uuid as uuid, 
	object_name as name, 
	plugin_package_name as manager, 
	file_path,
	object_deleted as deleted,
	media_type_override_id as set_media_type
from Objects left join Files on Objects.object_uuid = Files.file_uuid 
	where Files.file_deleted = 0 and Objects.object_uuid = ?1;
",
    )?;
    /*
     *  ##deserializing using query() and from_rows(), the most efficient way
        let mut statement = connection.prepare("SELECT * FROM example").unwrap();
        let mut res = from_rows::<Example>(statement.query([]).unwrap());
        assert_eq!(res.next().unwrap().unwrap(), row1);
        assert_eq!(res.next().unwrap().unwrap(), Example { id: 2, name: "second name".into() });
     */
    let record = stmt
        .query_row([the_uuid], |row| {
            Ok( ObjectRecord {
                uuid: row.get("uuid")?,
                name: row.get("name")?,
                manager: row.get("manager")?,
                file_path: row.get::<&str, String>("file_path")?,
                deleted: row.get("deleted")?,
                set_media_type: row.get("set_media_type")?
            })
        })
        .optional()?;
    return Ok(record);
}

pub fn get_attributes_for_object(conn: &Connection, the_uuid: &Uuid) -> Result<Vec<ObjectAttr>> {
    let mut stmt = conn.prepare_cached("select attribute_name, attribute_value from ObjectAttributes where ObjectAttributes.object_uuid = ?")?;
    let attr_rows = stmt.query_map([the_uuid], |row| {
        Ok(ObjectAttr {
            name: row.get(0)?,
            data: match row.get_ref(1)? {
                ValueRef::Null => AttrValue::NONE,
                ValueRef::Integer(i) => AttrValue::INT(i),
                ValueRef::Real(f) => AttrValue::FLOAT(f),
                ValueRef::Text(s) => AttrValue::STRING(String::from_utf8(s.to_vec()).expect("A text string was not utf-8")),
                ValueRef::Blob(b) => AttrValue::STRING(String::from_utf8(b.to_vec()).expect("A blob went wrong idk"))
            },
        })
    })?;
    Ok(attr_rows.map(|t|t.expect("just for now")).collect())
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
        let obj = get_object_by_uuid(&conn, &uuid!("DEADBEEFDEADBEEFDEADBEEFDEADBEEF"))?
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
}
