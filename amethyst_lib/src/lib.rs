use std::os::raw::{c_char, c_int, c_float};
use std::ffi::CStr;
use std::ptr;

use amethyst_core::{
    transform::Transform,
    ecs::{
        Entities,
        Component,
    }
};

//pub struct ScriptContext {
//    pub components_map: HashMap<String, (fn() -> dyn Component)>;
//}

// duplicated structure because of cyclic dependecy
pub struct EntityComponents {
    pub transform: Transform,
}

//#[no_mangle]
//pub extern "C" fn create_entity(entities: Entities, component: String,) {
    //let builder = entities.build_entity();

    //match component.as_str() {
    //    "Transform" => {
    //        builder.with(Transform::default());
    //        builder.build();
    //    },
    //    _ => {builder.build();},
    //}
//}

#[no_mangle]
pub extern "C" fn get_transform(components: *mut EntityComponents) -> *mut &'static Transform {
    unsafe{
        let t = &(*components).transform;    
        let b = Box::new(t);
        let raw = Box::into_raw(b);
        raw
    }
    //println!("before: {}", t.translation().y);
    //let raw = &*(t as *mut Transform);
}

//#[no_mangle]
//pub extern "C" fn set_transform(components: *mut EntityComponents, transform: mut Transform) {
//    unsafe {
//        (*components).transform = transform;
//    }
//}

#[no_mangle]
pub extern "C" fn set_transform(components: *mut EntityComponents, t: *mut Transform) {
    unsafe{
        (*components).transform = (*t).clone();
    }
}

#[no_mangle]
pub extern "C" fn move_up(t: *mut Transform, amount: c_float) {
    unsafe{
        //println!("MOVE UP SAYS: ADDRESS IS {:p}", &((*t).translation().y));
        (*t).move_up(amount);
    }
}

#[no_mangle]
pub extern "C" fn translation_y(t: *mut Transform) -> c_float{
    unsafe{
        (*t).translation().y
    }
}

#[no_mangle]
pub extern "C" fn print_addr(t: *mut Transform) {
    unsafe{
        println!("PRINT_ADDR: ADDRESS IS {:p}", &((*t).translation().y));
    }
}

/* fn get_meta() -> *mut MetaComponent | MetaComponent* get_meta();
 * fn get_component(meta: *mut MetaComponent, component: *c_char) -> | Component* get_component(MetaComponent* meta, char* component)
 * fn add_component(meta: *mut MetaComponent, name: *c_char)
 
 * */


//pub struct ScriptableComponents {
//    components: Vec<Box<dyn Component>>,
//}

//impl Component for ScriptableComponents {
//    type Storage = VecStorage<Self>;
//}

//#[no_mangle]
//pub extern "C" fn new_component() -> *mut Component {
//   println!("creating component");
//    Box::new(Component::new)
//}


//#[repr(C)]
//pub struct Position {
//    pub x: i32,
//    pub y: i32,
//    pub t: Test,
//}


//#[no_mangle]
//pub extern "C" fn tst(t: *mut Test) {
//    println!("yey");
//}


pub fn c_to_rust(c_str: *const c_char) -> String {
    unsafe{
        CStr::from_ptr(c_str).to_string_lossy().into_owned()
    }
}

#[no_mangle]
pub extern "C" fn rust_log(message: *const c_char) {
    println!(" --- RUST_LOG_: {} ---", c_to_rust(message));
}
