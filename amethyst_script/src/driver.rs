use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::mem;

#[repr(C)]
#[derive(Clone)]
pub struct lua_State { private: [u8; 0] }

extern { 
    fn luaL_newstate() -> *mut lua_State;
    fn lua_close(l: *mut lua_State);
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
    state: Arc<Mutex<*mut lua_State>>
}

impl LuaVM {
    fn clean_state(&mut self) {
        unsafe {
            let mut s = luaL_newstate();
            luaL_openlibs(s);
            let mut old_state = self.state.lock().unwrap();
            lua_close(*old_state);
            *old_state = s;
        }
    }
}

unsafe impl Send for LuaVM{}

impl Driver for LuaVM {
    fn new() -> Self {
        unsafe {
            let s = luaL_newstate();
            luaL_openlibs(s);
            Self {state: Arc::new(Mutex::new(s))}
        }
    }


    fn run(mut self, source: Vec<u8>) {
        unsafe {
            &self.clean_state();
            let s = *Arc::try_unwrap(self.state).unwrap_err().lock().unwrap();
            test_call_lua(s, source.as_ptr(), source.len());
        }
    }
}

