use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

struct Player;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
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
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_physics(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        shape: ColliderShape::ball(50.0),
        material: ColliderMaterial {
            restitution: 0.0,
            ..Default::default()
        },
        ..Default::default()
    };
    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert_bundle(SpriteBundle {
            material: materials.add(Color::BLUE.into()),
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            ..Default::default()
        })
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
