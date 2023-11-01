// Standard Uses
use std::env;
use std::path::PathBuf;

// Crate Uses

// External Uses
use mlua::{Lua, Result};


#[test]
fn test() -> Result<()> {
    let lua = make_lua()?;
    lua.load(
        r#"
        local yaml = require("test_lib")

        local function equals(t1, t2)
            if type(t1) == "table" and type(t2) == "table" and #t1 == #t2 then
                local eq = true
                for k in pairs(t1) do eq = eq and equals(t1[k], t2[k]) end
                for k in pairs(t2) do eq = eq and equals(t1[k], t2[k]) end
                return eq
            end
            return t1 == t2
        end

        local t1 = {
            a = yaml.null,
            b = "hello",
            c = setmetatable({}, yaml.array_mt),
            d = 4,
        }
        local encoded = yaml.encode(t1)
        assert(string.match(encoded, "c: %[%]") ~= nil)
        local t2 = yaml.decode(encoded)
        assert(equals(t1, t2))
    "#,
    ).exec()
}

fn make_lua() -> Result<Lua> {
    let (dylib_path, dylib_ext, separator);
    if cfg!(target_os = "macos") {
        dylib_path = env::var("DYLD_FALLBACK_LIBRARY_PATH").unwrap();
        dylib_ext = "dylib";
        separator = ":";
    } else if cfg!(target_os = "linux") {
        dylib_path = env::var("LD_LIBRARY_PATH").unwrap();
        dylib_ext = "so";
        separator = ":";
    } else if cfg!(target_os = "windows") {
        dylib_path = env::var("PATH").unwrap();
        dylib_ext = "dll";
        separator = ";";
    } else {
        panic!("unknown target os");
    };

    let mut cpath = dylib_path
        .split(separator)
        .take(3)
        .map(|p| {
            let mut path = PathBuf::from(p);
            path.push(format!("lib?.{}", dylib_ext));
            path.to_str().unwrap().to_owned()
        })
        .collect::<Vec<_>>()
        .join(";");

    if cfg!(target_os = "windows") {
        cpath = cpath.replace("\\", "\\\\");
        cpath = cpath.replace("lib?.", "?.");
    }

    let lua = unsafe { Lua::unsafe_new() }; // To be able to load C modules
    lua.load(&format!("package.cpath = \"{}\"", cpath)).exec()?;
    Ok(lua)
}
