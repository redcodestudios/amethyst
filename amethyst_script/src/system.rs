//!Script system
use crate::driver::{Driver, LuaDriver};
use amethyst_core::ecs::System;
use std::fs;


pub struct ScriptSystem;//<D: Driver> {
    //pub script_dir: &'a str,
    //pub driver: D
//}

//impl<'a, D> ScriptSystem<'a, D>
//where
//    D: Driver
//{
//    pub fn new(script_dir: &'a str, driver: D) -> Self {
//        Self {
//            script_dir: script_dir,
//            driver: driver,
//        }
//    }
//}

impl<'a> System<'a> for ScriptSystem
//where
//    D: Driver
{
    type SystemData = ();
    fn run(&mut self, data: Self::SystemData){
        let scripts_entries = fs::read_dir(String::from("examples/hello_script/scripts")).unwrap();
        for script_entry in scripts_entries {
            let script_path = String::from(script_entry.unwrap().path().to_str().unwrap());
            LuaDriver.exec_script(script_path);
        }
    }
}
