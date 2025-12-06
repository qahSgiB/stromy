struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) model_a: vec4<f32>,
    @location(3) model_b: vec4<f32>,
    @location(4) model_c: vec4<f32>,
    @location(5) model_d: vec4<f32>,
    @location(6) model_it_a: vec4<f32>,
    @location(7) model_it_b: vec4<f32>,
    @location(8) model_it_c: vec4<f32>,
    @location(9) model_it_d: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}



@group(0) @binding(0) var<uniform> projection: mat4x4<f32>;
@group(1) @binding(0) var<uniform> view: mat4x4<f32>;
@group(2) @binding(0) var<uniform> color: vec4<f32>;



@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var model = mat4x4<f32>(in.model_a, in.model_b, in.model_c, in.model_d);
    var model_it3 = mat3x3<f32>(in.model_it_a.xyz, in.model_it_b.xyz, in.model_it_c.xyz);

    var model_pos = vec4<f32>(in.position, 1.0);
    var proj_pos = projection * view * model * model_pos;

    var world_normal = normalize(model_it3 * in.normal);

    var out: VertexOutput;
    out.clip_position = proj_pos;
    out.world_normal = world_normal;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var diffuse = max(dot(normalize(vec3<f32>(3.0, 4.0, 1.0)), in.world_normal), 0.0) * 0.8 + 0.2;

    var color = color * diffuse;

    var out: FragmentOutput;
    out.color = color;

    return out;
}