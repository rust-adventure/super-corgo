use bevy::{prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::{
    na::{Isometry2, Vector2},
    prelude::*,
};
use itertools::Itertools;

#[derive(Debug)]
struct Player;
struct RespawnFloor;
struct ProcessedTile;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Super Corgo Run!".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(LdtkPlugin)
        .insert_resource(ClearColor(
            Color::hex("DFF6F5").unwrap(),
        ))
        .add_plugin(
            RapierPhysicsPlugin::<NoUserData>::default(),
        )
        .add_plugin(RapierRenderPlugin)
        .insert_resource(RapierConfiguration {
            scale: 25.0,
            ..Default::default()
        })
        .add_startup_system(setup_physics.system())
        .add_startup_system_to_stage(
            StartupStage::PreStartup,
            insert_ldtk.system(),
        )
        .add_system(setup_colliders.system())
        .add_system(print_ball_altitude.system())
        .add_system(control.system())
        .add_system(animate_sprite_system.system())
        .add_system(side_scroll.system())
        .add_system(respawn.system())
        .run();
}

fn setup_physics(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // /* Create the ground. */
    // let collider = ColliderBundle {
    //     shape: ColliderShape::cuboid(2.03, 0.1),
    //     position: ColliderPosition(Isometry2::new(
    //         Vector2::new(-2.95, 0.5),
    //         0.0,
    //     )),
    //     ..Default::default()
    // };
    // commands
    //     .spawn_bundle(collider)
    //     .insert_bundle(SpriteBundle {
    //         material: materials.add(Color::NONE.into()),
    //         sprite: Sprite::new(Vec2::new(106.0, 5.0)),
    //         ..Default::default()
    //     })
    //     .insert(ColliderPositionSync::Discrete);

    // /* Create the ground #2. */
    // let collider = ColliderBundle {
    //     shape: ColliderShape::cuboid(2.03, 0.1),
    //     position: ColliderPosition(Isometry2::new(
    //         Vector2::new(2.8, 2.65),
    //         0.0,
    //     )),
    //     ..Default::default()
    // };
    // commands
    //     .spawn_bundle(collider)
    //     .insert_bundle(SpriteBundle {
    //         material: materials.add(Color::NONE.into()),
    //         sprite: Sprite::new(Vec2::new(106.0, 5.0)),
    //         ..Default::default()
    //     })
    //     .insert(ColliderPositionSync::Discrete);

    /* Create the respawn floor */
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(1000.0, 0.1),
        position: ColliderPosition(Isometry2::new(
            Vector2::new(0.0, -20.0),
            0.0,
        )),
        flags: (ActiveEvents::CONTACT_EVENTS
            | ActiveEvents::INTERSECTION_EVENTS)
            .into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert_bundle(SpriteBundle {
            material: materials.add(Color::RED.into()),
            sprite: Sprite::new(Vec2::new(50000.0, 5.0)),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(RespawnFloor);

    spawn_player(
        &mut commands,
        asset_server,
        &mut texture_atlases,
    );
}

fn spawn_player(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(-4.0, 10.0).into(),
        mass_properties: RigidBodyMassProps {
            flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
            ..Default::default()
        },
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.3, 0.3),
        material: ColliderMaterial {
            restitution: 0.0,
            friction: 0.0,
            ..Default::default()
        },
        ..Default::default()
    };

    let texture_handle =
        asset_server.load("party-corgi-sprites.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(1724.0 / 4.0, 1385.0 / 3.0),
        4,
        3,
    );
    let texture_atlas_handle =
        texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::splat(0.03),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                flip_x: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.2, true))
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Player);
}

fn print_ball_altitude(
    positions: Query<&RigidBodyPosition>,
) {
    for rb_pos in positions.iter() {
        // println!(
        //     "Ball altitude: {}",
        //     rb_pos.position.translation.vector.y
        // );
    }
}

