//!Script system
use crate::driver::Driver;
use amethyst_core::ecs::System;
use std::fs;


pub struct ScriptSystem<'a, D: Driver> {
    pub script_dir: &'a str,
    pub driver: D
}

impl<'a, D> ScriptSystem<'a, D>
where
    D: Driver
{
    pub fn new(script_dir: &'a str, driver: D) -> Self {
        Self {
            script_dir: script_dir,
            driver: driver,
        }
    }
}

impl<'a, D> System<'a> for ScriptSystem<'a, D>
where
    D: Driver
{
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData){
        let scripts_entries = fs::read_dir(self.script_dir.to_string()).unwrap();
        for script_entry in scripts_entries {
            let script_path = String::from(script_entry.unwrap().path().to_str().unwrap());
            self.driver.exec_script(script_path);
        }
    }
}
