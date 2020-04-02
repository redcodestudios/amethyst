//!Script system
use crate::{
        asset::Script,
        formats::{LuaFormat, ScriptData},
        driver::{Driver, Language, LuaVM},
        res::*,
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
        Entities,
        shred::{Accessor, AccessorCow, System as ShredSys, SystemData as SysDataShred, RunNow},
    }
};

use amethyst_assets::{
    AssetStorage, Handle, Loader, Processor,
};

use std::{
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
    collections::HashMap,
};


type ScriptHandles = Vec<Option<Handle<Script>>>;
type ScriptableResources = HashMap<String, HashMap<String, String>>;

pub struct ScriptSystem<D: Driver> {
    handles: ScriptHandles,
    driver: D,
    script_world: World,
    accessor: ScriptingResAccessor,
}

impl<D: Driver> ScriptSystem<D> {
    fn new(handles: ScriptHandles, driver: D, script_world: World, accessor: ScriptingResAccessor) -> Self {
        Self {
            handles: handles,
            driver: driver,
            script_world: script_world,
            accessor: accessor,
        }
    }
}

impl<'a, D: std::clone::Clone + Driver> System<'a> for ScriptSystem<D> {
    type SystemData = Read<'a, AssetStorage::<Script>>;

    fn run(&mut self, assets: Self::SystemData) {
        for sh in &self.handles {
            if let Some(s) = sh.as_ref().and_then(|sh| assets.get(sh)){
                self.driver.clone().run(s.bytes.clone());
                let mut sys = MySys{accessor: self.accessor.clone()};
                sys.run_now(&self.script_world);
            }else{
                println!("lixo");
            }
        }
    }
}

pub struct MySys {
    accessor: ScriptingResAccessor,
}

impl<'a> ShredSys<'a> for MySys {
    type SystemData = ScriptingResData<'a>;
    
    fn run(&mut self, data: Self::SystemData) {
        for scripting_resource in data.reads {
            println!(
                "Fields of run-time resource: {:?}",
                scripting_resource.fields
            );
        }
    }

    fn accessor<'b>(&'b self) -> AccessorCow<'a, 'b, Self> {
        AccessorCow::Ref(&self.accessor)
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

        let mut script_world = World::empty();

        let mut interface = ScriptingInterface::new();

        interface.add_rt_resource(
            "Foo",
            ScriptableResource {
                fields: (vec![("foo_field".to_owned(), "5".to_owned())].into_iter().collect()),
            },
            &mut script_world,
        );
        script_world.insert(interface); 
        let mut accessor = ScriptingResAccessor::new(&["Foo"], &script_world);

        ScriptSystem::<D>::new(
            self.load_multiple_lua_scripts(world),
            D::new(),
            script_world,
            accessor,
        )
    }
}
