use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[repr(C)]
#[derive(Clone)]
pub struct lua_State { private: [u8; 0] }

extern { 
    fn luaL_newstate() -> *mut lua_State;
    fn luaL_openlibs(l: *mut lua_State);
    fn test_call_lua(l: *mut lua_State, source: *const u8, size: usize);
}

#[derive(Debug)]
pub enum Language {
    Lua(PathBuf),
    Python(PathBuf),
}

pub trait Driver {
    fn new() -> Self;
    fn run(self, source: Vec<u8>);
}

#[derive(Clone)]
pub struct LuaVM {
    running: bool,
    state: Arc<Mutex<*mut lua_State>>
}

unsafe impl Send for LuaVM{}

impl Driver for LuaVM {
    fn new() -> Self {
        unsafe {
            let s = luaL_newstate();
            luaL_openlibs(s);
            Self {running: false, state: Arc::new(Mutex::new(s))}
        }
    }

    fn run(mut self, source: Vec<u8>) {
        self.running = true;
        unsafe {
            let s = *Arc::try_unwrap(self.state).unwrap_err().lock().unwrap();
            test_call_lua(s, source.as_ptr(), source.len());
        }
        self.running = false;
    }
}

