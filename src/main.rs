use bevy::{prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::{
    na::{Isometry2, Vector2},
    prelude::*,
};
use rand::Rng;

#[derive(Debug)]
struct Player;
struct RespawnFloor;

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
        .add_startup_system(build_level.system())
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
    /* Create the ground. */
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(2.03, 0.1),
        position: ColliderPosition(Isometry2::new(
            Vector2::new(-2.95, 0.5),
            0.0,
        )),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert_bundle(SpriteBundle {
            material: materials.add(Color::NONE.into()),
            sprite: Sprite::new(Vec2::new(106.0, 5.0)),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    /* Create the ground #2. */
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(2.03, 0.1),
        position: ColliderPosition(Isometry2::new(
            Vector2::new(2.8, 2.65),
            0.0,
        )),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert_bundle(SpriteBundle {
            material: materials.add(Color::NONE.into()),
            sprite: Sprite::new(Vec2::new(106.0, 5.0)),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

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
        shape: ColliderShape::cuboid(0.4, 0.4),
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
            transform: Transform::from_scale(Vec3::splat(
                0.05,
            )),
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
        println!(
            "Ball altitude: {}",
            rb_pos.position.translation.vector.y
        );
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
            Vec2::new(0.0, 5.0).into(),
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
fn build_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
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
        asset_server.load("super-corgo-five.ldtk");

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

    // let texture_handle =
    //     asset_server.load("tiles_packed.png");
    // let material_handle = materials
    //     .add(ColorMaterial::texture(texture_handle));

    // // Create map entity and component:
    // let map_entity = commands.spawn().id();
    // let mut map = Map::new(0u16, map_entity);

    // // Creates a new layer builder with a layer entity.
    // let (mut layer_builder, _) = LayerBuilder::new(
    //     &mut commands,
    //     LayerSettings::new(
    //         UVec2::new(2, 2),
    //         UVec2::new(8, 8),
    //         Vec2::new(18.0, 18.0),
    //         Vec2::new(360.0, 162.0),
    //     ),
    //     0u16,
    //     0u16,
    // );
    // chunk(&mut layer_builder);
    // // layer_builder.set_all(TileBundle::default());

    // // Builds the layer.
    // // Note: Once this is called you can no longer edit the layer until a hard sync in bevy.
    // let layer_entity = map_query.build_layer(
    //     &mut commands,
    //     layer_builder,
    //     material_handle.clone(),
    // );

    // // Required to keep track of layers for a map internally.
    // map.add_layer(&mut commands, 0u16, layer_entity);

    // // Spawn Map
    // // Required in order to use map_query to retrieve layers/tiles.
    // commands
    //     .entity(map_entity)
    //     .insert(map.clone())
    //     .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
    //     .insert(GlobalTransform::default());

    // // Create map entity and component:
    // let map_entity = commands.spawn().id();
    // let mut map = Map::new(0u16, map_entity);

    // // Creates a new layer builder with a layer entity.
    // let (mut layer_builder, _) = LayerBuilder::new(
    //     &mut commands,
    //     LayerSettings::new(
    //         UVec2::new(2, 2),
    //         UVec2::new(8, 8),
    //         Vec2::new(18.0, 18.0),
    //         Vec2::new(360.0, 162.0),
    //     ),
    //     0u16,
    //     0u16,
    // );

    // layer_builder.set_all(TileBundle {
    //     tile: Tile {
    //         texture_index: 5,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });

    // // Builds the layer.
    // // Note: Once this is called you can no longer edit the layer until a hard sync in bevy.
    // let layer_entity = map_query.build_layer(
    //     &mut commands,
    //     layer_builder,
    //     material_handle,
    // );

    // // Required to keep track of layers for a map internally.
    // map.add_layer(&mut commands, 0u16, layer_entity);

    // // Spawn Map
    // // Required in order to use map_query to retrieve layers/tiles.
    // commands
    //     .entity(map_entity)
    //     .insert(map.clone())
    //     .insert(Transform::from_xyz(
    //         2.0 * 18.0 * 8.0 - 128.0,
    //         -128.0,
    //         0.0,
    //     ))
    //     .insert(GlobalTransform::default());
}
