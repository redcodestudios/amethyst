use amethyst_core::{
    ecs::{
        storage::VecStorage, Component
    }
};

use std::{
   path::PathBuf, 
};

pub struct Script {
    pub path: PathBuf,
}

impl Script {
    pub fn new(path: PathBuf) -> Self {
        Self { path: path }
    }
    
    pub fn new_from_string<S: Into<String>>(path: S) -> Self {
        Self { path: PathBuf::from(path.into()) }
    }
}

impl Component for Script {
    type Storage = VecStorage<Self>;
}
