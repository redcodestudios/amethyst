//!Script system
use crate::{
        asset::Script,
        formats::{LuaFormat, ScriptData},
        driver::{Driver, Language, LuaVM}
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


type ScriptHandles = Vec<Option<Handle<Script>>>;
type Pool<T> = Vec<T>;

#[derive(Default)]
pub struct ScriptSystem<D: Driver> {
    handles: ScriptHandles,
    pool: Pool<D>,
}

impl<D: Driver> ScriptSystem<D> {
    fn new(handles: ScriptHandles, pool: Pool<D>) -> Self {
        Self {handles: handles, pool: pool}
    }
}

impl<'a, D: std::clone::Clone + Driver> System<'a> for ScriptSystem<D> {
    type SystemData = Read<'a, AssetStorage::<Script>>;

    fn run(&mut self, data: Self::SystemData) {
        for sh in &self.handles {
            if let Some(s) = sh.as_ref().and_then(|sh| data.get(sh)){
                self.pool[0].clone().run(s.bytes.clone()); 
                //println!("{}", s.clone().to_string().unwrap());
            }else{
                println!("lixo");
            }
        }
    }
}

pub struct ScriptSystemDesc<D: Driver>{
    path: PathBuf,
    phantom: PhantomData<D>,
}

impl<D: Driver> ScriptSystemDesc<D> {
    pub fn new(path: PathBuf) -> Self {
        Self {path: path, phantom: PhantomData}
    }
    
    fn load_lua_script(world: &mut World, path: &str) -> Handle<Script> {

        let loader = world.read_resource::<Loader>();
        let script_storage = world.read_resource::<AssetStorage<Script>>();
    
        loader.load(
            path,
            LuaFormat::default(),
            (),
            &script_storage,
        )
    }

    fn load_multiple_lua_scripts(self, world: &mut World) -> Vec<Option<Handle<Script>>>{
        let mut handles = Vec::new();
        for entry in fs::read_dir(self.path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            handles.push(Some(Self::load_lua_script(world, path.to_str().unwrap())));
        }
        handles
    }
}

impl<'a, 'b, D: Driver + std::clone::Clone> SystemDesc<'a, 'b, ScriptSystem<D>> for ScriptSystemDesc<D> {
    fn build(self, world: &mut World) -> ScriptSystem<D> {
        <ScriptSystem<D> as System<'_>>::SystemData::setup(world);

        let mut pool = Vec::with_capacity(10);
        for _ in 0..10 {
            pool.push(D::new());
        }
        ScriptSystem::<D>::new(self.load_multiple_lua_scripts(world), pool)
    }
}
