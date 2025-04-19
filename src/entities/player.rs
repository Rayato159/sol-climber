use bevy::prelude::*;

use super::GRAVITY;
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
pub struct PlayerCollider {
    pub height: f32,
    pub radius: f32,
}

impl Default for PlayerCollider {
    fn default() -> Self {
        Self {
            height: 1.0,
            radius: 0.5,
        }
    }
}

#[derive(Debug, Component)]
pub struct PlayerPhysics {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub acceleration: f32,
    pub velocity: Vec3,
    pub jump_force: f32,
    pub on_ground: bool,
}

impl Default for PlayerPhysics {
    fn default() -> Self {
        Self {
            walk_speed: 2.0,
            run_speed: 4.0,
            acceleration: 0.5,
            velocity: Vec3::ZERO,
            jump_force: 3.5,
            on_ground: true,
        }
    }
}

#[derive(Debug, Component, Default)]
pub struct PlayerMoveDirection(pub Option<Vec3>);

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_collider = PlayerCollider::default();
    let init_pos = Vec3::new(0.0, player_collider.height / 2., 0.0);

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, player_collider.height, 0.5))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(init_pos),
        Player::new("E3YvQn4wk6JzyGY1uZMyzKCfu8ctM3kCu8Nk6KFZu8eM"),
        PlayerCollider::default(),
        PlayerPhysics::default(),
        PlayerMoveDirection::default(),
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PlayerMoveDirection, &PlayerPhysics), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let (mut transform, mut move_dir, player_physics) = query.single_mut();
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

        transform.translation += dir * speed * time.delta_secs();
        move_dir.0 = Some(dir);
    } else {
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
    mut player_query: Query<&mut PlayerPhysics, (With<Player>, Without<Camera3d>)>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let mut player_physics = player_query.single_mut();
    let camera = camera_query.single();

    if keyboard_input.just_pressed(KeyCode::Space) && player_physics.on_ground {
        player_physics.velocity.y = player_physics.jump_force;
        player_physics.on_ground = false;

        let forward = camera.forward().xz().normalize();
        let impulse = Vec3::new(forward.x, 0.0, forward.y) * player_physics.acceleration;
        player_physics.velocity += impulse;
    }
}

pub fn player_fall(
    time: Res<Time>,
    mut player_query: Query<(&PlayerCollider, &mut Transform, &mut PlayerPhysics), With<Player>>,
) {
    let (collider, mut transform, mut physics) = player_query.single_mut();

    physics.velocity.y += GRAVITY * time.delta_secs();
    transform.translation += physics.velocity * time.delta_secs();

    physics.velocity.x *= 0.98;
    physics.velocity.z *= 0.98;

    let min_y = collider.height / 2.0;
    if transform.translation.y <= min_y {
        transform.translation.y = min_y;
        physics.velocity.y = 0.0;
        physics.on_ground = true;
    }
}
