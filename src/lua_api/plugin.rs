use crate::db::*;
use anyhow::Result;
use exemplar::Model;
use hypertext::html_elements::object;
use mlua::{
    chunk, ExternalResult, FromLua, IntoLua, Lua, Result as luaResult, Table, UserData, Value,
};
use std::{fs::canonicalize, io, path::{Path, PathBuf}};
use rust_search::{FilterExt, SearchBuilder};

#[derive(Debug)]
struct LuaPluginUnparsed {
    package: String,
    path: String,
    source: String
}

fn find_plugin_lua_files (plugin_root_dir: &str) -> Result<Vec<LuaPluginUnparsed>> {
   let search :Vec<String> = SearchBuilder::default()
    .location(plugin_root_dir)
    .search_input("/*.plugin.(lua|luau)").depth(8).ignore_case().build().collect();
    let results = search.into_iter().map(|path| {
        let package = path.split(&['.', '\\', '/']).skip(1).collect::<Vec<&str>>()
        .into_iter().rev().skip(2).rev()
            .fold(None, |s1: Option<String>, s2| {
                Some(match s1 {
                    Some(s) => format!("{}.{}", s, s2),
                    None => s2.to_string()
                })
            }).expect("If there is no valid plugin package name we are sunk");
        LuaPluginUnparsed { path, package, source: "".to_string() }}).collect::<Vec<_>>();
    print!("results of searching: {:?}", results);

    assert!(false);
    Ok(vec![])
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UninitializedLuaPlugin {
    name: String,
    namespace: String,
    entry_point: PathBuf,
    script_contents: String
}

impl UninitializedLuaPlugin {
    fn new(entry_point: PathBuf, plugin_root: &Path) -> Self {
        let fqpn = canonicalize(&entry_point)
            .expect("SearchBuilder returned invalid path")
            .strip_prefix(&plugin_root)
            .expect("{entry_point} is not a child of {plugin_root}")
            .iter()
            .map(|s| s.to_str().unwrap().to_owned())
            .reduce(|acc, s| format!("{acc}.{s}"))
            .expect("Empty canon path");

        let mut spliterator = fqpn.rsplitn(4, '.')
            .skip(2); // Skip lua(u) and plugin

        let name = spliterator
            .next()
            .expect("Failed to get plugin name")
            .to_owned();
        let namespace = spliterator
            .next()
            .unwrap_or("")
            .to_owned();


        let script_contents = std::fs::read_to_string(&entry_point).expect("There was a problem reading the file");

        UninitializedLuaPlugin {
            name,
            namespace,
            entry_point,
            script_contents
        }
    }

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

fn discover_plugins(plugin_root: &str) -> io::Result<Vec<UninitializedLuaPlugin>> {
    let plugin_root = canonicalize(plugin_root)?;
    Ok(SearchBuilder::default()
        .location(&plugin_root)
        .search_input(r#"^([\w\-]+\.)*plugin"#)
        .ext("luau?")
        .ignore_case()
        .custom_filter(|entry| 
            entry.path().is_dir() || (entry.depth() == 1) ^ (entry.file_name().eq_ignore_ascii_case("plugin.lua"))
        )
        .build()
        .map(|entry_point| UninitializedLuaPlugin::new(entry_point.into(), &plugin_root))
        .collect()
    )
}

#[cfg(test)]
mod plugin_resoltuion_tests {
    use super::*;

    #[test]
    fn finds_plugins_by_plugin_dot_lua() -> Result<()> {
        // let res = find_plugin_lua_files("testplugins")?;
        let res = discover_plugins("src/testing_data/lua/plugins");
        print!("{:?}", res);
        // assert!(false);
        Ok(())
    }
}
