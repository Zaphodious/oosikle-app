use crate::db::*;
use crate::miko;
use crate::{db, miko::Miko};
use anyhow::Result;
use exemplar::Model;
use hypertext::html_elements::object;
use mlua::{
    chunk, ExternalResult, FromLua, IntoLua, Lua, LuaSerdeExt, Result as luaResult, Table,
    UserData, Value,
};
use rusqlite::{
    fallible_streaming_iterator::FallibleStreamingIterator,
    params, params_from_iter,
    types::{FromSql, ToSqlOutput, Value as rValue, ValueRef},
    CachedStatement, Connection, Error, OptionalExtension, Params, ParamsFromIter,
    Result as rResult, Row, ToSql,
};
use serde::{Deserialize, Serialize};
use std::{fmt, iter::zip, path::PathBuf};

mod data_model_impls;

#[derive(Debug)]
pub struct SQLua(Miko<Connection>);

impl Miko<Connection> {
    pub fn construct_connection_shrine(
        db_loc: PathBuf,
        init_script: &str,
    ) -> Result<(Miko<Connection>, miko::ShrineDestroyer)> {
        let string_script = init_script.to_string();
        Ok(Miko::build_shrine("sqlite_prime", move || {
            let conn = Connection::open(db_loc).map_err(mlua::Error::external)?;
            &conn
                .execute_batch(&string_script)
                .map_err(mlua::Error::external)?;
            Ok(conn)
        })?)
    }
}

impl SQLua {
    pub fn add_to_lua(sql_miko: Miko<Connection>, lua: &Lua) -> Result<()> {
        let this = SQLua(sql_miko);
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

pub enum LiberatedColumn {
    Bool(bool),
    String(String),
    Int(i64),
    Number(f64),
    Blob(Vec<u8>),
    Null,
}

impl FromSql for LiberatedColumn {
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Ok(match value {
            ValueRef::Integer(t) => LiberatedColumn::Int(t),
            ValueRef::Real(t) => LiberatedColumn::Number(t),
            ValueRef::Text(t) => {
                if let Ok(s) = String::from_utf8(t.to_vec()) {
                    LiberatedColumn::String(s)
                } else {
                    LiberatedColumn::Blob(t.to_vec())
                }
            }
            ValueRef::Blob(t) => LiberatedColumn::Blob(t.to_vec()),
            ValueRef::Null => LiberatedColumn::Null,
        })
    }
}

impl IntoLua for LiberatedColumn {
    fn into_lua(self, lua: &Lua) -> luaResult<Value> {
        Ok(match self {
            LiberatedColumn::Bool(t) => t.into_lua(lua)?,
            LiberatedColumn::String(t) => t.into_lua(lua)?,
            LiberatedColumn::Int(t) => t.into_lua(lua)?,
            LiberatedColumn::Number(t) => t.into_lua(lua)?,
            LiberatedColumn::Blob(t) => t.into_lua(lua)?,
            LiberatedColumn::Null => Value::NULL,
        })
    }
}

#[derive(Debug, Clone)]
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
    /*
    pub fn get_ext_attributes_for_object(
        lua: &Lua,
        this: &mut SQLua,
        object_uuid: String,
    ) -> luaResult<Table> {
        let t = lua.create_table()?;
        if let Some(conn) = &this.conn {
            let attrs = ObjectAttr::get_attributes_for_object_uuid(&conn, &object_uuid)
                .map_err(mlua::Error::external)?;
            for attr in attrs {
                t.set(
                    attr.attribute_name,
                    attr_value_to_lua(lua, attr.attribute_value)?,
                )?;
            }
        } else {
            panic!("Completely failed to secure a good DB connection");
        }
        Ok(t)
    }
    */

