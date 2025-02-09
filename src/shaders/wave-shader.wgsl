@group(0) @binding(0) var<storage, read_write> amplitudes: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let index = id.x;
    if (index < arrayLength(&amplitudes)) {
        let time = f32(index) * 0.01;
        amplitudes[index] = sin(time * 6.28318) * 0.5 + 0.5;
    }
}

