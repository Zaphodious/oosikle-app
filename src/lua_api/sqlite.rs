use crate::db::*;
use anyhow::Result;
use exemplar::Model;
use hypertext::html_elements::object;
use mlua::{
    chunk, ExternalResult, FromLua, IntoLua, Lua, Result as luaResult, Table, UserData, Value,
};
use rusqlite::{
    fallible_streaming_iterator::FallibleStreamingIterator,
    params, params_from_iter,
    types::{ToSqlOutput, Value as rValue, ValueRef},
    CachedStatement, Connection, Error, OptionalExtension, Params, ParamsFromIter,
    Result as rResult, Row, ToSql,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
pub struct SQLua {
    conn: Option<Connection>,
    db_loc: String,
    init_script: String
}

impl SQLua {
    pub fn add_to_lua(db_loc: &str, init_script: &str, lua: &Lua) -> Result<()> {
        let this = SQLua {
            conn: None,
            db_loc: db_loc.to_string(),
            init_script: init_script.to_string()
        };
        lua.globals().set("SQL", this)?;
        lua.load("SQL.__connect_to_db()").exec();
        Ok(())
    }
}

fn value_to_lua(lua: &Lua, value: ValueRef) -> luaResult<Value> {
    match value {
        ValueRef::Text(s) => Ok(Value::String(lua.create_string(s)?)),
        ValueRef::Integer(s) => Ok(Value::Integer(s.try_into().unwrap())),
        ValueRef::Real(s) => Ok(Value::Number(s)),
        ValueRef::Blob(s) => Ok(Value::Table({
            let t = lua.create_table()?;
            for b in s {
                t.push(*b)?;
            };
            t
        })),
        ValueRef::Null => Ok(Value::NULL),
    }
}

#[derive(Debug)]
pub struct ValueWrap(Value);

impl ToSql for ValueWrap {
    fn to_sql(&self) -> rResult<rusqlite::types::ToSqlOutput<'_>> {
        match &self.0 {
            Value::Boolean(s) => Ok(ToSqlOutput::Owned(rValue::Integer(if *s { 1 } else { 0 }))),
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

    pub fn connect_to_db(_lua: &Lua, this: &mut SQLua, (): ()) -> luaResult<()> {
        let conn = Connection::open(&this.db_loc).map_err(mlua::Error::external)?;
        conn.execute_batch(&this.init_script).map_err(mlua::Error::external)?;
        this.conn = Some(conn);
        Ok(())
    }

    pub fn query(lua: &Lua, this: &mut SQLua, (sqlstr, params): (String, Vec<ValueWrap>)) -> luaResult<Table> {
        if let Some(conn) = &this.conn {
        let nu_params = (&params)
            .into_iter()
            .map(|v| v.to_sql().expect("Falat error parsing lua"))
            .collect::<Vec<_>>();
        for param in &nu_params {
            println!("{:?}", param);
        }
        let mut stmt = conn
            .prepare_cached(&sqlstr)
            .into_lua_err()?;
        let headers = (stmt)
            .column_names()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for header in &headers {
            println!("header: {}", header);
        }
        let mut query_result = (stmt).query([]).map_err(mlua::Error::external)?;
        //let mut ret: Vec<Table> = vec![];
        let mut ret = lua.create_table()?;
        while let Some(row) = query_result.next().map_err(mlua::Error::external)? {
            println!("thingamabob is {:?}", row);
            let t = lua
                .create_table()?;
            for head in &headers {
                t.set(
                    head.as_str(),
                    value_to_lua(lua, row.get_ref_unwrap(head.as_str())).unwrap(),
                )
                .unwrap();
                println!("The header is {}", head);
            }
            ret.push(t)?;
        }

        println!("Does anything fucking work in this fucking shithole?");

        Ok(ret)
        } else {
            SQLua::connect_to_db(lua, this, ())?;
            SQLua::query(lua, this, (sqlstr, params))
        }

    }
}

impl UserData for SQLua {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("query", SQLua::query);
        methods.add_method_mut("__connect_to_db", SQLua::connect_to_db);
    }
}

impl UserData for CollectionRecord {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{db, lua_api};
    use rusqlite::Connection;

    static TESTING_VALUES: &'static str = include_str!("./many_testing_values.sql");
    static TESTING_LUA: &'static str = include_str!("./sqlua_testing.lua");

    fn init() -> Lua {
        let conn = db::init_db("./testingdb.sqlite3").unwrap();
        //conn.execute(TESTING_VALUES, []).unwrap();

        let mut lua = lua_api::init(None).unwrap();
        let _ = lua.load(TESTING_LUA).exec();

        let _ = SQLua::add_to_lua(":memory:", &(include_str!("./init_db.sql").to_string() + &TESTING_VALUES.to_string()), &lua);
        //SQLua::init(conn).unwrap().inject_into(&lua).unwrap();
        lua
    }

    #[test]
    fn plain_sql_works() -> Result<(), Error> {
        let conn = db::init_db("./testingdb.sqlite3")?;
        // conn.execute(TESTING_VALUES, []).unwrap();
        let the_query =
            "select * from Objects where Objects.object_uuid='DEADBEEFDEADBEEFDEADBEEFDEADBEEF';";
        let mut stmt = conn.prepare_cached(the_query)?;
        let mut res1 = stmt.query([])?;
        res1.next().unwrap();
        return Ok(());
    }

    #[test]
    fn can_do_simple_get() -> Result<()> {
        let mut lua = init();
        let res = lua.load("SQLuaFetches()").eval::<String>()?;
        assert!(res == "Welcome File");
        //assert!(lua.globals().get::<String>("TestReturn")? == "Welcome File".to_string());
        Ok(())
    }
}
