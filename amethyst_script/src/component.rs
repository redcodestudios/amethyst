use amethyst_core::{
    ecs::{
        storage::VecStorage, Component
    }
};

pub struct Script {
    path: String,
}

impl Script {
    pub fn new<S: Into<String>>(path: S) -> Self {
        Self { path: path.into() }
    }
}

impl Component for Script {
    type Storage = VecStorage<Self>;
}
