use rusqlite::{Connection, Error, Result};
use uuid::Uuid;
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


#[cfg(test)]
mod tests {
    static DB_INIT_SQL: &'static str = include_str!("./testing_values.sql");

    use super::*;

    fn init() -> Result<Connection, Error> {
        let conn = init_db("./tmp/test_generated_db.sqlite")?;
        conn.execute_batch(DB_INIT_SQL)?;
        return Ok(conn);
    }

    #[test]
    fn getting_a_record() -> Result<(), Error> {
        let conn = init();

        return Ok(());
    }
}
