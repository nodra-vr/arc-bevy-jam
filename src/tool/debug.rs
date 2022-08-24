use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

#[cfg(debug_assertions)]
pub const ENABLE_INSPECTOR: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_INSPECTOR: bool = false;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default());
        }
        if ENABLE_INSPECTOR {
            app.insert_resource(WorldInspectorParams {
                enabled: false,
                ..Default::default()
            });
            app.add_plugin(WorldInspectorPlugin::new());
            app.add_system(toggle_world_inspector);
        }
    }
}

fn toggle_world_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled
    }
}