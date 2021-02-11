pub type ComponentId = u64;
pub type Scripts = Vec<String>;

use log::*;

pub fn create_scripting_system(
    scripts: Vec<String>,
    id_count: &mut u64,
) -> Box<dyn FnMut(&mut amethyst_core::ecs::World, &mut amethyst_core::ecs::Resources) + '_> {
    Box::new(move |_world: &mut amethyst_core::ecs::World, _resources| {
        debug!("World len do system {}", _world.len());
        for s in &scripts {
            crate::driver::run_script(_world, s.clone(), id_count);
        }
    })
}
