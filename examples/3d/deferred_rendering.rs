//! Loads and renders a glTF file as a scene.

use std::f32::consts::*;

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use bevy_internal::{
    core_pipeline::{
        fxaa::Fxaa,
        prepass::{DeferredPrepass, DepthPrepass, MotionVectorPrepass, NormalPrepass},
    },
    pbr::{DefaultOpaqueRendererMethod, NotShadowCaster, OpaqueRendererMethod},
    render::render_resource::TextureFormat,
};

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(DefaultOpaqueRendererMethod(OpaqueRendererMethod::Deferred))
        .insert_resource(ClearColor(Color::rgb_linear(0.05, 0.05, 0.05)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .insert_resource(Normal(None))
        .insert_resource(Pause(true))
        .add_systems(Startup, (setup, setup_parallax))
        .add_systems(
            Update,
            (animate_light_direction, switch_mode, spin, update_normal),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                //hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.7, 0.7, 1.0)
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.05, 0.05, 0.05, 1.0),
            falloff: FogFalloff::Linear {
                start: 1.0,
                end: 8.0,
            },
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        },
        //NormalPrepass,
        DepthPrepass,
        MotionVectorPrepass,
        DeferredPrepass,
        Fxaa::default(),
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 3,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, -FRAC_PI_4)),
        ..default()
    });

    // FlightHelmet
    let helmet_scene = asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0");

    commands.spawn(SceneBundle {
        scene: helmet_scene.clone(),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: helmet_scene,
        transform: Transform::from_xyz(-3.0, 0.0, -3.0),
        ..default()
    });

    let mut forward_mat: StandardMaterial = Color::rgb(0.1, 0.2, 0.1).into();
    forward_mat.opaque_render_method = Some(OpaqueRendererMethod::Forward);
    let forward_mat_h = materials.add(forward_mat);

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: forward_mat_h.clone(),
        ..default()
    });

    let cube_h = meshes.add(Mesh::from(shape::Cube { size: 0.1 }));
    let sphere_h = meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.125,
        sectors: 128,
        stacks: 128,
    }));

    // cubes
    commands.spawn(PbrBundle {
        mesh: cube_h.clone(),
        material: forward_mat_h.clone(),
        transform: Transform::from_xyz(-0.3, 0.5, -0.2),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: cube_h.clone(),
        material: forward_mat_h,
        transform: Transform::from_xyz(0.2, 0.5, 0.2),
        ..default()
    });

    let sphere_color = Color::rgb(10.0, 4.0, 1.0);
    let sphere_pos = Transform::from_xyz(0.4, 0.5, -0.8);
    // emissive sphere
    let mut unlit_mat: StandardMaterial = sphere_color.into();
    unlit_mat.unlit = true;
    commands.spawn((
        PbrBundle {
            mesh: sphere_h.clone(),
            material: materials.add(unlit_mat),
            transform: sphere_pos,
            ..default()
        },
        NotShadowCaster,
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1.0,
            radius: 0.125,
            shadows_enabled: true,
            color: sphere_color,
            ..default()
        },
        transform: sphere_pos,
        ..default()
    });

    // spheres
    for i in 0..6 {
        let j = i % 3;
        let s_val = if i < 3 { 0.0 } else { 0.2 };
        let material = if j == 0 {
            materials.add(StandardMaterial {
                base_color: Color::rgb(s_val, s_val, 1.0),
                perceptual_roughness: 0.089,
                metallic: 0.0,
                ..default()
            })
        } else if j == 1 {
            materials.add(StandardMaterial {
                base_color: Color::rgb(s_val, 1.0, s_val),
                perceptual_roughness: 0.089,
                metallic: 0.0,
                ..default()
            })
        } else {
            materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, s_val, s_val),
                perceptual_roughness: 0.089,
                metallic: 0.0,
                ..default()
            })
        };
        commands.spawn(PbrBundle {
            mesh: sphere_h.clone(),
            material,
            transform: Transform::from_xyz(
                j as f32 * 0.25 + if i < 3 { -0.15 } else { 0.15 } - 0.4,
                0.125,
                -j as f32 * 0.25 + if i < 3 { -0.15 } else { 0.15 } + 0.4,
            ),
            ..default()
        });
    }
}

#[derive(Resource)]
struct Pause(bool);

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
    pause: Res<Pause>,
) {
    if pause.0 {
        return;
    }
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}

