
use mlua::prelude::*;


pub fn demotest() -> LuaResult<()> {
    let lua = Lua::new();

    let map_table = lua.create_table()?;
    map_table.set(1, "one")?;
    map_table.set("two", 2)?;

    lua.globals().set("map_table", map_table)?;

    lua.load("for k,v in pairs(map_table) do print(k,v) end").exec()?;

    Ok(())
}

pub fn init() -> LuaResult<Lua> {
    let lua = Lua::new();
    return Ok(lua);
}

#[cfg(test)]
mod lua_tests {
    use super::*;
    static BASIC_TESTING_SCRIPT: &'static str = include_str!("./basic_testing.luau");

    fn test_init() -> LuaResult<Lua> {
        let mut lua = init()?;
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

