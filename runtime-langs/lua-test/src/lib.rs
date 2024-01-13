use mlua::{AnyUserData, MetaMethod, MultiValue, UserData, UserDataRef};
use mlua::prelude::*;

pub trait Object {
    fn print(&self);

    fn call_method<'l>(&self, lua: &'l Lua, name: &str, args: MultiValue<'l>) -> LuaResult<MultiValue<'l>>;
}

impl UserData for Box<dyn Object> {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::Index, |lua, (_, name): (AnyUserData, String)| {
            lua.create_function(move |lua, (this, args): (UserDataRef<Self>, MultiValue)| {
                this.call_method(lua, &*name, args)
            })
        });
    }
}

pub struct ConcreteObject {}

impl Object for ConcreteObject {
    fn print(&self) {
        println!("I am ConcreteObject!!!");
    }

    fn call_method<'l>(&self, lua: &'l Lua, name: &str, args: MultiValue<'l>) -> LuaResult<MultiValue<'l>> {
        lua.scope(|scope| scope.create_userdata_ref(self)?.call_method(name, args))
    }
}

impl UserData for ConcreteObject {}

pub struct OtherObject {}

impl Object for OtherObject {
    fn print(&self) {
        println!("I am OtherObject!!");
    }

    fn call_method<'l>(&self, lua: &'l Lua, name: &str, args: MultiValue<'l>) -> LuaResult<MultiValue<'l>> {
        lua.scope(|scope| scope.create_userdata_ref(self)?.call_method(name, args))
    }
}

impl UserData for OtherObject {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("DoSomething", |_lua, this, other: AnyUserData| {
            let other = other.borrow::<Box<dyn Object>>()?;
            this.print();
            other.print();
            Ok(())
        });
    }
}

#[cfg(test)]
mod tests {
    use mlua::Lua;
    use crate::{ConcreteObject, Object, OtherObject};

    #[test]
    fn main() {
        let lua = Lua::new();

        let ud = lua.create_userdata::<Box<dyn Object>>(Box::new(ConcreteObject{})).unwrap();
        let ud2 = lua.create_userdata::<Box<dyn Object>>(Box::new(OtherObject{})).unwrap();
        let other = OtherObject{};

        lua.globals().set("object1", ud).unwrap();
        lua.globals().set("object2", ud2).unwrap();
        lua.globals().set("other", other).unwrap();

        lua.load("
        print(object1)
        print(object2)
        print(other)
        print(other.DoSomething)
        print(object2.DoSomething)
        object2:DoSomething(object2)
    ").exec().unwrap();
    }
}

