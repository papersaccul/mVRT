#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    mesh_view_bindings::view,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct FresnelMaterial {
    fresnel_color: vec4<f32>, // r, g, b, intensity
    fresnel_params: vec4<f32>, // x: power, y: enabled, z: unused, w: unused
}

@group(2) @binding(100)
var<uniform> fresnel_material: FresnelMaterial;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // Generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // Alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // In deferred mode, we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // Apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // Calculate Fresnel effect if enabled
    if fresnel_material.fresnel_params.y > 0.5 {
        let world_normal = normalize(in.world_normal);
        let world_position = in.world_position.xyz;
        let view_position = view.world_position;

        // Calculate view direction from fragment to camera
        let view_dir = normalize(view_position - world_position);

        // Calculate fresnel factor
        let fresnel_dot = max(0.0, dot(world_normal, view_dir));
        let fresnel_factor = pow(1.0 - fresnel_dot, fresnel_material.fresnel_params.x);

        // Apply fresnel glow as additive color
        let fresnel_glow = fresnel_material.fresnel_color.rgb * fresnel_factor * fresnel_material.fresnel_color.w;
        out.color = vec4<f32>(out.color.rgb + fresnel_glow, out.color.a);
    }

    // Apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    return out;
}