    pub fn query(
        lua: &Lua,
        this: &mut SQLua,
        (sqlstr, params): (String, Vec<ValueWrap>),
    ) -> luaResult<Table> {
        println!("Going to send the messenger");
        /*
        let nu_params = (params)
            .into_iter()
            .map(|v| v.to_sql().expect("Falat error parsing lua"))
            .collect::<Vec<rValue>>();
        */
        let (headers, rows) = this.0.send_messenger(move |conn| {
            let mut p1: Vec<&dyn ToSql> = vec![];
            for n in &params {
                p1.push(n);
            }
            let pp: &[&dyn ToSql] = p1.as_slice(); 
            let mut stmt = conn.prepare_cached(&sqlstr)?;
            println!("getting headers");
            let headers: Vec<String> = (stmt)
                .column_names().iter().map(|s| s.to_string()).collect();
            println!("headers should be {:?}", headers);
            /*
            let params = params_from_iter(param_clone.into_iter());
            println!("params are {:?}", &params);
            */
            println!("got headers, running query");
            let mut query_result = match (stmt).query(pp) {
                Ok(s) => s,
                Err(e) => {
                    println!("There has been an error running the query: {:?}", e);
                    panic!();
                }
            };
            println!("query ran, shoving results into a vector");
            let mut ret: Vec<Vec<LiberatedColumn>> = Vec::new();
            while let Some(row) = query_result.next()? {
                let mut retrow: Vec<LiberatedColumn> = Vec::new();
                for header in &headers {
                    retrow.push(row.get::<&str, LiberatedColumn>(header.as_str())?);
                }
                ret.push(retrow);
            }
            println!("about to return from query messenger");
            Ok((headers, ret))
        })?;
        /*
        for header in &headers {
            println!("header: {}", header);
        } */
        //let mut ret: Vec<Table> = vec![];
        println!("Making the return lua table");
        let ret = lua.create_table()?;
        println!("Going to iterate over ret values");
        for rowvec in rows {
            let t = lua.create_table()?;
            let ziprow = zip(headers.clone(), rowvec);
            for (header, val) in ziprow {
                println!("Iterated over a head thing: {:?}", header);
                t.set(header, val)?;
            }
            let _ = &ret.push(t)?;
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod basic_functionality_tests {
    use super::*;
    use crate::{db, lua_api};
    use rusqlite::Connection;

    static TESTING_VALUES: &'static str = include_str!("../../testing_data/sql/testing_values.sql");
    static INIT_DB_STR: &'static str = include_str!("../../db/init_db.sql");

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
    use miko::ShrineDestroyer;
    use mlua::LuaSerdeExt;
    use rusqlite::Connection;

    static TESTING_VALUES: &'static str = include_str!("../../testing_data/sql/testing_values.sql");
    static INIT_DB_STR: &'static str = include_str!("../../db/init_db.sql");
    static TESTING_LUA: &'static str = include_str!("../../testing_data/lua/sqlua_testing.luau");


    fn init() -> Result<(Lua, ShrineDestroyer)> {
        let lua = lua_api::init(None).expect("Lua failed to initialize");
        lua.load(TESTING_LUA)
            .exec()
            .expect("Lua failed to load the testing script");

        let (miko, destroyer): (Miko<Connection>, ShrineDestroyer) =
            Miko::construct_connection_shrine(
                ":memory:".into(),
                &(INIT_DB_STR.to_string() + TESTING_VALUES),
            )?;

        SQLua::add_to_lua(miko, &lua)?;
        Ok((lua, destroyer))
    }

    #[test]
    fn can_do_simple_get() -> Result<()> {
        let (lua, des) = init()?;
        let res = lua.load("SQLuaFetches()").eval::<String>()?;
        assert!(res == "Welcome File");
        //assert!(lua.globals().get::<String>("TestReturn")? == "Welcome File".to_string());
        Ok(())
    }

    #[test]
    fn can_make_html_for_list() -> Result<()> {
        let (lua, des) = init()?;
        let res = lua
            .load("SQLuaCreatesHTMLBasic([[BADC0FFEE0DDF00DBADC0FFEE0DDF00D]])")
            .eval::<String>()?;
        assert!(res.contains("<tr draggable=\"true\"> <td>Welcome File</td> <td>TheHotFish</td> <td></td> <td>1970-01-01</td> </tr>"));
        //print!("{:?}", res);
        des.invoke();
        println!("can make html test should be ending");
        Ok(())
    }

    #[test]
    fn from_lua_serde_works() -> Result<()> {
        let (lua, des) = init()?;
        let res = lua
            .load("SerdeWorksAsExpected([[VIDEOGAME]])")
            .eval::<MediaCategoryRecord>()?;
        let test_mc = MediaCategoryRecord {
            media_category_id: "VIDEOGAME".into(),
            media_category_string_key: "media_category_videogame".into(),
        };
        assert!(res == test_mc);
        des.invoke();
        println!("from lua serde test should be ending");
        Ok(())
    }

    #[test]
    fn into_lua_serde_works() -> Result<()> {
        let (lua, des) = init()?;
        let res = MediaCategoryRecord {
            media_category_id: "foo".into(),
            media_category_string_key: "oosike.foo".into(),
        };
        let res_conv = res.into_lua(&lua)?;
        if let Value::Table(t) = res_conv {
            assert!(t.get::<String>("media_category_id")? == "foo")
        } else {
            assert!(false);
        }
        des.invoke();
        println!("into lua serde test should be ending");
        Ok(())
    }

    /*
    #[test]
    fn can_insert_media_categories() -> Result<()> {
        let (lua, des) = init()?;
        let res = lua
            .load("SQLuaAddsMediaCategory([[foob]], [[foob_key]])")
            .eval::<String>()?;
        println!("what is the key? {:?}", res);
        assert!(res == "foob_key");
        des.invoke();
        println!("can insert media categories test should be ending");
        Ok(())
    }
    */

}
