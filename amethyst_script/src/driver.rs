use std::ffi::CString;
use std::os::raw::c_char;
use std::path::PathBuf;

use amethyst_core::transform::Transform;

// C driver functions
extern {
    fn call_python(path: *const c_char, transform: *mut Transform);
    fn call_on_start_py(path: *const c_char);
    

    fn call_lua(path: *const c_char, transform: *mut Transform);
    fn call_on_start_lua(path: *const c_char);
}

pub trait Driver {
    fn exec_script(path: PathBuf, transform: *mut Transform) -> Result<(), ()>;
    fn exec_on_start(path: PathBuf) -> Result<(), ()>;
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

    fn exec_on_start(path: PathBuf) -> Result<(), ()> {
        unsafe {
            let script_path = String::from(path.to_str().unwrap());

            call_on_start_py(
                CString::new(script_path).expect("CString::new failed").as_ptr()
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

    fn exec_on_start(path: PathBuf) -> Result<(), ()> {
        unsafe {
            let script_path = String::from(path.to_str().unwrap());

            call_on_start_lua(
                CString::new(script_path).expect("CString::new failed").as_ptr()
            );
        }
        Ok(())
    }
}

