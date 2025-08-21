//#![windows_subsystem = "windows"]
use bevy::audio::{AudioPlugin, GlobalVolume, Volume};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;

mod config;
mod kernel;
mod rendering;
mod state;
mod user_interface;

use config::*;
use kernel::*;
use rendering::*;
use state::*;
use user_interface::*;

use config::config::*;
use config::settings::*;
use kernel::camera::*;
use kernel::game::*;
use kernel::target::*;
use kernel::utils::*;
use rendering::fresnel::*;
use user_interface::ui::*;
use user_interface::ui_components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Modern Reaction Test".into(),
                resolution: (1600.0, 900.0).into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_asset::<AudioSource>()
        .insert_resource(GlobalVolume::new(Volume::Linear(1.0)))
        .add_plugins(ConfigPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(MaterialPlugin::<ExtendedMaterial>::default())
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 1000.0))
        .init_state::<AppState>()
        .init_resource::<Settings>()
        .init_resource::<ReactionTest>()
        .init_resource::<GameState>()
        .init_resource::<InputFocus>()
        .init_resource::<FpsUiState>()
        .init_resource::<FresnelTracker>()
        // Loading
        .add_systems(Update, load_app.run_if(in_state(AppState::Loading)))
        .add_systems(
            Update,
            (
                settings_input_system.run_if(in_state(AppState::Settings)),
                game_input_system.run_if(in_state(AppState::Game)),
            ),
        )
        .add_systems(
            Update,
            (
                update_camera,
                update_ui_crosshair,
                update_game_ui,
                update_button_styles,
                update_arena_walls_color,
            )
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(Update, update_button_styles)
        .add_systems(Update, handle_color_picker_clicks)
        .add_systems(Update, handle_fullscreen_toggle)
        .add_systems(Update, handle_color_picker_overlay_clicks)
        .add_systems(Update, handle_color_picker_blocking)
        .add_systems(Update, block_disabled_interactions)
        .add_systems(
            Update,
            (
                handle_color_picker_buttons,
                handle_color_picker_sliders,
                update_color_picker_display,
                update_color_picker_handles,
                handle_fresnel_clicks,
            )
                .run_if(resource_exists::<Settings>),
        )
        .add_systems(Update, handle_color_picker_escape)
        .add_systems(Update, (update_fps_ui,))
        .add_systems(FixedUpdate, update_target.run_if(in_state(AppState::Game)))
        .add_systems(
            Update,
            (
                handle_mouse_lock,
                apply_theme_to_scene,
                apply_fov_to_camera,
                update_lighting,
            ),
        )
        .add_systems(
            Update,
            (
                settings_button_system.run_if(in_state(AppState::Settings)),
                update_settings_text.run_if(in_state(AppState::Settings)),
                settings_slider_system.run_if(in_state(AppState::Settings)),
            ),
        )
        .add_systems(
            Update,
            (
                dpi_input_box_system.run_if(in_state(AppState::Settings)),
                cm_input_box_system.run_if(in_state(AppState::Settings)),
                fov_input_box_system.run_if(in_state(AppState::Settings)),
                fresnel_intensity_input_box_system.run_if(in_state(AppState::Settings)),
                fresnel_power_input_box_system.run_if(in_state(AppState::Settings)),
                directional_light_input_box_system.run_if(in_state(AppState::Settings)),
                ambient_light_input_box_system.run_if(in_state(AppState::Settings)),
                clear_other_editing_states_system.run_if(in_state(AppState::Settings)),
                update_fresnel_ui.run_if(in_state(AppState::Settings)),
            ),
        )
        .add_systems(OnExit(AppState::Settings), update_fresnel_target_material)
        .add_systems(OnEnter(AppState::Settings), setup_settings_ui)
        .add_systems(OnEnter(AppState::Settings), restart_test_on_settings_enter)
        .add_systems(OnExit(AppState::Settings), cleanup_settings_ui)
        .add_systems(OnExit(AppState::Settings), restart_test_on_settings_exit)
        .add_systems(
            OnEnter(AppState::Game),
            (setup_game_scene, refresh_target_on_game_enter).chain(),
        )
        .add_systems(OnExit(AppState::Game), cleanup_game_scene)
        .run();
}

fn load_app(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Settings);
}
