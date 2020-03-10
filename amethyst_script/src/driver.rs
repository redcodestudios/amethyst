use std::ffi::CString;
use std::os::raw::c_char;
use std::path::PathBuf;

use crate::system::EntityComponents;
use amethyst_core::transform::Transform;

// C driver functions
extern {
    fn call_python(path: *const c_char, transform: *mut Transform);
    fn call_lua(path: *const c_char, transform: *mut Transform);
}

pub trait Driver {
    fn exec_script(path: PathBuf, transform: *mut Transform) -> Result<(), ()>;
}

pub struct PythonDriver;
impl Driver for PythonDriver {
    fn exec_script(path: PathBuf, transform: *mut Transform) -> Result<(), ()>{
        unsafe{
            let script_path = String::from(path.to_str().unwrap());
            //let b = Box::new(transform);
            //let transform_ptr = Box::into_raw(b);
            
            call_python(
                CString::new(script_path).expect("CString::new failed").as_ptr(),
                transform
            );
        }
        Ok(())
    }
}

pub struct LuaDriver;
impl Driver for LuaDriver {
    fn exec_script(path: PathBuf,  transform: *mut Transform) -> Result<(), ()> {
        unsafe{
            let script_path = String::from(path.to_str().unwrap());
            //let b = Box::new(transform);
            //let transform_ptr = Box::into_raw(b);
            
            call_lua(
                CString::new(script_path).expect("CString::new failed").as_ptr(),
                transform
            );
        }
        Ok(())
    }
}

