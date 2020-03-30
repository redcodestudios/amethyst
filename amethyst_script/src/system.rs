//!Script system
use crate::driver::{Driver, LuaDriver};
use crate::component::Script;
use amethyst_core::{
    transform::Transform,
    ecs::{
        System,
        storage::{ReadStorage, WriteStorage},
        Join,
    }
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
    type SystemData = (WriteStorage<'a, Script>, WriteStorage<'a, Transform>);
    
    fn run(&mut self, (mut scripts, mut transforms): Self::SystemData){
        for (mut script, mut maybe_transform) in (&mut scripts, (&mut transforms).maybe()).join() {
            let mut path = PathBuf::from(&self.script_dir);
            path.push(script.path.clone());
            
            if(!script.is_started) {
                D::exec_on_start(path.clone());
                script.is_started = true;
            }
             
            let mut tdf = Transform::default();
            let mut transform: &mut Transform = maybe_transform.unwrap_or(&mut tdf);
            println!("RUST: transform is {}", (*transform).translation().y);
            
            if(path.exists()) {
                unsafe {
                    D::exec_script(path, transform);
                }
            } else if(script.path.exists()) {
                unsafe {
                    D::exec_script(script.path.clone(), transform);
                }
            }else{
                eprintln!("Invalid script path '{}'!", path.display());
            }
        }
    }
}
