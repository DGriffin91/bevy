//! This example compares Forward, Forward + Prepass, and Deferred rendering.

use std::f32::consts::*;

use bevy::{
    core_pipeline::{
        fxaa::Fxaa,
        prepass::{DeferredPrepass, DepthPrepass, MotionVectorPrepass, NormalPrepass},
    },
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    pbr::{DefaultOpaqueRendererMethod, NotShadowCaster, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::TextureFormat,
};
use bevy_internal::window::PresentMode;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(DefaultOpaqueRendererMethod(OpaqueRendererMethod::Deferred))
        .insert_resource(ClearColor(Color::rgb_linear(0.2, 0.2, 0.2)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                //hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(-10.5, 1.7, -1.0)
                .looking_at(Vec3::new(0.0, 3.5, 0.0), Vec3::Y),
            ..default()
        },
        DepthPrepass,
        //MotionVectorPrepass,
        DeferredPrepass,
        //Fxaa::default(),
    ));

    // FlightHelmet
    //let helmet_scene = asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0");
    let helmet_scene = asset_server.load("H:/dev/programming/rust/bevy/GI/contact_shadows/bevy_mod_standard_material/assets/main_sponza/NewSponza_Main_glTF_002_no_decals.gltf#Scene0");

    commands.spawn(SceneBundle {
        scene: helmet_scene.clone(),
        ..default()
    });

    commands.spawn(SceneBundle {
        transform: Transform::from_xyz(-12.5, -0.5, -1.0),
        scene: helmet_scene.clone(),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });
}
