use std::os::raw::c_char;
use std::ffi::CStr;

#[repr(C)]
pub struct Test {
    name: *const c_char,
}

#[repr(C)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub t: Test,
}

#[no_mangle]
pub extern "C" fn tst(t: *mut Test) {
    println!("yey");
}

#[no_mangle]
pub extern "C" fn translate(pos: *mut Position) {
    println!("yey");
}

pub fn c_to_rust(c_str: *const c_char) -> String {
    unsafe{
        CStr::from_ptr(c_str).to_string_lossy().into_owned()
    }
}

#[no_mangle]
pub extern "C" fn rust_log(message: *const c_char) {
    println!(" --- RUST_LOG_: {} ---", c_to_rust(message));
}
