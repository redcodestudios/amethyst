use std::ffi::CString;
use std::os::raw::c_char;
use std::path::PathBuf;


// C driver functions
extern {
    fn call_python(path: *const c_char);
    fn call_lua(path: *const c_char);
}

pub enum Language {
    Python,
    Lua,
}

pub trait Driver {
    fn exec_script(path: PathBuf) -> Result<(), ()>;
}

pub struct PythonDriver;
impl Driver for PythonDriver {
    fn exec_script(path: PathBuf) -> Result<(), ()>{
        unsafe{
            let script_path = String::from(path.to_str().unwrap());
            call_python(CString::new(script_path).expect("CString::new failed").as_ptr());
        }
        Ok(())
    }
}

pub struct LuaDriver;
impl Driver for LuaDriver {
    fn exec_script(path: PathBuf) -> Result<(), ()> {
        unsafe{
            let script_path = String::from(path.to_str().unwrap());
            call_lua(CString::new(script_path).expect("CString::new failed").as_ptr());
        }
        Ok(())
    }
}

pub struct NotImplementedDriver;
impl Driver for NotImplementedDriver {
    fn exec_script(path: PathBuf) -> Result<(), ()> {
        eprintln!("Script driver for this language is not implemented!");
        Ok(())
    }
}
