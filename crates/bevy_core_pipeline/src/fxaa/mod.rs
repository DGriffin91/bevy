mod node;

use bevy_ecs::query::QueryItem;
use bevy_render::camera::Camera;
use bevy_render::extract_component::{ExtractComponent, ExtractComponentPlugin};
pub use node::FXAANode;

use bevy_app::prelude::*;
use bevy_asset::{load_internal_asset, HandleUntyped};
use bevy_ecs::prelude::*;
use bevy_render::renderer::RenderDevice;
use bevy_render::texture::BevyDefault;
use bevy_render::{render_resource::*, RenderApp};

use bevy_reflect::TypeUuid;

use crate::fullscreen_vertex_shader::fullscreen_shader_vertex_state;

const FXAA_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 17015358199668024512);

const FXAA_SHARED_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2499420578245347910);

const BLIT_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2982161071241723543);

pub struct FXAAPlugin;

impl Plugin for FXAAPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, FXAA_SHADER_HANDLE, "fxaa.wgsl", Shader::from_wgsl);
        load_internal_asset!(
            app,
            FXAA_SHARED_SHADER_HANDLE,
            "fxaa_shared.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(app, BLIT_SHADER_HANDLE, "blit.wgsl", Shader::from_wgsl);

        app.add_plugin(ExtractComponentPlugin::<FXAA>::default());

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app.init_resource::<FXAAPipeline>();
    }
}

pub struct FXAAPipeline {
    hdr_texture_bind_group: BindGroupLayout,
    fxaa_pipeline_id: CachedRenderPipelineId,
    blit_pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for FXAAPipeline {
    fn from_world(render_world: &mut World) -> Self {
        let fxaa_texture_bind_group = render_world
            .resource::<RenderDevice>()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("fxaa_texture_bind_group_layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let fxaa_descriptor = RenderPipelineDescriptor {
            label: Some("fxaa pipeline".into()),
            layout: Some(vec![fxaa_texture_bind_group.clone()]),
            vertex: fullscreen_shader_vertex_state(),
            fragment: Some(FragmentState {
                shader: FXAA_SHADER_HANDLE.typed(),
                shader_defs: vec![],
                entry_point: "fs_main".into(),
                targets: vec![ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: None,
                    write_mask: ColorWrites::ALL,
                }],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
        };

        let blit_descriptor = RenderPipelineDescriptor {
            label: Some("blit pipeline".into()),
            layout: Some(vec![fxaa_texture_bind_group.clone()]),
            vertex: fullscreen_shader_vertex_state(),
            fragment: Some(FragmentState {
                shader: BLIT_SHADER_HANDLE.typed(),
                shader_defs: vec![],
                entry_point: "fs_main".into(),
                targets: vec![ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: None,
                    write_mask: ColorWrites::ALL,
                }],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
        };
        let mut cache = render_world.resource_mut::<PipelineCache>();
        FXAAPipeline {
            hdr_texture_bind_group: fxaa_texture_bind_group,
            fxaa_pipeline_id: cache.queue_render_pipeline(fxaa_descriptor),
            blit_pipeline_id: cache.queue_render_pipeline(blit_descriptor),
        }
    }
}

#[derive(Component, Clone)]
pub struct FXAA {
    pub is_enabled: bool,
}

impl ExtractComponent for FXAA {
    type Query = &'static Self;
    type Filter = With<Camera>;

    fn extract_component(item: QueryItem<Self::Query>) -> Self {
        item.clone()
    }
}
