//! The simplest Amethyst example.

use amethyst::{
    prelude::*,
    assets::{AssetStorage, Handle, Loader, Processor},
    core::{TransformBundle, transform::Transform},
    ecs::{storage::Storage, VecStorage, Component},
    script::{
        system::ScriptSystem,
        driver::{LuaDriver, PythonDriver},
        component::Script,
        asset::Script as ScriptAsset,
        formats::{LuaFormat, ScriptData},
    },
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle, Camera, ImageFormat,
        SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture
    },
    utils::application_root_dir,
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // Assign the sprite for the ball
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1, // ball is the second sprite on the sprite_sheet
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .with(Script::new_from_string("pong.lua"))
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

fn load_script(world: &mut World) -> Handle<ScriptAsset> {

    let loader = world.read_resource::<Loader>();
    let script_storage = world.read_resource::<AssetStorage<ScriptAsset>>();
    
    loader.load(
        "scripts/lua/pong.lua",
        LuaFormat::default(),
        (),
        &script_storage,
    )
}

#[derive(Default)]
pub struct Pong {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    script_handle: Option<Handle<ScriptAsset>>,
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        
        self.script_handle.replace(load_script(world));

        //let storage = Read<AssetStorage::<ScriptAsset>>;
        //let s = storage.get(script_handle);
        //let entity = world
        //    .create_entity()
        //    .with(Script::new_from_string("pong.lua"))
        //    .with(Transform::default())
        //    .build();    
        
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        initialise_camera(world);
        initialise_ball(world, self.sprite_sheet_handle.clone().unwrap());
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans{ 
    //
    //    let storage = AssetStorage::<ScriptAsset>::new();
    //    
    //    match storage.get(self.script_handle.as_ref().unwrap()){
    //        Some(s) => {
    //            let s1 = s.clone().to_string().unwrap();
    //            println!("{}", s1);
    //        },
    //         _ => println!("miou")
    //    }
        
         // importante
        data.data.update(&mut data.world);
        
        let storage = data.world.read_resource::<AssetStorage<ScriptAsset>>();
        for sh in [&self.script_handle].iter() {
            if let Some(s) = sh.as_ref().and_then(|sh| storage.get(sh)){
                println!("{}", s.clone().to_string().unwrap());
            }else{
                println!("lixo");
            }
        }
        Trans::None
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
    let assets_dir = app_root.join("examples/script/assets");

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
        .with(ScriptSystem::<PythonDriver>::new(python_scripts_path), "python_script_system", &[])
        .with(Processor::<ScriptAsset>::new(), "processor", &[]);

    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();
    Ok(())
}
