use crate::{
    asset::Script as ScriptAsset,
    system::{ScriptAssetSystemDesc, ScriptAssetSystem},
    driver::LuaDriver,
};

use amethyst_error::Error;

use amethyst_core::{
    ecs::prelude::{DispatcherBuilder, World},
    bundle::SystemBundle,
    SystemDesc,
};

use amethyst_assets::Processor;

#[derive(Debug, Default)]
pub struct ScriptBundle<'a> {
    dep: &'a [&'a str],
}

impl<'a> ScriptBundle<'a> {
    /// Create a new script bundle
    pub fn new() -> Self {
        ScriptBundle {
            dep: Default::default(),
        }
    }

    /// Set dependencies for the `ScriptSystem`
    pub fn with_dep(mut self, dep: &'a [&'a str]) -> Self {
        self.dep = dep;
        self
    }
}

impl<'a, 'b, 'c> SystemBundle<'a, 'b> for ScriptBundle<'c> {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<ScriptAsset>::new(),
            "script_asset_processor",
            self.dep,
        );
        builder.add(
            ScriptAssetSystemDesc::<LuaDriver>::new().build(world),
            "script_asset_system_desc",
            &["script_asset_processor"],
        );
        Ok(())
    }
}
