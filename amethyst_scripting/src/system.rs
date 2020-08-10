use crate::{
    asset::Script,
    formats::{LuaFormat},
    driver::{Driver},
};
use amethyst_core::{
    SystemDesc,
    ecs::{
        SystemData,
        WorldExt,
        World,
        System,
        Read,
    }
};

use amethyst_assets::{
    AssetStorage, Handle, Loader,
};

use std::{
    fs,
    marker::PhantomData,
    path::{PathBuf},
};

type ScriptHandles = Vec<Option<Handle<Script>>>;

pub struct ScriptSystem<D> {
    script_handles: ScriptHandles,
    driver: D,
}

impl<D: Driver> ScriptSystem<D> {
    fn new(script_handles: ScriptHandles, driver: D) -> Self {
        Self { script_handles, driver }
    }
}

impl<'a, D: std::clone::Clone + Driver> System<'a> for ScriptSystem<D> {
    type SystemData = Read<'a, AssetStorage::<Script>>;

    fn run(&mut self, assets: Self::SystemData) {
        for sh in &self.script_handles {
            if let Some(script) = sh.as_ref().and_then(|sh| assets.get(sh)) {
                self.driver.clone().run(script.bytes.clone()); 
            }
        }
    }
}

pub struct ScriptSystemDesc<D: Driver> {
    path: PathBuf,
    phantom: PhantomData<D>,
}

impl<D: Driver> ScriptSystemDesc<D> {
    pub fn new(path: PathBuf) -> Self {
        Self { path, phantom: PhantomData }
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

    fn load_multiple_lua_scripts(self, world: &mut World) -> Vec<Option<Handle<Script>>> {
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
        ScriptSystem::<D>::new(self.load_multiple_lua_scripts(world), D::new())
    }
}
