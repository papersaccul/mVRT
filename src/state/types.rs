use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Settings,
    Game,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputField {
    Dpi,
    Cm,
    Fov,
    FresnelIntensity,
    FresnelPower,
    DirectionalLight,
    AmbientLight,
}

#[derive(Clone, Copy)]
pub enum ColorTarget {
    Arena,
    Target,
    Fresnel,
}

#[derive(Clone, Debug)]
pub struct DataPoint {
    pub time: f32,
    pub target_pos: Vec3,
    pub crosshair_dir: Vec3,
    pub target_x: f32,
    pub target_y: f32,
    pub camera_pos: Vec3,
    //pub is_direction_change: bool,
    pub crosshair_x: f32,
    pub crosshair_y: f32,
}

#[derive(Clone, Debug)]
pub struct DirectionChange {
    pub time: f32,
    pub target_new_direction: Vec2,
}
