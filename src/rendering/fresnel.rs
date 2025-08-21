use bevy::pbr::{MaterialExtension, StandardMaterial};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct FresnelMaterial {
    /// Fresnel data: [r, g, b, intensity]
    #[uniform(100)]
    pub fresnel_color: Vec4,

    /// Fresnel params: [power, enabled, 0, 0]
    #[uniform(100)]
    pub fresnel_params: Vec4, // x: power, y: enabled, z: unused, w: unused
}

impl Default for FresnelMaterial {
    fn default() -> Self {
        Self {
            fresnel_color: Vec4::new(0.0, 1.0, 1.0, 1.0), // r, g, b, intensity
            fresnel_params: Vec4::new(2.0, 0.0, 0.0, 0.0), // power, enabled, unused, unused
        }
    }
}

impl MaterialExtension for FresnelMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fresnel.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/fresnel.wgsl".into()
    }
}

pub type ExtendedMaterial = bevy::pbr::ExtendedMaterial<StandardMaterial, FresnelMaterial>;

pub fn setup_fresnel_shader(app: &mut App) {
    app.add_plugins(MaterialPlugin::<ExtendedMaterial>::default());
}
