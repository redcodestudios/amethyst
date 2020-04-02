use hashbrown::HashMap;
use amethyst_core::{
    ecs::shred::{Accessor, AccessorCow, DynamicSystemData, Fetch, ResourceId, RunNow, System, World},
};

#[derive(Debug)]
pub struct ScriptableResource {
    pub fields: HashMap<String, String>,
}

#[derive(Debug)]
pub struct ScriptingInterface {
    id_alloc: u64,
    type_map: HashMap<String, u64>,
}

impl ScriptingInterface {
    pub fn new() -> Self {
        ScriptingInterface {
            id_alloc: 1,
            type_map: HashMap::new(),
        }
    }

    /// Registers a run-time resource as `name` and adds it to `world`.
    pub fn add_rt_resource(&mut self, name: &str, res: ScriptableResource, world: &mut World) {
        self.type_map.insert(name.to_owned(), self.id_alloc);
        self.id_alloc += 1;

        let id = self.resource_id(name).unwrap();
        world.insert_by_id(id, res);
    }

    pub fn remove_rt_resource(
        &mut self,
        name: &str,
        world: &mut World,
    ) -> Option<ScriptableResource> {
        let id = self.type_map.remove(name);

        id.and_then(|id| {
            world.remove_by_id(ResourceId::new_with_dynamic_id::<ScriptableResource>(id))
        })
    }

    pub fn clear_rt_resources(&mut self, world: &mut World) {
        for &dynamic_id in self.type_map.values() {
            world.remove_by_id::<ScriptableResource>(ResourceId::new_with_dynamic_id::<
                ScriptableResource,
            >(dynamic_id));
        }

        self.type_map.clear();
        self.id_alloc = 1;
    }

    /// Returns the resource ID for the dynamic type identified by `name`
    pub fn resource_id(&self, name: &str) -> Option<ResourceId> {
        self.type_map
            .get(name)
            .cloned()
            .map(ResourceId::new_with_dynamic_id::<ScriptableResource>)
    }
}

#[derive(Clone)]
pub struct ScriptingResAccessor {
    reads: Vec<ResourceId>,
    // could also add `writes` here
}

impl ScriptingResAccessor {
    pub fn new(reads: &[&str], world: &World) -> Self {
        let interface = world.fetch::<ScriptingInterface>();

        ScriptingResAccessor {
            reads: reads
                .into_iter()
                .flat_map(|&name| interface.resource_id(name))
                .collect(),
        }
    }
}

impl Accessor for ScriptingResAccessor {
    fn try_new() -> Option<Self> {
        None
    }

    fn reads(&self) -> Vec<ResourceId> {
        self.reads.clone()
    }

    fn writes(&self) -> Vec<ResourceId> {
        vec![]
    }
}

pub struct ScriptingResData<'a> {
    pub reads: Vec<Fetch<'a, ScriptableResource>>,
}

impl<'a> DynamicSystemData<'a> for ScriptingResData<'a> {
    type Accessor = ScriptingResAccessor;

    fn setup(_accessor: &Self::Accessor, _world: &mut World) {}

    fn fetch(access: &ScriptingResAccessor, world: &'a World) -> Self {
        ScriptingResData {
            reads: access
                .reads
                .iter()
                .map(|id| {
                    world
                        .try_fetch_by_id(id.clone())
                        .expect("Resource no longer exists")
                })
                .collect(),
        }
    }
}

