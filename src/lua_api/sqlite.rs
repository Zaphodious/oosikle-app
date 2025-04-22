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
use std::fmt;

#[derive(Debug)]
pub struct SQLua {conn: Connection, sql_str: Option<String>}

impl SQLua {
    pub fn init(conn: Connection) -> Result<Self> {
        let this = SQLua{
            conn,
            sql_str: None
        };
        Ok(this)
    }

    pub fn inject_into(self, lua: &Lua) -> Result<()> {
        lua.globals().set("SQLua", self)?;
        Ok(())
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
    pub fn with_sql(lua: &Lua, this: &mut Self, sqlstring: String) -> luaResult<()> {
        this.sql_str = Some(sqlstring);
        Ok(())
    }
    pub fn query(lua: &Lua, this: &SQLua, params: Vec<ValueWrap>) -> luaResult<Vec<Table>> {
        let mut stmt = this.conn.prepare_cached(this.sql_str.clone().or(Some("".to_string())).expect("see prev").as_str()).into_lua_err()?;
        let headers = (stmt).column_names().iter().map(|s|s.to_string()).collect::<Vec<String>>();
        let res = (stmt).query_map(params_from_iter(params), |r| {
            let t = lua.create_table().expect("If lua cannot make a table, the app cannot continue");
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
        methods.add_method_mut("with_sql", SQLua::with_sql);
    }
}

impl UserData for CollectionRecord {}

#[cfg(test)]
mod test {
    use super::*;
    use rusqlite::Connection;
    use crate::{db, lua_api};

    static TESTING_VALUES: &'static str = include_str!("./many_testing_values.sql");
    static TESTING_LUA: &'static str = include_str!("./sqlua_testing.lua");


    fn init() -> Lua {
        let conn = db::init_db(":memory:").unwrap();
        conn.execute(TESTING_VALUES, []).unwrap();

        let mut lua = lua_api::init(None).unwrap();
        let _ = lua.load(TESTING_LUA).exec();

        SQLua::init(conn).unwrap().inject_into(&lua).unwrap();
        lua
    }

    #[test]
    fn can_do_simple_get() -> Result<()> {
        let lua = init(); 
        lua.load("SQLuaFetches()").exec()?;
        assert!(lua.globals().get::<String>("TestReturn")? == "Welcome File".to_string());
        Ok(())
    }
}
