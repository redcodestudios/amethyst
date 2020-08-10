use amethyst_assets::*;
use amethyst_error::Error;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct ScriptData(pub Vec<u8>);
amethyst_assets::register_format_type!(ScriptData);

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct LuaFormat;
amethyst_assets::register_format!("lua", LuaFormat as ScriptData);

impl Format<ScriptData> for LuaFormat {
    fn name(&self) -> &'static str {
        "lua"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<ScriptData, Error> {
        Ok(ScriptData(bytes))
    }
}
