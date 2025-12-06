struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
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
@group(2) @binding(0) var<uniform> model: mat4x4<f32>;
@group(2) @binding(1) var<uniform> model_it: mat4x4<f32>;
@group(2) @binding(2) var<uniform> color: vec4<f32>;



@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var model_pos = vec4<f32>(in.position, 1.0);
    var proj_pos = projection * view * model * model_pos;

    var model_it3 = mat3x3<f32>(model_it[0].xyz, model_it[1].xyz, model_it[2].xyz);
    var world_normal = normalize(model_it3 * in.normal);

    var out: VertexOutput;
    out.clip_position = proj_pos;
    out.world_normal = world_normal;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var diffuse = max(dot(normalize(vec3<f32>(3.0, 4.0, 1.0)), in.world_normal), 0.0) * 0.95 + 0.05;

    var color = color * diffuse;

    var out: FragmentOutput;
    out.color = color;

    return out;
}