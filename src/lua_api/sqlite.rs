use crate::db::*;
use exemplar::Model;
use hypertext::html_elements::object;
use rusqlite::{
    params, params_from_iter, types::ValueRef, types::{ToSqlOutput, Value as rValue}, CachedStatement, Connection, Error, OptionalExtension, Params, ParamsFromIter, Result as rResult, Row, ToSql
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use mlua::{ExternalResult, FromLua, IntoLua, Lua, Result as luaResult, Table, UserData, Value};
use anyhow::Result;

pub struct SQLua {conn: Connection, sql_str: Option<String>}

impl SQLua {
    pub fn init(db_loc: &str) -> Result<SQLua> {
        Ok(SQLua{
            conn: init_db(db_loc)?,
            sql_str: None
        })
    }
}

fn value_to_lua(lua: &Lua, value: ValueRef) -> luaResult<Value> {
    match value {
        ValueRef::Text(s) => s.into_lua(lua),
        ValueRef::Integer(s) => s.into_lua(lua),
        ValueRef::Real(s) => s.into_lua(lua),
        ValueRef::Blob(s) => s.into_lua(lua),
        ValueRef::Null => todo!(),
    }
}

pub struct ValueWrap(Value);

impl ToSql for ValueWrap {
    fn to_sql(&self) -> rResult<rusqlite::types::ToSqlOutput<'_>> {
        match &self.0 {
            Value::Boolean(s) => Ok(ToSqlOutput::Owned(rValue::Integer(if *s {1} else {0}))),
            Value::Integer(s) => s.to_sql(),
            Value::Number(s) => s.to_sql(),
            Value::String(s) => Ok(ToSqlOutput::Owned(rValue::Text(s.to_string_lossy()))),
            _ => Ok(ToSqlOutput::Owned(rValue::Null)),
        }
    }
}

impl FromLua for ValueWrap {
    fn from_lua(value: Value, lua: &Lua) -> luaResult<Self> {
        Ok(ValueWrap(value))
    }
}

impl SQLua {
    pub fn set_sql_string(lua: &Lua, this: &mut SQLua, sqlstring: String) -> luaResult<()> {
        this.sql_str = Some(sqlstring);
        Ok(())
    }
    pub fn query(lua: &Lua, this: &SQLua, params: Vec<ValueWrap>) -> luaResult<Vec<Table>> {
        let mut stmt = this.conn.prepare_cached(this.sql_str.clone().or(Some("".to_string())).expect("see prev").as_str()).into_lua_err()?;
        let headers = (&mut stmt).column_names().iter().map(|s|s.to_string()).collect::<Vec<String>>();
        /*
        let mut retvec: Vec<Table> = Vec::new();
        let mut rows = stmt.query(params)?;
        while let Some(row) = (&rows).next()? {
            for head in &headers {
                let t = lua.create_table().unwrap();
                t.set(head.into_lua(lua)?, value_to_lua(lua, row.get_ref_unwrap(head.as_str()))?);
            }
        }
 */
        let res = (stmt).query_map(params_from_iter(params), |r| {
            let mut t = lua.create_table().expect("If lua cannot make a table, the app cannot continue");
            for head in &headers {
                t.set(head.as_str(), value_to_lua(lua, r.get_ref_unwrap(head.as_str())).unwrap()).unwrap();
            }
            Ok(t)
        }).map_err(mlua::Error::external)?.filter_map(|r| r.ok()).collect::<Vec<Table>>();
        Ok(res)
    }
}

impl UserData for SQLua {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("query", SQLua::query);
        methods.add_method_mut("set_sql_string", SQLua::set_sql_string);
    }
}

impl UserData for CollectionRecord {}

#[cfg(test)]
mod test {
    use super::*;
    use rusqlite::Connection;
    use crate::{db, lua_api};

    fn init() -> (Lua, Connection) {
        let lua = lua_api::init(None).unwrap();
        let conn = db::init_db(":memory:").unwrap();
        (lua, conn)
    }
}
