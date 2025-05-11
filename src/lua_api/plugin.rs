use crate::db::*;
use anyhow::Result;
use exemplar::Model;
use hypertext::html_elements::object;
use mlua::{
    chunk, ExternalResult, FromLua, IntoLua, Lua, Result as luaResult, Table, UserData, Value,
};
use rust_search::{FilterExt, SearchBuilder};
use std::{
    fs::canonicalize,
    io,
    path::{Path, PathBuf},
};

use super::{init as lua_init, sqlite::SQLua};

#[derive(Debug, Clone)]
struct LuaPluginRegistrar {
    plugin_credit: Option<Table>,
    media_categories: Vec<Table>,
    media_types: Vec<Table>,
    file_extensions: Vec<Table>,
    view_adapters: Vec<Table>,
    package_name: String,
}

impl LuaPluginRegistrar {
    pub fn new(package_name: String) -> Result<LuaPluginRegistrar> {
        Ok(LuaPluginRegistrar {
            plugin_credit: None,
            media_categories: vec![],
            media_types: vec![],
            file_extensions: vec![],
            view_adapters: vec![],
            package_name,
        })
    }
    pub fn define_plugin_credit(
        &mut self,
        credit_table: Table,
    ) -> luaResult<()> {
        self.plugin_credit = Some(credit_table);
        Ok(())
    }
    pub fn define_media_category(&mut self, addition: Table) -> luaResult<()> {
        self.media_categories.push(addition);
        Ok(())
    }
    pub fn define_media_type(&mut self, addition: Table) -> luaResult<()> {
        self.media_types.push(addition);
        Ok(())
    }
}

impl UserData for LuaPluginRegistrar {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("Credit", |_, s, t: Table| s.define_plugin_credit(t));
        methods.add_method_mut("DefineMediaCategory", |_, s, t: Table| s.define_media_category(t));
        methods.add_method_mut("DefineMediaType", |_, s, t: Table| s.define_media_type(t));
    }
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

    fn parse(&self, lua: Lua) -> Result<LuaPluginRegistrar> {
        lua.load(&self.script_contents).exec()?;
        let registrar = LuaPluginRegistrar::new(self.full_name())?;
        Ok(registrar)
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
    use std::collections::HashSet;

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
}
