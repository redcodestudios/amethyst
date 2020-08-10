use amethyst_assets::{ Asset, Handle, ProcessableAsset, ProcessingState,
};

use amethyst_core::ecs::prelude::VecStorage;
use amethyst_error::Error;


use crate::formats::ScriptData;


pub type ScriptHandle = Handle<Script>;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Script {
    pub bytes: Vec<u8>,
}

impl Script {
    pub fn to_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.bytes)
    }
}

impl Asset for Script {
    const NAME: &'static str = "scripting::Script";

    type Data = ScriptData;
    type HandleStorage = VecStorage<ScriptHandle>;
}

impl ProcessableAsset for Script {
    fn process(data: ScriptData) -> Result<ProcessingState<Script>, Error> {
        Ok(ProcessingState::Loaded(Script { bytes: data.0 }))
    }
}
