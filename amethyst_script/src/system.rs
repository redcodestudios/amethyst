//!Script system
use crate::driver::{Driver, LuaDriver};
use crate::component::Script;
use amethyst_core::ecs::{
    System,
    storage::ReadStorage,
    Join,
};
use std::{
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};


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
    fn run(&mut self, scripts: Self::SystemData){
        for script in scripts.join() {
            let mut path = PathBuf::from(&self.script_dir);
            path.push(script.path.clone());
            if(path.exists()) {
                D::exec_script(path);
            } else if(script.path.exists()) {
                D::exec_script(script.path.clone());
            }else{
                eprintln!("Invalid script path '{}'!", path.display());
            }
        }
    }
}