fn switch_mode(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut default_opaque_renderer_method: ResMut<DefaultOpaqueRendererMethod>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cameras: Query<Entity, With<Camera>>,
    mut pause: ResMut<Pause>,
) {
    if input.just_pressed(KeyCode::Space) {
        pause.0 = !pause.0;
    }

    if input.just_pressed(KeyCode::D) {
        default_opaque_renderer_method.0 = OpaqueRendererMethod::Deferred;
        println!("DefaultOpaqueRendererMethod: Deferred");
        for _ in materials.iter_mut() {}
        for camera in &cameras {
            commands.entity(camera).remove::<NormalPrepass>();
            commands.entity(camera).insert(DepthPrepass);
            commands.entity(camera).insert(MotionVectorPrepass);
            commands.entity(camera).insert(DeferredPrepass);
        }
    }
    if input.just_pressed(KeyCode::F) {
        default_opaque_renderer_method.0 = OpaqueRendererMethod::Forward;
        println!("DefaultOpaqueRendererMethod: Forward");
        for _ in materials.iter_mut() {}
        for camera in &cameras {
            commands.entity(camera).remove::<NormalPrepass>();
            commands.entity(camera).remove::<DepthPrepass>();
            commands.entity(camera).remove::<MotionVectorPrepass>();
            commands.entity(camera).remove::<DeferredPrepass>();
        }
    }
    if input.just_pressed(KeyCode::P) {
        default_opaque_renderer_method.0 = OpaqueRendererMethod::Forward;
        println!("DefaultOpaqueRendererMethod: Forward + Prepass");
        for _ in materials.iter_mut() {}
        for camera in &cameras {
            commands.entity(camera).insert(NormalPrepass);
            commands.entity(camera).insert(DepthPrepass);
            commands.entity(camera).insert(MotionVectorPrepass);
            commands.entity(camera).insert(DeferredPrepass);
        }
    }
}

fn setup_parallax(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut normal: ResMut<Normal>,
    asset_server: Res<AssetServer>,
) {
    // The normal map. Note that to generate it in the GIMP image editor, you should
    // open the depth map, and do Filters → Generic → Normal Map
    // You should enable the "flip X" checkbox.
    let normal_handle = asset_server.load("textures/parallax_example/cube_normal.png");
    normal.0 = Some(normal_handle);

    let mut cube: Mesh = shape::Cube { size: 0.15 }.into();

    // NOTE: for normal maps and depth maps to work, the mesh
    // needs tangents generated.
    cube.generate_tangents().unwrap();

    let parallax_material = materials.add(StandardMaterial {
        perceptual_roughness: 0.4,
        base_color_texture: Some(asset_server.load("textures/parallax_example/cube_color.png")),
        normal_map_texture: normal.0.clone(),
        // The depth map is a greyscale texture where black is the highest level and
        // white the lowest.
        depth_map: Some(asset_server.load("textures/parallax_example/cube_depth.png")),
        parallax_depth_scale: 0.09,
        parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 4 },
        max_parallax_layer_count: 5.0f32.exp2(),
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(cube),
            material: parallax_material.clone(),
            transform: Transform::from_xyz(0.4, 0.2, -0.8),
            ..default()
        },
        Spin { speed: 0.3 },
    ));
}

/// Store handle of the normal to later modify its format in [`update_normal`].
#[derive(Resource)]
struct Normal(Option<Handle<Image>>);

/// See parallax_mapping.rs for reasoning
fn update_normal(
    mut already_ran: Local<bool>,
    mut images: ResMut<Assets<Image>>,
    normal: Res<Normal>,
) {
    if *already_ran {
        return;
    }
    if let Some(normal) = normal.0.as_ref() {
        if let Some(image) = images.get_mut(normal) {
            image.texture_descriptor.format = TextureFormat::Rgba8Unorm;
            *already_ran = true;
        }
    }
}

#[derive(Component)]
struct Spin {
    speed: f32,
}

fn spin(time: Res<Time>, mut query: Query<(&mut Transform, &Spin)>, pause: Res<Pause>) {
    if pause.0 {
        return;
    }
    for (mut transform, spin) in query.iter_mut() {
        transform.rotate_local_y(spin.speed * time.delta_seconds());
        transform.rotate_local_x(spin.speed * time.delta_seconds());
        transform.rotate_local_z(-spin.speed * time.delta_seconds());
    }
}
