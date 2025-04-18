use bevy::prelude::*;
// use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Component)]
pub struct Player {
    pub address: String,
    pub speed: f32,
}

impl Player {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            speed: 1.5,
        }
    }

    // pub fn get_address(&self) -> Pubkey {
    //     Pubkey::from_str_const(self.address.as_str())
    // }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let init_pos = Vec3::new(0.0, 0.5, 0.0);

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 1.0, 0.5))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(init_pos),
        Player::new("E3YvQn4wk6JzyGY1uZMyzKCfu8ctM3kCu8Nk6KFZu8eM"),
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let (mut transform, player) = player_query.single_mut();
    let camera_transform = camera_query.single();

    let mut direction = Vec3::ZERO;

    let forward = camera_transform.forward().xz().normalize();
    let right = camera_transform.right().xz().normalize();

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
        let velocity = direction.normalize() * player.speed * time.delta_secs();
        transform.translation += velocity;
    }
}