fn control(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(
        &Player,
        &mut RigidBodyVelocity,
        &RigidBodyMassProps,
        &mut RigidBodyForces,
        &mut TextureAtlasSprite,
    )>,
) {
    let mut player = player
        .single_mut()
        .expect("always expect a player");
    if keyboard_input.just_pressed(KeyCode::Up) {
        player.1.apply_impulse(
            player.2,
            Vec2::new(0.0, 2.5).into(),
        );
    };
    if keyboard_input.pressed(KeyCode::Left) {
        player.1.apply_impulse(
            player.2,
            Vec2::new(-0.1, 0.0).into(),
        );
        player.3.force = Vec2::new(-0.5, 0.0).into();
        player.4.flip_x = false;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        player.1.apply_impulse(
            player.2,
            Vec2::new(0.1, 0.0).into(),
        );
        player.3.force = Vec2::new(0.5, 0.0).into();
        player.4.flip_x = true;
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    mut velocities: Query<&mut RigidBodyVelocity>,
) {
    for (
        entity,
        mut timer,
        mut sprite,
        texture_atlas_handle,
    ) in query.iter_mut()
    {
        timer.tick(time.delta());
        if let Ok(true) =
            velocities.get_mut(entity).map(|velocity| {
                let x_speed = velocity.linvel.data.0[0][0];
                x_speed > 1.0 || x_speed < -1.0
            })
        {
            if timer.finished() {
                let texture_atlas = texture_atlases
                    .get(texture_atlas_handle)
                    .unwrap();

                sprite.index = {
                    if sprite.index == 1
                        || sprite.index == 0
                    {
                        texture_atlas.textures.len() as u32
                            - 1
                    } else {
                        sprite.index - 1
                    }
                }
            }
        }
    }
}

fn side_scroll(
    camera: Query<Entity, With<Camera>>,
    player: Query<Entity, With<Player>>,
    mut transforms: Query<&mut Transform>,
) {
    if let Ok(player) = player.single() {
        let camera = camera
            .single()
            .expect("there to only be one camera ever");

        let player_transform: Transform = transforms
            .get_component::<Transform>(player)
            .expect("should exist")
            .clone();

        let mut camera_transform = transforms
            .get_mut(camera)
            .expect("should exist");

        if player_transform.translation.x < 0.0 {
            camera_transform.translation.x = 0.0;
        } else {
            camera_transform.translation.x =
                player_transform.translation.x;
        }
    }
}

fn respawn(
    narrow_phase: Res<NarrowPhase>,
    floor: Query<Entity, With<RespawnFloor>>,
    player: Query<Entity, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let entity1 = floor.single().unwrap();
    let entity2 = player.single().unwrap();

    /* Find the contact pair, if it exists, between two colliders. */
    if let Some(contact_pair) = narrow_phase
        .contact_pair(entity1.handle(), entity2.handle())
    {
        // The contact pair exists meaning that the broad-phase identified a potential contact.
        if contact_pair.has_any_active_contact {
            commands.entity(entity2).despawn_recursive();
            // The contact pair has active contacts, meaning that it
            // contains contacts for which contact forces were computed.

            // TODO: game.respawns += 1
            spawn_player(
                &mut commands,
                asset_server,
                &mut texture_atlases,
            );
        }
    }
}
fn insert_ldtk(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // mut map_query: MapQuery,
) {
    let camera_transform = Transform {
        scale: Vec3::new(0.5, 0.5, 1.0),
        ..Transform::from_xyz(0.0, 0.0, 1.0)
    };
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: camera_transform,
        ..OrthographicCameraBundle::new_2d()
    });

    let handle: Handle<LdtkMap> =
        asset_server.load("super-corgo.ldtk");

    let map_entity = commands.spawn().id();

    commands.entity(map_entity).insert_bundle(
        LdtkMapBundle {
            ldtk_map: handle,
            map: Map::new(0u16, map_entity),
            transform: Transform::from_xyz(
                -(0.5 * 1920.0),
                706.0 - (1080.0 / 2.0)
                    + (1080.0 / 2.0) / 2.0,
                0.0,
            ),
            ..Default::default()
        },
    );
}

// fn setup_colliders(
//     mut commands: Commands,
//     // mut layer_query: Query<(Entity, &mut Layer)>,
//     map_query: Query<(Entity, &Map)>,
//     // mut map_events: EventReader<AssetEvent<LdtkMap>>,
//     tiles: Query<(Entity, &Tile), Without<ProcessedTile>>,
//     layers: Query<(Entity, &Layer)>,
//     chunks: Query<(Entity, &Chunk)>,
//     // maps: Res<Assets<LdtkMap>>,
// ) {
//     // only expecting one map, but yolo because it might not
//     // exist yet
//     for map in map_query.iter() {
//         // check for layer 0u16 so that we only proceed if the ldtk
//         // map has been processed and layers have been inserted
//         if let Some(layer) = map.1.get_layer_entity(0u16) {
//             // dbg!(layer);
//             // for d in layers.iter() {
//             //     dbg!(d.0);
//             // }
//             let the_layer = layers.get(*layer).unwrap();
//             let map_size = the_layer.1.settings.map_size;
//             for chunk_number in (0..map_size.x)
//                 .cartesian_product(0..map_size.y)
//             {
//                 for chunk in
//                     the_layer.1.get_chunk(UVec2::new(
//                         chunk_number.0,
//                         chunk_number.1,
//                     ))
//                 {
//                     chunks
//                         .get(chunk)
//                         .unwrap()
//                         .1
//                         .for_each_tile_entity(
//                             |(pos, tile_entity)| {
//                                 if let Some(entity) =
//                                     tile_entity
//                                 {
//                                     if let Ok((
//                                         entity,
//                                         tile,
//                                     )) =
//                                         tiles.get(*entity)
//                                     {
//                                         // dbg!(tile);

