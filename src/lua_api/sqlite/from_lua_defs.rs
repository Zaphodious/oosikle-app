use crate::db;
use crate::db::*;
use anyhow::Result;
use exemplar::Model;
use hypertext::html_elements::object;
use mlua::{
    chunk, ExternalResult, FromLua, IntoLua, Lua, Result as luaResult, Table, UserData, Value,
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

impl FromLua for db::MediaCategoryRecord {
    fn from_lua(value: Value, lua: &Lua) -> luaResult<Self> {
        if let Value::Table(t) = value {
            Ok(MediaCategoryRecord {
                id: t.get::<String>("media_category_id")?,
                string_key: t.get::<String>("media_category_string_key")?,
            })
        } else {
            panic!("we only support tables at this time")
        }
    }
}

impl FromLua for db::MediaTypeRecord {
    fn from_lua(value: Value, lua: &Lua) -> luaResult<Self> {
        if let Value::Table(t) = value {
            Ok(MediaTypeRecord {
                id: t.get::<String>("media_type_id")?,
                string_key: t.get::<String>("media_type_string_key")?,
                media_category_id: t.get::<String>("media_category_key")?,
            })
        } else {
            panic!("we only support tables at this time")
        }
    }
}

impl FromLua for db::FileExtensionRecord {
    fn from_lua(value: Value, lua: &Lua) -> luaResult<Self> {
        if let Value::Table(t) = value {
            Ok(FileExtensionRecord {
                tag: t.get::<String>("file_extension_tag")?,
                string_key: t.get::<String>("file_extension_desc_string_key")?,
            })
        } else {
            panic!("we only support tables at this time")
        }
    }
}
