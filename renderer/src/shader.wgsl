@group(0) @binding(0)
var<uniform> resolution: vec2<u32>;

struct VertexInput {
    @location(0) position: vec2<u32>,
};

struct InstanceInput {
    @location(1) position: vec2<u32>,
    @location(2) color: vec3<f32>,
    @location(3) lighting: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = instance.color * instance.lighting;
    var vertex_position: vec2<f32>;
    vertex_position.x = f32(f32(instance.position.x + model.position.x) * 2.0) / f32(resolution.x);
    vertex_position.y = f32(f32(instance.position.y + model.position.y) * -2.0) / f32(resolution.y);
    out.clip_position = vec4<f32>(vertex_position.x - 1.0, vertex_position.y + 1.0, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
