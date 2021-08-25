use std::collections::HashSet;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

mod helpers;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let layer_settings = LayerSettings::new(
        UVec2::new(10, 10),
        UVec2::new(64, 64),
        Vec2::new(16.0, 16.0),
        Vec2::new(96.0, 256.0),
    );

    let center = layer_settings.get_pixel_center();

    // Chunk sizes of 64x64 seem optimal for meshing updates.
    let (mut layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 0u16, None);
    map.add_layer(&mut commands, 0u16, layer_entity);

    layer_builder.for_each_tiles_mut(|tile_entity, tile_data| {
        // True here refers to tile visibility.
        *tile_data = Some(TileBundle::default());
        // Tile entity might not exist at this point so you'll need to create it.
        if tile_entity.is_none() {
            *tile_entity = Some(commands.spawn().id());
        }
        commands.entity(tile_entity.unwrap()).insert(LastUpdate::default());
    });

    map_query.build_layer(&mut commands, layer_builder, material_handle);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-center.x, -center.y, 0.0))
        .insert(GlobalTransform::default());
}

#[derive(Default)]
struct LastUpdate {
    value: f64,
}

// In this example it's better not to use the default `MapQuery` SystemParam as
// it's faster to do it this way:
fn random(
    time: ResMut<Time>,
    mut query: Query<(&mut Tile, &TileParent, &mut LastUpdate)>,
    mut chunk_query: Query<&mut Chunk>,
) {
    let current_time = time.seconds_since_startup();
    let mut random = thread_rng();
    let mut chunks = HashSet::new();
    for (mut tile, tile_parent, mut last_update) in query.iter_mut() {
        if (current_time - last_update.value) > 0.05 {
            tile.texture_index = random.gen_range(0..6);
            last_update.value = current_time;
            chunks.insert(tile_parent.chunk);
        }
    }

    for chunk_entity in chunks.drain() {
        if let Ok(mut chunk) = chunk_query.get_mut(chunk_entity) {
            chunk.needs_remesh = true;
        }
    }
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Warn)
        .init();

    App::build()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Random Map Example"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup.system())
        .add_system(random.system())
        .add_system(helpers::camera::movement.system())
        .add_system(helpers::texture::set_texture_filters_to_nearest.system())
        .run();
}
