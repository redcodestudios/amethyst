use crate::{
    asset::Script,
    system::{ScriptSystemDesc, ScriptSystem},
    driver::{Driver, Language, LuaVM},
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
    languages: Vec<Language>
}

impl<'a> ScriptBundle<'a> {
    /// Create a new script bundle
    pub fn new() -> Self {
        ScriptBundle {
            dep: Default::default(),
            languages: Vec::new()
        }
    }

    pub fn with_language(mut self, lang: Language) -> Self {
        self.languages.push(lang);
        self
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
            Processor::<Script>::new(),
            "script_asset_processor",
            self.dep,
        );

        for lang in self.languages {
            match lang {
                Language::Lua(path) => {
                    builder.add(
                        ScriptSystemDesc::<LuaVM>::new(path).build(world),
                        "script_system_lua",
                        &["script_asset_processor"],
                    );
                },
            }
        }
        Ok(())
    }
}
