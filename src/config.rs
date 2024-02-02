use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

#[derive(Resource, Reflect)]
pub struct Config {
    pub mouse_sensibility: f32,
    pub movement_speed: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mouse_sensibility: 0.001,
            movement_speed: 20.,
        }
    }
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Config>().init_resource::<Config>();

        #[cfg(debug_assertions)]
        app.add_plugins(ResourceInspectorPlugin::<Config>::default());
    }
}
