//! The simplest Amethyst example.

use amethyst::{
    prelude::*,
    core::{TransformBundle, transform::Transform},
    ecs::{storage::Storage},
    script::{
        system::ScriptSystem,
        driver::{LuaDriver, PythonDriver},
        component::Script,
    },
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub struct Pong;
impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let entity = world
            .create_entity()
            .with(Script::new_from_string("pong.lua"))
            .with(Transform::default())
            .build();    
    }
}

/*#
 * pub struct ScriptSys;
 *
 * impl System for ScriptSys {
 *  type Storage = (ReadStorage<'a, Transform>)
 * }
 * */

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("examples/script/config/display.ron");
    let lua_scripts_path = app_root.join("examples/script/scripts/lua");
    let python_scripts_path = app_root.join("examples/script/scripts/python");

    // This line is not mentioned in the pong tutorial as it is specific to the context
    // of the git repository. It only is a different location to load the assets from.
    let assets_dir = app_root.join("examples/assets/");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and
                // drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with(ScriptSystem::<LuaDriver>::new(lua_scripts_path), "lua_script_system", &[])
        .with(ScriptSystem::<PythonDriver>::new(python_scripts_path), "python_script_system", &[]);

    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();
    Ok(())
}
