use rusqlite::{Connection, Error, OptionalExtension, Result};
use uuid::{Uuid, uuid};
use serde_rusqlite;
use serde::{Serialize, Deserialize};
use micromap::Map;

static DB_INIT_SQL: &'static str = include_str!("./init_db.sql");

pub fn init_db(db_loc: &str) -> Result<Connection, Error> {
    let conn = Connection::open(db_loc)?;
    conn.execute_batch(DB_INIT_SQL)?;
    return Ok(conn);
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttrValue {
    STRING {value: String},
    INT {value: i32},
    FLOAT {value: f32},
    BOOL {value: bool}
}

#[derive(Debug, Serialize, Deserialize)]
struct ObjectAttr {
    name: String,
    data: AttrValue
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRecord {
    uuid: Uuid,
    name: String,
    manager: String,
    #[serde(skip)]
    attributes: Option<Box<[AttrValue]>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    uuid: Uuid,
    name: String,
}

pub fn get_object_by_uuid(conn: Connection, the_uuid: Uuid) -> Result<Option<ObjectRecord>, Error>{
    let mut stmt = conn.prepare_cached("select object_name, plugin_package_name from Objects where Objects.object_uuid = ?1")?;
    let record = stmt.query_row([the_uuid], |row| {
        let mut obj = ObjectRecord {
            uuid: the_uuid,
            name: row.get(0)?,
            manager: row.get(1)?,
            attributes: Some(Box::new([]))
        };
        Ok(obj)
    } ).optional()?;
    return Ok(record)
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
    fn getting_a_record() -> Result<(), Error> {
        let conn = init()?;
        let obj = get_object_by_uuid(conn, uuid!("ABADCAFEABADCAFEABADCAFEABADCAFE"))?.expect("There is no entity here");
        assert!(obj.name == "Welcome File");
        return Ok(());
    }
}
