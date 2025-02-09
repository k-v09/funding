@group(0) @binding(0) var<uniform> time: f32;
@group(0) @binding(1) var<uniform> emission_position: vec2<f32>;

@fragment
fn main(@builtin(position) frag_pos: vec4<f32>) -> @location(0) vec4<f32> {
    let pos = frag_pos.xy / 10.0; // Normalize space
    let dist = distance(pos, emission_position);
    let wave = sin(10.0 * dist - 5.0 * time);
    let color = 0.5 + 0.5 * wave; // Normalize to range [0,1]
    return vec4<f32>(color, color, color, 1.0);
}

