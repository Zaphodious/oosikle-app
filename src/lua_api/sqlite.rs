use crate::db::*;
use crate::db;
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
use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub struct SQLua {
    conn: Option<Connection>,
    db_loc: PathBuf,
    init_script: String,
}

impl SQLua {
    pub fn add_to_lua(db_loc: PathBuf, init_script: &str, lua: &Lua) -> Result<()> {
        let this = SQLua {
            conn: None,
            db_loc,
            init_script: init_script.to_string(),
        };
        lua.globals().set("DB", this)?;
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
            }
            t
        })),
        ValueRef::Null => Ok(Value::NULL),
    }
}

fn attr_value_to_lua(lua: &Lua, value: AttrValue) -> luaResult<Value> {
        Ok(match value {
            AttrValue::NONE => Value::NULL,
            AttrValue::INT(i) => Value::Integer(i.try_into().unwrap()),
            AttrValue::FLOAT(f) => Value::Number(f.try_into().unwrap()),
            AttrValue::STRING(s) => Value::String(lua.create_string(s)?),
            AttrValue::BYTES(b) => Value::Table({
            let t = lua.create_table()?;
            for b in b {
                t.push(b)?;
            }
            t
        }),

        })
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

fn detect_reset_sqlite_connection(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare_cached("SELECT count(*) as count FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';")?;
    Ok(stmt.query_row([], |row| {
        let count: i32 = row.get("count")?;
        Ok(count == 0)
    })?)
}

impl SQLua {
    pub fn connect_to_db(_lua: &Lua, this: &mut SQLua, (): ()) -> luaResult<()> {
        match &this.conn {
            Some(conn) => {
                if detect_reset_sqlite_connection(&conn)? {
                    this.conn = None;
                    SQLua::connect_to_db(_lua, this, ())
                } else {
                    Ok(())
                }
            }
            None => {
                let conn = Connection::open(&this.db_loc).map_err(mlua::Error::external)?;
                conn.execute_batch(&this.init_script)
                    .map_err(mlua::Error::external)?;
                this.conn = Some(conn);
                Ok(())
            }
        }
    }

    pub fn get_ext_attributes_for_object(lua: &Lua, this: &mut SQLua, object_uuid: String) -> luaResult<Table> {
        SQLua::connect_to_db(lua, this, ())?;
        let t = lua.create_table()?;
        if let Some(conn) = &this.conn {
            let attrs = ObjectAttr::get_attributes_for_object_uuid(&conn, &object_uuid).map_err(mlua::Error::external)?;
            for attr in attrs {
                t.set(attr.name, attr_value_to_lua(lua, attr.data)?)?;
            }
        } else {
            panic!("Completely failed to secure a good DB connection");
        }
        Ok(t)
    }

    /*
    pub fn add_media_category(lua: &Lua, this: &mut SQLua, media_category_name: String) -> luaResult<Table> {

    }
     */

    pub fn query(
        lua: &Lua,
        this: &mut SQLua,
        (sqlstr, params): (String, Vec<ValueWrap>),
    ) -> luaResult<Table> {
        SQLua::connect_to_db(lua, this, ())?;
        if let Some(conn) = &this.conn {
            let nu_params = (&params)
                .into_iter()
                .map(|v| v.to_sql().expect("Falat error parsing lua"))
                .collect::<Vec<_>>();
            for param in &nu_params {
                println!("{:?}", param);
            }
            let mut stmt = conn.prepare_cached(&sqlstr).into_lua_err()?;
            let headers = (stmt)
                .column_names()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            /*
            for header in &headers {
                println!("header: {}", header);
            } */
            let mut query_result = (stmt).query(params_from_iter(params)).map_err(mlua::Error::external)?;
            //let mut ret: Vec<Table> = vec![];
            let mut ret = lua.create_table()?;
            while let Some(row) = query_result.next().map_err(mlua::Error::external)? {
                // println!("thingamabob is {:?}", row);
                let t = lua.create_table()?;
                for head in &headers {
                    t.set(
                        head.as_str(),
                        value_to_lua(lua, row.get_ref_unwrap(head.as_str())).unwrap(),
                    )
                    .unwrap();
                    // println!("The header is {}", head);
                }
                ret.push(t)?;
            }

            Ok(ret)
        } else {
            panic!("Completely failed to secure a good DB connection");
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
mod basic_functionality_tests {
    use super::*;
    use crate::{db, lua_api};
    use rusqlite::Connection;

    static TESTING_VALUES: &'static str = include_str!("../testing_data/sql/testing_values.sql");
    static INIT_DB_STR: &'static str = include_str!("../db/init_db.sql");

    #[test]
    fn plain_sql_works() -> Result<(), Error> {
        let conn = db::init_db(":memory:")?;
        conn.execute(TESTING_VALUES, []).unwrap();
        let the_query =
            "select * from Objects where Objects.object_uuid='DEADBEEFDEADBEEFDEADBEEFDEADBEEF';";
        let mut stmt = conn.prepare_cached(the_query)?;
        let mut res1 = stmt.query([])?;
        res1.next().unwrap();
        return Ok(());
    }

    #[test]
    fn reset_connection_checker_checks_correctly() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        assert!(detect_reset_sqlite_connection(&conn)?);
        conn.execute_batch(INIT_DB_STR)?;
        assert!(!detect_reset_sqlite_connection(&conn)?);
        Ok(())
    }
}
#[cfg(test)]
mod read_from_lua_tests {
    use super::*;
    use crate::{db, lua_api};
    use rusqlite::Connection;

    static TESTING_VALUES: &'static str = include_str!("../testing_data/sql/testing_values.sql");
    static INIT_DB_STR: &'static str = include_str!("../db/init_db.sql");
    static TESTING_LUA: &'static str = include_str!("../testing_data/lua/sqlua_testing.luau");

    fn init() -> Lua {
        let mut lua = lua_api::init(None).expect("Lua failed to initialize");
        lua.load(TESTING_LUA).exec().expect("Lua failed to load the testing script");

        SQLua::add_to_lua(
            ":memory:".into(),
            &(INIT_DB_STR.to_string() + TESTING_VALUES),
            &lua,
        ).expect("SQLua failed to properly initialize");
        lua
    }


    #[test]
    fn can_do_simple_get() -> Result<()> {
        let mut lua = init();
        let res = lua.load("SQLuaFetches()").eval::<String>()?;
        assert!(res == "Welcome File");
        //assert!(lua.globals().get::<String>("TestReturn")? == "Welcome File".to_string());
        Ok(())
    }

    #[test]
    fn can_make_html_for_list() -> Result<()> {
        let mut lua = init();
        let res = lua.load("SQLuaCreatesHTMLBasic([[BADC0FFEE0DDF00DBADC0FFEE0DDF00D]])").eval::<String>()?;
        assert!(res.contains("<tr draggable=\"true\"> <td>Welcome File</td> <td>TheHotFish</td> <td></td> <td>1970-01-01</td> </tr>"));
        //print!("{:?}", res);
        Ok(()) 
    }

}