//                                         // if tile is platform, insert collider
//                                     }
//                                 }
//                             },
//                         );
//                 }
//             }
//         }
//     }
//     // let mut changed_maps =
//     //     Vec::<Handle<LdtkMap>>::default();
// }

fn setup_colliders(
    mut commands: Commands,
    mut chunks: Query<
        (Entity, &mut Chunk),
        Without<ProcessedTile>,
    >,
    tiles: Query<&Tile>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, chunk) in chunks.iter_mut() {
        let chunk_size = chunk.settings.size;
        let chunk_pos = (
            -(0.5 * 1920.0)
                + chunk.settings.position.x as f32
                    * chunk_size.x as f32
                    * 18.0,
            // chunk_size.x as f32
            //     * chunk.settings.position.x as f32, //* 18.0,
            chunk_size.y as f32 * 18.0
                + chunk.settings.position.y as f32,
        );
        /// Chunk debug spawn
        // // dbg!(chunk.settings.position);
        // let num = if chunk.settings.position.y > 0 {
        //     chunk.settings.position.x * 20
        // } else {
        //     chunk.settings.position.x * 10
        // };
        // commands.spawn_bundle(SpriteBundle {
        //     material: materials.add(
        //         Color::rgb_u8(
        //             num as u8, num as u8, num as u8,
        //         )
        //         .into(),
        //     ),
        //     sprite: Sprite::new(Vec2::new(
        //         chunk_size.x as f32 * 18.0,
        //         chunk_size.y as f32 * 18.0,
        //         // chunk_size.x as f32 * 18.0,
        //         // chunk_size.y as f32 * 18.0,
        //     )),
        //     transform: Transform::from_xyz(
        //         chunk_pos.0,
        //         // chunk_pos.1,
        //         chunk_size.y as f32
        //             * chunk.settings.position.y as f32
        //             * 18.0
        //             + (706.0 - (1080.0 / 2.0)
        //                 + (1080.0 / 2.0) / 2.0)
        //             - chunk_size.y as f32 * 18.0,
        //         0.0,
        //     ),
        //     ..Default::default()
        // });
        // println!("### {:?} {:?}", entity, chunk_pos);
        // dbg!(chunk_pos);
        commands.entity(entity).insert(ProcessedTile);
        chunk.for_each_tile_entity(|(pos, tile)| {
            if let Some(tile_entity) = tile {
                if let Ok(tile) = tiles.get(*tile_entity) {
                    // println!("{}: {}", pos, tile.texture_index);
                    let px_pos = (
                        chunk_pos.0 + pos.x as f32 * 18.0,
                        chunk_pos.1 + pos.y as f32 * 18.0,
                    );
                    // println!("  at: {:?}", px_pos);
                    if [0,1,2,3,12,13,14,15,20,21,22,23].contains(&tile.texture_index) {
                        // let rigid_body = RigidBodyBundle {
                        //     // position: Vec2::new(px_pos.0, 0.0).into(),
                        //     body_type: RigidBodyType::Static,
                        //     mass_properties: RigidBodyMassProps {
                        //         flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                        //         ..Default::default()
                        //     },
                        //     ..Default::default()
                        // };
                        // let y_offset_px = 706.0 - (1080.0 / 2.0)
                        // + (1080.0 / 2.0) / 2.0;
                        dbg!(px_pos.1);
                        let collider = ColliderBundle {
                            shape: ColliderShape::cuboid(
                                0.36, 0.36,
                            ),
                            material: ColliderMaterial {
                                restitution: 0.0,
                                friction: 0.0,
                                ..Default::default()
                            },
                            position: ColliderPosition(
                                Isometry2::new(
                                    Vector2::new(
                                        px_pos.0 * 0.04 + 0.37,
                                        (0.72 * pos.y as f32) - (706.0/2.0 * 0.047) + 6.15//px_pos.1 
                                    ),
                                    0.0,
                                ),
                            ),
                            ..Default::default()
                        };
                        commands.entity(*tile_entity)
                       // .insert_bundle(rigid_body)
                            .insert_bundle(collider)
                            .insert_bundle(SpriteBundle {
                                material: materials.add(
                                    Color::NONE.into(),
                                ),
                                sprite: Sprite::new(
                                    Vec2::new(
                                        18.0, 18.0,
                                    ),
                                ),
                                // transform:
                                //     Transform::from_xyz(
                                //         px_pos.0, px_pos.1,
                                //         5.0,
                                //     ),
                                ..Default::default()
                            })
                            .insert(ColliderPositionSync::Discrete);
                    }
                };
            }
        })
    }
}
