use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
// use solana_sdk::pubkey::Pubkey;

const PLAYER_MODEL_PATH: &str = "characters/MainCharacter.glb";

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
            run_speed: 6.0,
            acceleration: 1.5,
            jump_force: 4.0,
            player_height: 1.66,
            player_width: 0.562503,
            player_depth: 0.75153,
        }
    }
}

#[derive(Debug, Component)]
pub struct PlayerGroundSensor(pub bool);

#[derive(Debug, Component, Default)]
pub struct PlayerRunningSensor(pub bool);

#[derive(Debug, Component, Default)]
pub struct PlayerMoveDirection(pub Option<Vec3>);

#[derive(Resource)]
pub struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph_handle: Handle<AnimationGraph>,
}

pub fn spawn_player(
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut commands: Commands,
) {
    let player_physics = PlayerPhysics::default();

    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(PLAYER_MODEL_PATH)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(PLAYER_MODEL_PATH)),
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(PLAYER_MODEL_PATH)),
    ]);

    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations: node_indices,
        graph_handle,
    });

    commands.spawn((
        Name::new("Player"),
        Transform::from_translation(Vec3::ZERO),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(PLAYER_MODEL_PATH))),
        RigidBody::Dynamic,
        Collider::cylinder(
            player_physics.player_height / 2.0,
            player_physics.player_width / 2.0,
        ),
        GravityScale(1.0),
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        ActiveEvents::COLLISION_EVENTS,
        PlayerGroundSensor(true),
        Player::new("E3YvQn4wk6JzyGY1uZMyzKCfu8ctM3kCu8Nk6KFZu8eM"),
        player_physics,
        PlayerMoveDirection::default(),
        PlayerRunningSensor::default(),
    ));
}

pub fn init_player_animation(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph_handle.clone()))
            .insert(transitions);
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (
            &mut Velocity,
            &mut PlayerMoveDirection,
            &PlayerPhysics,
            &mut PlayerRunningSensor,
            &PlayerGroundSensor,
        ),
        With<Player>,
    >,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
) {
    for (mut player_animation, mut animation_transitions) in &mut animation_players {
        let Some((&playing_animation_index, _)) = player_animation.playing_animations().next()
        else {
            continue;
        };

        let (
            mut velocity,
            mut move_dir,
            player_physics,
            mut player_running_sensor,
            player_ground_sensor,
        ) = player_query.single_mut();
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
            if playing_animation_index.index() != 3 && player_ground_sensor.0 {
                animation_transitions
                    .play(
                        &mut player_animation,
                        animations.animations[2],
                        Duration::from_secs_f32(0.1),
                    )
                    .repeat();
            }

            let dir = direction.normalize();

            let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
                if !player_running_sensor.0 {
                    player_running_sensor.0 = true;
                }
                player_physics.run_speed
            } else {
                if player_running_sensor.0 {
                    player_running_sensor.0 = false;
                }
                player_physics.walk_speed
            };

            if player_running_sensor.0 {
                let playing_animation = player_animation
                    .animation_mut(playing_animation_index)
                    .unwrap();
                playing_animation.set_speed(1.2);
            } else {
                let playing_animation = player_animation
                    .animation_mut(playing_animation_index)
                    .unwrap();
                playing_animation.set_speed(1.0);
            }

            let movement = dir * speed;

            velocity.linvel.x = movement.x;
            velocity.linvel.z = movement.z;

            move_dir.0 = Some(dir);

            return;
        } else {
            if playing_animation_index.index() != 1 && playing_animation_index.index() != 2 {
                animation_transitions
                    .play(
                        &mut player_animation,
                        animations.animations[0],
                        Duration::from_secs_f32(0.1),
                    )
                    .repeat();
            }

            velocity.linvel.x = 0.0;
            velocity.linvel.z = 0.0;
            move_dir.0 = None;
        }
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

pub fn player_jumping_animation(
    player_query: Query<&PlayerGroundSensor, With<Player>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
) {
    let ground_sensor = player_query.single();
    for (mut player_animation, mut animation_transitions) in &mut animation_players {
        let Some((&playing_animation_index, _)) = player_animation.playing_animations().next()
        else {
            continue;
        };

        if !ground_sensor.0 && playing_animation_index.index() != 2 {
            animation_transitions.play(
                &mut player_animation,
                animations.animations[1],
                Duration::from_secs_f32(0.1),
            );
        } else if ground_sensor.0 && playing_animation_index.index() == 2 {
            animation_transitions
                .play(
                    &mut player_animation,
                    animations.animations[0],
                    Duration::from_secs_f32(0.1),
                )
                .repeat();
        }
    }
}
