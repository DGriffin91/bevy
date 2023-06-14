#define_import_path bevy_pbr::mesh_vertex_output

#ifdef SSAA
@location(0) @interpolate(perspective, sample) world_position: vec4<f32>,
@location(1) @interpolate(perspective, sample) world_normal: vec3<f32>,
#ifdef VERTEX_UVS
@location(2) @interpolate(perspective, sample) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
@location(3) @interpolate(perspective, sample) world_tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
@location(4) @interpolate(perspective, sample) color: vec4<f32>,
#endif
#else //SSAA
@location(0) world_position: vec4<f32>,
@location(1) world_normal: vec3<f32>,
#ifdef VERTEX_UVS
@location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
@location(3) world_tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
@location(4) color: vec4<f32>,
#endif
#endif //SSAA