#import bevy_render::view::View
#import bevy_ui::ui_vertex_output::UiVertexOutput

struct CustomUiMaterial {
    @location(0) texture_resolution: vec2<f32>,
	@location(0) _padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> view: View;

@group(1) @binding(0) var<uniform> input: CustomUiMaterial;
@group(1) @binding(1) var texture: texture_2d<f32>;
@group(1) @binding(2) var texture_sampler: sampler;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
	let texture_coords = vec2<f32>(
		in.position.x / 6. % input.texture_resolution.x / input.texture_resolution.x,
		in.position.y / 6. % input.texture_resolution.y / input.texture_resolution.y,
	);
	let color = textureSample(texture, texture_sampler, texture_coords);
	

    return vec4<f32>(color);
}
