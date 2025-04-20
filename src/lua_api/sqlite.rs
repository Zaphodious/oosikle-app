use crate::db::*;
use rusqlite::{
    params, types::ValueRef, CachedStatement, Connection, Error, OptionalExtension, Params, Result as rResult,
    Row, ToSql,
};
use uuid::Uuid;
use mlua::{Table, Lua, UserData};
use anyhow::Result;

pub fn get_object_as_table(lua: &Lua, conn: &Connection, object_uuid: Uuid) -> Result<Option<Table>> {
    match ObjectRecord::get_from_id(conn, &object_uuid)? {
        Some(ob) => {
            let tab = lua.create_table()?;
            // tab.set("id", object_uuid.as_);
            let attrs = ob.get_attributes(conn)?;

            Ok(Some(tab))
        },
        None => {
            Ok(None)
        }
    }

}

pub struct CollectionMatrix {
    record: CollectionRecord,
    headers: Vec<String>,
    rows: Vec<String>
}

pub fn get_attribute_rows(obj: &ObjectRecord, conn: &Connection, headers: &Vec<&str>) -> Result<Option<Vec<String>>> {
    let mut file_attrs = FileRecord::get_from_id(conn, &obj.uuid)?.expect("There is an object without a file").as_object_attrs()?;
    let name_attr = ObjectAttr { object_uuid: obj.uuid, name: "Name".to_string(), data: AttrValue::STRING(obj.name.clone()) };
    let mut attrs = obj.get_attributes(conn)?;
    attrs.append(&mut file_attrs);
    Ok(Some(headers.iter().map(|column_name| {
        let thingy = attrs.iter().find(|a| {a.name == *column_name});
        match thingy {
            Some(t) => Some(t),
            None => if column_name.to_lowercase() == "name" {
                Some(&name_attr)
            } else {None}
        }
    }).map(|t| {
        match t {
            Some(t) => t.data.to_string(),
            None => "".to_string()
        }
    }).collect()))
}

pub fn get_collection_matrix(conn: &Connection, coll_uuid: Uuid, row_names: &Vec<&str>) -> Result<CollectionMatrix> {
    panic!("Not implimented")
}
