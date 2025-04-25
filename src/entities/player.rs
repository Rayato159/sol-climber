use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
// use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Component)]
pub struct Player {
    pub address: String,
}

impl Player {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    // pub fn get_address(&self) -> Pubkey {
    //     Pubkey::from_str_const(self.address.as_str())
    // }
}

#[derive(Debug, Component)]
pub struct PlayerPhysics {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub acceleration: f32,
    pub jump_force: f32,
    pub player_height: f32,
    pub player_width: f32,
    pub player_depth: f32,
}

impl Default for PlayerPhysics {
    fn default() -> Self {
        Self {
            walk_speed: 2.0,
            run_speed: 4.0,
            acceleration: 0.5,
            jump_force: 3.5,
            player_height: 1.66,
            player_width: 0.6,
            player_depth: 0.55,
        }
    }
}

#[derive(Debug, Component)]
pub struct PlayerGroundSensor(pub bool);

#[derive(Debug, Component, Default)]
pub struct PlayerMoveDirection(pub Option<Vec3>);

pub fn spawn_player(asset_server: Res<AssetServer>, mut commands: Commands) {
    let player_physics = PlayerPhysics::default();

    let init_pos = Vec3::new(0.0, 0.0, 0.0);

    commands.spawn((
        Name::new("Player"),
        SceneRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("characters/MainCharacter.glb#Scene0")),
        ),
        RigidBody::Dynamic,
        Collider::cuboid(
            player_physics.player_width / 2.0,
            player_physics.player_height / 2.0,
            player_physics.player_depth / 2.0,
        ),
        GravityScale(1.0),
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        ActiveEvents::COLLISION_EVENTS,
        Transform::from_translation(init_pos),
        PlayerGroundSensor(true),
        Player::new("E3YvQn4wk6JzyGY1uZMyzKCfu8ctM3kCu8Nk6KFZu8eM"),
        player_physics,
        PlayerMoveDirection::default(),
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut PlayerMoveDirection, &PlayerPhysics), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let (mut velocity, mut move_dir, player_physics) = query.single_mut();
    let camera = camera_query.single();

    let mut direction = Vec3::ZERO;

    let forward = camera.forward().xz().normalize();
    let right = camera.right().xz().normalize();

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec3::new(forward.x, 0.0, forward.y);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= Vec3::new(forward.x, 0.0, forward.y);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec3::new(right.x, 0.0, right.y);
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= Vec3::new(right.x, 0.0, right.y);
    }

    if direction.length_squared() > 0.0 {
        let dir = direction.normalize();

        let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
            player_physics.run_speed
        } else {
            player_physics.walk_speed
        };

        let movement = dir * speed;

        velocity.linvel.x = movement.x;
        velocity.linvel.z = movement.z;

        move_dir.0 = Some(dir);
    } else {
        velocity.linvel.x = 0.0;
        velocity.linvel.z = 0.0;
        move_dir.0 = None;
    }
}

pub fn player_rotation(mut query: Query<(&mut Transform, &PlayerMoveDirection), With<Player>>) {
    for (mut transform, move_dir) in query.iter_mut() {
        if let Some(dir) = move_dir.0 {
            let target_rot = Quat::from_rotation_arc(Vec3::Z, dir);
            transform.rotation = transform.rotation.slerp(target_rot, 0.2);
        }
    }
}

pub fn player_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut PlayerGroundSensor, &PlayerPhysics), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let (mut velocity, sensor, physics) = player_query.single_mut();
    let camera = camera_query.single();

    if keyboard_input.just_pressed(KeyCode::Space) && sensor.0 {
        velocity.linvel.y = physics.jump_force;

        let forward = camera.forward().xz().normalize();
        let impulse = Vec3::new(forward.x, 0.0, forward.y) * physics.acceleration;

        velocity.linvel.x += impulse.x;
        velocity.linvel.z += impulse.z;
    }
}

pub fn player_ground_check(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(&mut PlayerGroundSensor, Entity), With<Player>>,
) {
    let (mut ground_sensor, player_entity) = player_query.single_mut();

    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                if *e1 == player_entity || *e2 == player_entity {
                    ground_sensor.0 = true;
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if *e1 == player_entity || *e2 == player_entity {
                    ground_sensor.0 = false;
                }
            }
        }
    }
}
