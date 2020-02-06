//!Script system
use crate::driver::{Driver, LuaDriver};
use crate::component::Script;
use amethyst_core::ecs::{
    System,
    storage::ReadStorage,
};
use std::{
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};


//#[derive(Debug, derivative::Derivative)]
//#[derivative(Default(bound = ""))]
pub struct ScriptSystem<D: Driver> {
    script_dir: PathBuf,
    phantom: PhantomData<D>
}

impl <D: Driver> ScriptSystem<D> {
    pub fn new(script_dir: PathBuf) -> Self {
        Self {
            script_dir: PathBuf::from(script_dir),
            phantom: PhantomData,
        }
    }
}

impl<'a, D: Driver> System<'a> for ScriptSystem<D> {
    type SystemData = (ReadStorage<'a, Script>);
    fn run(&mut self, data: Self::SystemData){
        let scripts_entries = fs::read_dir(&self.script_dir).unwrap();
        for script_entry in scripts_entries {
            let script_path = String::from(script_entry.unwrap().path().to_str().unwrap());
            D::exec_script(script_path);
        }
    }
}
