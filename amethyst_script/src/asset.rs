//! Provides structures used to load script files.
//!
use amethyst_assets::{
    Asset, AssetStorage, Handle, Loader, ProcessableAsset, ProcessingState,
};
use amethyst_core::ecs::prelude::VecStorage;
use amethyst_error::Error;

use std::string::FromUtf8Error;

use crate::formats::ScriptData;

// Hold a reference to a script asset loaded
pub type ScriptHandle = Handle<Script>;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Script {
    pub bytes: Vec<u8>,
}

impl Script {
    pub fn to_string(self) -> Result<String, std::string::FromUtf8Error> {
        match String::from_utf8(self.bytes) {
            Ok(utf8_str) => Ok(String::from(utf8_str)),
            Err(error) => Err(error)
        }
    }
}

impl Asset for Script {
    const NAME: &'static str = "script::Script";

    type Data = ScriptData;
    type HandleStorage = VecStorage<ScriptHandle>;

}

impl ProcessableAsset for Script {
    fn process(data: ScriptData) -> Result<ProcessingState<Script>, Error> {
        Ok(ProcessingState::Loaded(Script { bytes: data.0 }))
    }
}
