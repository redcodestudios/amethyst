//!Script system
use crate::driver::{Driver, LuaDriver};
use crate::component::Script;
use crate::{
        asset::Script as ScriptAsset,
        formats::{LuaFormat, ScriptData},
};
use amethyst_core::{
    SystemDesc,
    transform::Transform,
    ecs::{
        SystemData,
        WorldExt,
        World,
        System,
        Read,
        storage::{ReadStorage, WriteStorage},
        Join,
    }
};

use amethyst_assets::{
    AssetStorage, Handle, Loader, Processor,
};

use std::{
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

fn load_script(world: &mut World) -> Handle<ScriptAsset> {

    let loader = world.read_resource::<Loader>();
    let script_storage = world.read_resource::<AssetStorage<ScriptAsset>>();
    
    loader.load(
        "scripts/lua/pong.lua",
        LuaFormat::default(),
        (),
        &script_storage,
    )
}

type ScriptHandles = Vec<Option<Handle<ScriptAsset>>>;

#[derive(Default)]
pub struct ScriptAssetSystem{
    handles: ScriptHandles,
}

impl ScriptAssetSystem {
    fn new(handles: ScriptHandles) -> Self {
        Self {handles: handles}
    }
}

impl<'a> System<'a> for ScriptAssetSystem {
    type SystemData = Read<'a, AssetStorage::<ScriptAsset>>;

    fn run(&mut self, data: Self::SystemData) {
        for sh in &self.handles {
            if let Some(s) = sh.as_ref().and_then(|sh| data.get(sh)){
                println!("{}", s.clone().to_string().unwrap());
            }else{
                println!("lixo");
            }
        }
    }
}

pub struct ScriptAssetSystemDesc<D: Driver> {
    phantom: PhantomData<D>,
}

impl <D: Driver>ScriptAssetSystemDesc<D> {
    pub fn new() -> Self {
        Self {phantom: PhantomData}
    }
}

impl<'a, 'b, D: Driver> SystemDesc<'a, 'b, ScriptAssetSystem> for ScriptAssetSystemDesc<D> {
    fn build(self, world: &mut World) -> ScriptAssetSystem {
        <ScriptAssetSystem as System<'_>>::SystemData::setup(world);
        ScriptAssetSystem::new(vec![Some(load_script(world))])
    }
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
