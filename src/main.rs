use bevy::{prelude::*, render::camera::Camera};
use bevy_rapier2d::prelude::*;

#[derive(Debug)]
struct Player;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Super Corgo Run!".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(
            0.04, 0.04, 0.1,
        )))
        .add_plugin(
            RapierPhysicsPlugin::<NoUserData>::default(),
        )
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(setup_physics.system())
        .add_system(print_ball_altitude.system())
        .add_system(control.system())
        .add_system(animate_sprite_system.system())
        .add_system(side_scroll.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_physics(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    /* Create the ground. */
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(10000.0, 0.1),
        ..Default::default()
    };
    commands.spawn_bundle(collider).insert_bundle(
        SpriteBundle {
            material: materials.add(Color::WHITE.into()),
            sprite: Sprite::new(Vec2::new(1000.0, 5.0)),
            ..Default::default()
        },
    );

    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(0.0, 75.0).into(),
        mass_properties: RigidBodyMassProps {
            flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
            ..Default::default()
        },
        ..Default::default()
    };
    let collider = ColliderBundle {
        // shape: ColliderShape::ball(50.0),
        shape: ColliderShape::cuboid(50.0, 50.0),
        material: ColliderMaterial {
            restitution: 0.0,
            ..Default::default()
        },
        ..Default::default()
    };

    let texture_handle = asset_server
        .load("textures/party-corgi-sprites.png");
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
        // .insert_bundle(SpriteBundle {
        //     material: materials.add(Color::BLUE.into()),
        //     sprite: Sprite::new(Vec2::new(100.0, 100.0)),
        //     ..Default::default()
        // })
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(
                0.2,
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
    )>,
) {
    let mut player = player
        .single_mut()
        .expect("always expect a player");
    if keyboard_input.just_pressed(KeyCode::Up) {
        dbg!("apply impulse");
        player.1.apply_impulse(
            player.2,
            Vec2::new(1000.0, 200000.0).into(),
        );
    };
    if keyboard_input.pressed(KeyCode::Left) {
        player.1.apply_impulse(
            player.2,
            Vec2::new(-100000.0, 0.0).into(),
        );
        player.3.force = Vec2::new(-100000.0, 2.0).into();
    }
    if keyboard_input.pressed(KeyCode::Right) {
        player.1.apply_impulse(
            player.2,
            Vec2::new(100000.0, 0.0).into(),
        );
        player.3.force = Vec2::new(100000.0, 2.0).into();
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in
        query.iter_mut()
    {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases
                .get(texture_atlas_handle)
                .unwrap();
            sprite.index = ((sprite.index as usize + 1)
                % texture_atlas.textures.len())
                as u32;
        }
    }
}

fn side_scroll(
    camera: Query<Entity, With<Camera>>,
    player: Query<Entity, With<Player>>,
    mut transforms: Query<&mut Transform>,
) {
    if let Ok(player) = player.single() {
        dbg!("has player");
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

        camera_transform.translation.x =
            player_transform.translation.x;
    }
}
