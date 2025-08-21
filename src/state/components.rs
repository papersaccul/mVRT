use crate::ColorTarget;
use bevy::prelude::*;

// Components
#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct Crosshair;

#[derive(Component)]
pub struct DirectionalLightEntity;

#[derive(Component)]
pub struct Arena;

#[derive(Component)]
pub struct ArenaWall;

#[derive(Component)]
pub struct SettingsUI;

#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct SettingsInfoText;

#[derive(Component)]
pub struct StartCenterText;

#[derive(Component)]
pub struct FpsText;

// Settings UI button markers
#[derive(Component)]
pub struct BtnArenaColor;
#[derive(Component)]
pub struct BtnTargetColor;
#[derive(Component)]
pub struct BtnStartGame;
#[derive(Component)]
pub struct BtnBackMenu;
#[derive(Component)]
pub struct DiscordLink;

// Input field components
#[derive(Component)]
pub struct DpiInput;
#[derive(Component)]
pub struct Cm360Input;
#[derive(Component)]
pub struct ArenaHueSlider;
#[derive(Component)]
pub struct ArenaSaturationSlider;
#[derive(Component)]
pub struct ArenaLightnessSlider;

#[derive(Component)]
pub struct TargetHueSlider;
#[derive(Component)]
pub struct TargetSaturationSlider;
#[derive(Component)]
pub struct TargetLightnessSlider;

// Slider fill nodes
#[derive(Component)]
pub struct ArenaHueFill;
#[derive(Component)]
pub struct ArenaSaturationFill;
#[derive(Component)]
pub struct ArenaLightnessFill;

#[derive(Component)]
pub struct TargetHueFill;
#[derive(Component)]
pub struct TargetSaturationFill;
#[derive(Component)]
pub struct TargetLightnessFill;

// Компоненты для цветовых пикеров
#[derive(Component)]
pub struct ArenaColorPicker;

#[derive(Component)]
pub struct TargetColorPicker;

// Top overlay UI components
#[derive(Component)]
pub struct TopLeftEscHint;

#[derive(Component)]
pub struct FullscreenToggle;

// Input editing
#[derive(Component)]
pub struct DpiEditing;
#[derive(Component)]
pub struct CmEditing;
#[derive(Component)]
pub struct DpiBuffer(pub String);
#[derive(Component)]
pub struct CmBuffer(pub String);
#[derive(Component)]
pub struct FovEditing;
#[derive(Component)]
pub struct FovBuffer(pub String);
#[derive(Component)]
pub struct FovInput;

// Fresnel UI components
#[derive(Component)]
pub struct FresnelEnabledCheckbox;
#[derive(Component)]
pub struct FresnelIntensityInput;
#[derive(Component)]
pub struct FresnelPowerInput;
#[derive(Component)]
pub struct FresnelColorPicker;
#[derive(Component)]
pub struct FresnelIntensityBuffer(pub String);
#[derive(Component)]
pub struct FresnelPowerBuffer(pub String);
#[derive(Component)]
pub struct FresnelIntensityEditing;
#[derive(Component)]
pub struct FresnelPowerEditing;

// Lighting settings components
#[derive(Component)]
pub struct DirectionalLightInput;

#[derive(Component)]
pub struct DirectionalLightBuffer(pub String);

#[derive(Component)]
pub struct AmbientLightInput;

#[derive(Component)]
pub struct AmbientLightBuffer(pub String);

#[derive(Component)]
pub struct DirectionalLightEditing;

#[derive(Component)]
pub struct AmbientLightEditing;

// Text update markers
#[derive(Component)]
pub struct DpiText;
#[derive(Component)]
pub struct CmText;
#[derive(Component)]
pub struct FovText;
#[derive(Component)]
pub struct GameInfoText;

#[derive(Component)]
pub struct ColorPickerWindow {
    pub target_type: ColorTarget,
}

#[derive(Component)]
pub struct ColorPickerOverlay;

#[derive(Component, Default)]
pub struct HueBar;

#[derive(Component, Default)]
pub struct SaturationBar;

#[derive(Component, Default)]
pub struct BrightnessBar;

#[derive(Component)]
pub struct HueGradient;

#[derive(Component)]
pub struct SaturationSegment {
    pub index: u8,
    pub count: u8,
}

#[derive(Component)]
pub struct BrightnessSegment {
    pub index: u8,
    pub count: u8,
}

#[derive(Component)]
pub struct HueHandle;

#[derive(Component)]
pub struct SaturationHandle;

#[derive(Component)]
pub struct BrightnessHandle;

#[derive(Component)]
pub struct ColorDisplay;

#[derive(Component)]
pub struct CloseColorPicker;

#[derive(Component)]
pub struct ApplyColorPicker;
