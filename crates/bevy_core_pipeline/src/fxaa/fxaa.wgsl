#import bevy_core_pipeline::fullscreen_vertex_shader
#import bevy_core_pipeline::fxaa

[[group(0), binding(0)]]
var texture: texture_2d<f32>;
[[group(0), binding(1)]]
var samp: sampler;

[[stage(fragment)]]
fn fs_main(in: FullscreenVertexOutput) -> [[location(0)]] vec4<f32> {
    let resolution = vec2<f32>(textureDimensions(texture));

    let frag_coord = in.position.xy;
	let inverseVP = 1.0 / resolution;

    // TODO move to vertex shader to optimize the shader by making 5 of the texture2D calls non-dependent
    // compute the texture coords
    let v_rgbNW = (frag_coord + vec2<f32>(-1.0, -1.0)) * inverseVP;
    let v_rgbNE = (frag_coord + vec2<f32>(1.0, -1.0)) * inverseVP;
    let v_rgbSW = (frag_coord + vec2<f32>(-1.0, 1.0)) * inverseVP;
    let v_rgbSE = (frag_coord + vec2<f32>(1.0, 1.0)) * inverseVP;
	let v_rgbM = vec2<f32>(frag_coord * inverseVP);

	//compute FXAA
    var output_color = fxaa(texture, samp, frag_coord, resolution, 
                            v_rgbNW, v_rgbNE, v_rgbSW, v_rgbSE, v_rgbM);

    return output_color;
}
