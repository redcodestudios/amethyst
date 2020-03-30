use amethyst_core::{
    ecs::{
        Entity,
        storage::VecStorage, Component,
    }
};

use std::{
   path::PathBuf, 
};

#[derive(Debug, Clone)]
pub struct Script {
    pub is_started: bool,
    pub path: PathBuf,
}

impl Script {
    pub fn new(path: PathBuf) -> Self {
        Self { is_started: false, path: path}
    }
    
    pub fn new_from_string<S: Into<String>>(path: S) -> Self {
        Self { is_started: false, path: PathBuf::from(path.into())}
    }
}

impl Component for Script {
    type Storage = VecStorage<Self>;
}
