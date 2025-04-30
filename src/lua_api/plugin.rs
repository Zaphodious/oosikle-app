use crate::db::*;
use anyhow::Result;
use exemplar::Model;
use hypertext::html_elements::object;
use mlua::{
    chunk, ExternalResult, FromLua, IntoLua, Lua, Result as luaResult, Table, UserData, Value,
};
use rust_search::SearchBuilder;

struct LuaPluginUnparsed {
    package: String,
    source: String
}

fn find_plugins_lua_files (plugin_root_dir: &str) -> Result<Vec<LuaPluginUnparsed>> {
   let search :Vec<String> = SearchBuilder::default()
    .location(plugin_root_dir)
    .search_input("plugin.").ext("lua|luau").depth(8).ignore_case().build().collect();
    print!("{:?}", search);

    assert!(false);
    Ok(vec![])
}

#[cfg(test)]
mod plugin_resoltuion_tests {
    use super::*;

    #[test]
    fn finds_plugins_by_plugin_dot_lua() -> Result<()> {
        let res = find_plugins_lua_files("plugins")?;
        Ok(())
    }
}
