use crate::db::*;
use rusqlite::{
    params, types::ValueRef, CachedStatement, Connection, Error, OptionalExtension, Params, Result as rResult,
    Row, ToSql,
};
use uuid::Uuid;
use mlua::{Table, Lua, UserData};
use anyhow::Result;

pub struct LuaObjectRecord {

}

impl UserData for LuaObjectRecord {}

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