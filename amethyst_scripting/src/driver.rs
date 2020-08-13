use std::path::PathBuf;
use std::sync::{Arc, Mutex};

extern crate cbindgen;

#[repr(C)]
#[derive(Clone)]
pub struct lua_State { private: [u8; 0] }

extern {
    fn luaL_newstate() -> *mut lua_State;
    fn lua_close(l: *mut lua_State);
    fn luaL_openlibs(l: *mut lua_State);
    fn C_call_lua_bytes(l: *mut lua_State, bytes: *const u8, size: usize);
}

#[derive(Debug)]
pub enum Language {
    Lua(PathBuf),
}

pub trait Driver {
    fn new() -> Self;
    fn run(self, source: Vec<u8>);
}

#[derive(Clone)]
pub struct LuaVM {
    state: Arc<Mutex<*mut lua_State>>,
}

impl LuaVM {
    fn clean_state(&mut self) {
        unsafe {
            let s = luaL_newstate();
            luaL_openlibs(s);
            let mut old_state = self.state.lock().unwrap();
            lua_close(*old_state);
            *old_state = s;
        }
    }
}

unsafe impl Send for LuaVM {}

impl Driver for LuaVM {
    fn new() -> Self {
        unsafe {
            let s = luaL_newstate();
            luaL_openlibs(s);
            Self { state: Arc::new(Mutex::new(s)) }
        }
    }

    fn run(mut self, source: Vec<u8>) {
        unsafe {
            &self.clean_state();
            let s = *Arc::try_unwrap(self.state).unwrap_err().lock().unwrap();
            C_call_lua_bytes(s, source.as_ptr(), source.len());
        }
    }
}

pub fn load_components(path: PathBuf) {
    cbindgen::Builder::new()
        .with_src(path)
        .with_language(cbindgen::Language::C)
        .generate()
        .unwrap()
        .write_to_file("amethyst_scripting/c_drivers/generated_engine.h");
}
