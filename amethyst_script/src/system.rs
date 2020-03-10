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

pub struct EntityComponents {
    pub transform: Transform,
}

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
    type SystemData = (ReadStorage<'a, Script>, WriteStorage<'a, Transform>);
    
    fn run(&mut self, (scripts, mut transforms): Self::SystemData){
        for (script, mut maybe_transform) in (&scripts, (&mut transforms).maybe()).join() {
            let mut path = PathBuf::from(&self.script_dir);
            path.push(script.path.clone());
            
            //let mut default_t = Transform::default();
            //let mut transform: &mut Transform = match maybe_transform {
            //    Some(mut transform) => &mut transform,
            //    None => {
            //       let mut t = Transform::default();
            //        &mut t
            //   },
            //};
            
            let mut tdf = Transform::default();
            let mut transform: &mut Transform = maybe_transform.unwrap_or(&mut tdf);
            println!("RUST: transform is {}", (*transform).translation().y);
            
            //maybe_transform.unwrap_or(&mut df);

            //if let Some(mut transform) = maybe_transform {
            //    println!("{}", transform.translation().y);
            //    transform.move_up(50.0);
            //}
            
            //let mut components = EntityComponents {transform: transform};
            //transform.move_up(50.0);
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
