use std::path::PathBuf;

use mlua::prelude::*;
use mlua::{
    StdLib,
    Table,
};

mod sqlite;



pub fn demotest() -> LuaResult<()> {
    let lua = Lua::new();

    let map_table = lua.create_table()?;
    map_table.set(1, "one")?;
    map_table.set("two", 2)?;

    lua.globals().set("map_table", map_table)?;

    lua.load("for k,v in pairs(map_table) do print(k,v) end").exec()?;

    Ok(())
}

#[tokio::main]
pub async fn mt_test() -> LuaResult<()> {
    const BASIC_TESTING_SCRIPT: &'static str = include_str!("./basic_testing.luau");

    let lua = init(None)?;
    lua.load(BASIC_TESTING_SCRIPT).exec()?;

    let mut threads = Vec::new();

    for _ in 1..10 {
        let func: LuaFunction = lua.globals().get("basic_function")?;

        let thread = tokio::spawn(async move {
            let result: usize = func.call(()).unwrap();
            println!("Lua result: {result}");
        });
        threads.push(thread);
    }

    for thread in threads {
        let _ = thread.await;
    }
    
    Ok(())
}

pub fn init(search_path: Option<PathBuf>) -> LuaResult<Lua> {
    let lua = Lua::new_with(
        StdLib::ALL,
        LuaOptions::default()
    )?;

    let search_path = search_path.unwrap_or_else(|| {
        let mut path = std::env::current_dir().unwrap();
        path.push("plugins");

        path
    });

    let search_path = format!("{0}/?.luau;{0}/?.lua;{0}/?/init.luau;{0}/?/init.lua;{0}", search_path.display());

    let packages: Table = lua.globals().get("package")?;
    packages.set("path", search_path)?;

    return Ok(lua);
}

#[cfg(test)]
mod lua_tests {
    use super::*;
    static BASIC_TESTING_SCRIPT: &'static str = include_str!("./basic_testing.luau");

    fn test_init() -> LuaResult<Lua> {
        let mut lua = init(None)?;
        lua.load(BASIC_TESTING_SCRIPT).exec()?;
        Ok(lua)
    }

    #[test]
    fn testing_script_loads_without_errors() -> LuaResult<()> {
        let _lua = test_init();
        Ok(())
    }

    #[test]
    fn basic_function_works() -> LuaResult<()> {
        let lua = test_init()?;
        let basic_function: LuaFunction = lua.globals().get("basic_function")?;
        let res: usize = basic_function.call(())?;
        assert!(res == 42);
        Ok(())
    }
}

