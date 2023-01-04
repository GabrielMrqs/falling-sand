use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    sprite::collide_aabb::collide,
};

use crate::{HEIGHT, WIDTH};

pub struct ParticlePlugin;

#[derive(Component)]
pub struct Particle {
    size: f32,
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_camera)
            .add_system(spawn)
            .add_system(movement);
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn(
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        for event in cursor_moved_events.iter() {
            let x = event.position.x - WIDTH / 2.;
            let y = event.position.y - HEIGHT / 2.;
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("sand.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(5., 5.)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.),
                    ..default()
                },
                Particle { size: 5. },
            ));
        }
    }
}

fn movement(time: Res<Time>, mut particles: Query<(&mut Particle, &mut Transform)>) {
    let mut a = particles.iter();
    while let Some((particle, mut transform)) = a.next() {
        for (a, b) in particles. {
            let y = transform.translation.y;
            let x = transform.translation.x;
    
            if y >= (-HEIGHT / 2.) + particle.size {
                transform.translation.y -= 5.0;
            }
        }
    }
}

fn collision_check(
    target_particle_pos: Vec3,
    particles: &Query<(&mut Particle, &mut Transform)>,
) -> bool {
    for (_, transform) in particles.iter() {
        let collision = collide(
            target_particle_pos,
            Vec2::splat(5.0),
            transform.translation,
            Vec2::splat(1.0),
        );
        if collision.is_some() {
            return false;
        }
    }
    return true;
}
