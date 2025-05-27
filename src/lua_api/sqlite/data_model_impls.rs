use crate::db;
use crate::db::*;
use crate::lua_api::sqlite::SQLua;
use anyhow::Result;
use exemplar::Model;
use hypertext::html_elements::object;
use mlua::{
    chunk, ExternalResult, FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, Lua, LuaSerdeExt, MaybeSend, Result as luaResult, Table, UserData, Value
};
use proptest::strategy::W;
use rusqlite::{
    fallible_streaming_iterator::FallibleStreamingIterator,
    params, params_from_iter,
    types::{ToSqlOutput, Value as rValue, ValueRef},
    CachedStatement, Connection, Error, OptionalExtension, Params, ParamsFromIter,
    Result as rResult, Row, ToSql,
};
use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};


fn make_upsert_name(thestring: &str) -> String {
    let mut accum = "upsert".to_string();
    for ch in thestring.chars() {
        if ch.is_ascii_uppercase() {
            accum.push('_');
        } 
        accum.push(ch.to_ascii_lowercase());
    }
    return accum;
}

macro_rules! mut_method_upsert_record {
    ($methods:ident, $type:path) => {
        $methods.add_method_mut(make_upsert_name(stringify!($type)), |_, t, rec: $type| {
            let reccopy = rec.clone();
            t.0.send_messenger(move |conn| {
                reccopy.insert_or(conn, exemplar::OnConflict::Replace)?;
                Ok(reccopy)
            })?;
            return Ok(true);
        })
    };
    ($methods:ident, $($type:path),+) => {
        $(mut_method_upsert_record!($methods, $type);)+
    }
}
macro_rules! make_sql_lua_boilerplate {
    ($val:path) => {
        impl FromLua for $val {
            fn from_lua(value: Value, lua: &Lua) -> luaResult<Self> {
                if let Value::Table(_) = value {
                    Ok(lua.from_value::<Self>(value)?)
                } else {
                    panic!("Only a table can be converted to a {:?}", stringify!($val));
                }
            }
        }
        impl IntoLua for $val {
            fn into_lua(self, lua: &Lua) -> luaResult<Value> {
                lua.to_value(&self)
            }
        }
        impl $val {


        }
    };
    ($($val:path),+) => {
        $(make_sql_lua_boilerplate![$val];)+
    }
}

impl UserData for SQLua {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("query", SQLua::query);
        mut_method_upsert_record!(methods,
            MediaCategoryRecord,
            MediaTypeRecord,
            FileExtensionRecord,
            MediaTypeForFileExtensionsRecord,
            FileRecord,
            FileArtworkRecord,
            ObjectAttr,
            ObjectExtraFileRecord,
            ObjectRecord,
            ObjectInCollection,
            CollectionRecord,
            DeviceRecord,
            DeviceSyncListRecord
        );
    }
}

make_sql_lua_boilerplate![
    MediaCategoryRecord,
    MediaTypeRecord,
    FileExtensionRecord,
    MediaTypeForFileExtensionsRecord,
    FileRecord,
    FileArtworkRecord,
    ObjectAttr,
    ObjectExtraFileRecord,
    ObjectRecord,
    ObjectInCollection,
    PageOfObjectsInCollection,
    CollectionRecord,
    DeviceRecord,
    DeviceSyncListRecord
];

