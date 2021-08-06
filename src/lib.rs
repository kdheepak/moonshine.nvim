use mlua::chunk;
use mlua::prelude::*;

fn hello(lua: &Lua, _name: String) -> LuaResult<LuaTable> {
    let t = lua.create_table()?;
    let _globals = lua.globals();
    let a = 1;
    let b = 2;
    let name = "world";
    lua.load(chunk! {
        print($a + $b)
        print("hello, " .. $name)
    })
    .exec()?;
    t.set("hello", "hello")?;
    Ok(t)
}

#[mlua::lua_module]
fn moonshine(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("hello", lua.create_function(hello)?)?;
    Ok(exports)
}
