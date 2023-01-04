use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

mod particle;
use particle::ParticlePlugin;

#[derive(Resource)]
struct BevyCounter {
    pub count: usize,
    pub color: Color,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Falling Sand".to_string(),
                width: WIDTH,
                height: HEIGHT,
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(ParticlePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .insert_resource(BevyCounter {
            count: 0,
            color: Color::WHITE,
        })
        .run();
}
