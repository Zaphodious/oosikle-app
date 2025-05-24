use crate::db::*;
use anyhow::Result;
use exemplar::Model;
use hypertext::html_elements::object;
use mlua::{
    chunk, AnyUserData, Error, ExternalResult, FromLua, Function, IntoLua, Lua,
    Result as luaResult, Table, UserData, Value,
};
use rusqlite::Connection;
use rust_search::{FilterExt, SearchBuilder};
use std::{
    fs::canonicalize,
    io,
    path::{Path, PathBuf},
};
use time::{macros::format_description, Date};

use super::{init as lua_init, sqlite::SQLua};

#[derive(Debug, Clone, PartialEq, FromLua)]
pub struct LuaObjectAdapter {
    media_type: String,
    custom_detail_view: Function,
    play_action: Function,
    initialize_object: Function,
    settings: Table,
}

/*
       {
           media_type = "pico8",
           custom_detail_view = function(object_uuid, settings) end,
           play_action = function(object_uuid, settings)
               return { action = "run", exe = "path_from_settings", args = "run=path_to_p8_file" }
           end,
           initialize_object = function(file_table, settings) end,
           settings = {
               pico8path = { type = "filepath", default = nil }
           },
       },
*/

#[derive(Debug, Clone, PartialEq)]
pub enum ViewAdapterKind {
    MediaCategory(String),
    MediaType(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LuaViewAdapter {
    adapter_kind: ViewAdapterKind,
    page_sql: String,
    columns: Table,
    settings: Table,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LuaPluginParsedDefintions {
    file_extensions: Vec<FileExtensionRecord>,
    types: Vec<MediaTypeRecord>,
    categories: Vec<MediaCategoryRecord>,
    types_for_file_extensions: Vec<MediaTypeForFileExtensionsRecord>,
}

impl FromLua for LuaPluginParsedDefintions {
    fn from_lua(value: Value, lua: &Lua) -> luaResult<Self> {
        let the_table = value.as_table().expect("Value should be a table");
        Ok(Self {
            categories: the_table.get("categories")?,
            types: the_table.get("types")?,
            file_extensions: the_table.get("file_extensions")?,
            types_for_file_extensions: the_table.get("types_for_file_extensions")?,
        })
    }
}

impl LuaPluginParsedDefintions {
    fn insert_definitions(&self, conn: &Connection) -> Result<()> {
        for n in &self.categories {
            n.insert(conn)?;
        }
        for n in &self.types {
            n.insert(conn)?;
        }
        for n in &self.file_extensions {
            n.insert(conn)?;
        }
        for n in &self.types_for_file_extensions {
            n.insert(conn)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LuaPluginParseResult {
    pub namespace: String,
    pub authors: Vec<String>,
    pub version: u32,
    pub date: String,
    pub definitions: Option<LuaPluginParsedDefintions>,
    pub view_adapters: Option<Vec<Table>>,
    pub object_adapters: Option<Vec<Table>>,
}

impl FromLua for LuaPluginParseResult {
    fn from_lua(value: Value, lua: &Lua) -> luaResult<Self> {
        let format = format_description!("[year]-[month]-[day]");
        let the_table = value.as_table().expect("Value should be a table");
        Ok(Self {
            date: Date::parse(
                &the_table
                    .get::<String>("date")
                    .expect("Date string needs to be properly formatted"),
                format,
            )
            .into_lua_err()?
            .to_string(),
            namespace: the_table.get("namespace")?,
            authors: the_table.get("authors")?,
            version: the_table.get("version")?,

            definitions: the_table.get("definitions")?,

            view_adapters: the_table.get("view_adapters")?,
            object_adapters: the_table.get("object_adapters")?,
        })
    }
}

#[derive(Debug, Clone)]
struct LuaPluginRegistrar {
    plugin_credit: Option<Table>,
    media_categories: Vec<Table>,
    media_types: Vec<Table>,
    file_extensions: Vec<Table>,
    view_adapters: Vec<Table>,
    object_adapters: Vec<Table>,
    extensions: Vec<Table>,
    package_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UnparsedLuaPlugin {
    name: String,
    namespace: String,
    entry_point: PathBuf,
    script_contents: String,
}

impl UnparsedLuaPlugin {
    fn new(entry_point: PathBuf, plugin_root: &Path) -> Self {
        let fqpn = canonicalize(&entry_point)
            .expect("SearchBuilder returned invalid path")
            .strip_prefix(&plugin_root)
            .expect("{entry_point} is not a child of {plugin_root}")
            .iter()
            .map(|s| s.to_str().unwrap().to_owned())
            .reduce(|acc, s| format!("{acc}.{s}"))
            .expect("Empty canon path");

        let mut spliterator = fqpn.rsplitn(4, '.').skip(2); // Skip lua(u) and plugin

        let name = spliterator
            .next()
            .expect("Failed to get plugin name")
            .to_owned();
        let namespace = spliterator.next().unwrap_or("").to_owned();

        let script_contents =
            std::fs::read_to_string(&entry_point).expect("There was a problem reading the file");

        UnparsedLuaPlugin {
            name,
            namespace,
            entry_point,
            script_contents,
        }
    }

    fn parse(&self, lua: &Lua) -> Result<LuaPluginParseResult> {
        const PLUGIN_PRELOAD_FN: &str = include_str!("./plugin_dec_pre_load.lua");
        let plugin_wrap_fn = lua.load(PLUGIN_PRELOAD_FN).eval::<Function>()?;
        let plugin_fn = lua.load(self.script_contents()).eval::<Function>()?;
        let thingy = plugin_wrap_fn.call::<LuaPluginParseResult>(plugin_fn)?;
        Ok(thingy)
    }
}

impl UnparsedLuaPlugin {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn full_name(&self) -> String {
        if self.namespace.is_empty() {
            self.name.clone()
        } else {
            format!("{}.{}", self.namespace, self.name)
        }
    }

    fn entry_point(&self) -> &Path {
        &self.entry_point
    }

    pub fn script_contents(&self) -> &str {
        &self.script_contents
    }
}

fn discover_plugins(plugin_root: &str) -> Result<Vec<UnparsedLuaPlugin>> {
    let plugin_root = canonicalize(plugin_root)?;
    Ok(SearchBuilder::default()
        .location(&plugin_root)
        .search_input(r#"^([\w\-]+\.)*plugin"#)
        .ext("luau?")
        .ignore_case()
        .custom_filter(|entry| {
            entry.path().is_dir()
                || (entry.depth() == 1) ^ (entry.file_name().eq_ignore_ascii_case("plugin.lua"))
        })
        .build()
        .map(|entry_point| UnparsedLuaPlugin::new(entry_point.into(), &plugin_root))
        .collect())
}

#[cfg(test)]
mod plugin_resoltuion_tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    const PLUGIN_DIR: &str = "src/testing_data/lua/plugins";

    #[test]
    fn plugin_finder_doesnt_error() -> Result<()> {
        // let res = find_plugin_lua_files("testplugins")?;
        let _res = discover_plugins(PLUGIN_DIR);
        // assert!(false);
        Ok(())
    }

    #[test]
    fn plugin_finder_finds_what_it_should() -> Result<()> {
        let res = discover_plugins(PLUGIN_DIR)?;
        let names = res
            .into_iter()
            .map(|p| (&p).name().to_string())
            .collect::<HashSet<_>>();
        println!("{:?}", names);
        assert!(names.contains("basic"));
        assert!(names.contains("test"));
        assert!(names.contains("bang"));
        assert!(names.contains("foo"));
        assert!(names.contains("bar"));
        Ok(())
    }

    #[test]
    fn plugin_finder_doesnt_find_what_it_shouldnt() -> Result<()> {
        let res = discover_plugins(PLUGIN_DIR)?;
        let names = res
            .into_iter()
            .map(|p| (&p).name().to_string())
            .collect::<HashSet<_>>();
        println!("{:?}", names);
        assert!(!names.contains("nota"));
        assert!(!names.contains("stillnota"));
        Ok(())
    }

    /*
    #[test]
    fn unparsed_plugin_parses_without_error() -> Result<()> {
        let plugin = UnparsedLuaPlugin {
            name: "testing".into(),
            namespace: "testing.testing".into(),
            entry_point: "".into(),
            script_contents: "".into(),
        };
        let lua = Lua::new();
        plugin.parse(&lua)?;
        Ok(())
    } */

    fn grab_testing_plugin_unparsed() -> Result<UnparsedLuaPlugin> {
        Ok(discover_plugins(PLUGIN_DIR)?
            .into_iter()
            .filter(|p| p.full_name() == "test")
            .nth(0)
            .expect("Testing plugin not found"))
    }

    #[test]
    fn plugin_finder_generates_unparsed_plugin_correctly() -> Result<()> {
        let plugin = grab_testing_plugin_unparsed()?;
        assert!(plugin.script_contents.contains("Plugin:Credit({"));
        Ok(())
    }

    fn grab_videogame_basic_unparsed() -> Result<UnparsedLuaPlugin> {
        Ok(discover_plugins(PLUGIN_DIR)?
            .into_iter()
            .filter(|p| p.full_name() == "videogame_basic")
            .nth(0)
            .expect("Testing plugin not found"))
    }

    #[test]
    fn plugin_parser_does_the_thing() -> Result<()> {
        let plugin = grab_videogame_basic_unparsed()?;
        let lua = Lua::new();
        let res = plugin.parse(&lua)?;
        assert!(res.namespace == "oosikle.builtin.pico8");
        assert!(res.version == 1);
        Ok(())
    }
    /*
    #[test]
    fn unparsed_plugin_correctly_parses() -> Result<()> {
        let unparsed_plugin = grab_testing_plugin_unparsed()?;
        let lua = Lua::new();
        let registrar = unparsed_plugin.parse(&lua)?;
        assert!(registrar.package_name == unparsed_plugin.full_name());
        assert!(registrar.plugin_credit.expect("Credit table not found").get::<Table>("authors")?.get::<String>(1)? == "HotFish");
        Ok(())
    } */
}
