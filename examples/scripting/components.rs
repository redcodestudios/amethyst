use amethyst::prelude::*;


#[derive(Component)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

#[no_mangle]
pub extern "C" fn get_r(colors: *mut Color) -> u8 {
    (*colors).r
}


#[no_mangle]
pub extern "C" fn get_g(colors: *mut Color) -> u8 {
    (*colors).g
}


#[no_mangle]
pub extern "C" fn get_b(colors: *mut Color) -> u8 {
    (*colors).b
}


#[no_mangle]
pub extern "C" fn set_r(colors: *mut Color, c: u8) {
    (*colors).r = c
}


#[no_mangle]
pub extern "C" fn set_g(colors: *mut Color, c: u8) {
    (*colors).g = c
}

#[no_mangle]
pub extern "C" fn set_b(colors: *mut Color, c: u8) {
    (*colors).b = c
}
