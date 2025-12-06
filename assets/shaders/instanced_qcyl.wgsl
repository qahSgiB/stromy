struct VertexInput {
    @location(0) radius: f32,
    @location(1) model_a: vec4<f32>,
    @location(2) model_b: vec4<f32>,
    @location(3) model_c: vec4<f32>,
    @location(4) model_d: vec4<f32>,
    @location(5) model_it_a: vec4<f32>,
    @location(6) model_it_b: vec4<f32>,
    @location(7) model_it_c: vec4<f32>,
    @location(8) model_it_d: vec4<f32>,
    @builtin(vertex_index) vertex_index: u32,
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
@group(2) @binding(1) var<uniform> resolution: u32;
@group(2) @binding(2) var<uniform> smooth_normals_u32: u32;



const PI = 3.14159265358979323846264338327950288;



@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // # global consts
    // TODO: move outside fn
    var offsets_up_down    = array<f32, 12>(1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0);
    var offsets_in_out     = array<f32, 12>(1.0, 1.0, 0.0,  1.0, 1.0, 1.0,  1.0,  1.0, 1.0,  1.0,  1.0,  0.0);
    var offsets_left_right = array<u32, 12>(  0,   1,   0,    0,   1,   0,    0,    1,   1,    1,    0,    0);

    var normals = array<vec3<f32>, 12>(
        vec3<f32>( 1.0, 0.0, 0.0),
        vec3<f32>( 1.0, 0.0, 0.0),
        vec3<f32>( 1.0, 0.0, 0.0),
        vec3<f32>( 0.0, 1.0, 1.0),
        vec3<f32>( 0.0, 1.0, 1.0),
        vec3<f32>( 0.0, 1.0, 1.0),
        vec3<f32>( 0.0, 1.0, 1.0),
        vec3<f32>( 0.0, 1.0, 1.0),
        vec3<f32>( 0.0, 1.0, 1.0),
        vec3<f32>(-1.0, 0.0, 0.0),
        vec3<f32>(-1.0, 0.0, 0.0),
        vec3<f32>(-1.0, 0.0, 0.0),
    );


    var smooth_normals = smooth_normals_u32 != u32(0);

    // # offsets
    var offset_index = in.vertex_index % u32(12);
    var angle_index = in.vertex_index / u32(12);

    var offset_up_down = offsets_up_down[offset_index];
    var offset_in_out = offsets_in_out[offset_index];
    var offset_left_right = offsets_left_right[offset_index];

    // # position
    var r = select(1.0, in.radius, offset_up_down == 1.0);

    var resolution_angle = 2.0 * PI / f32(resolution); // TODO: opt
    var angle = f32(angle_index + offset_left_right) * resolution_angle;

    // TODO: coordinates order
    var position = vec3<f32>(
        offset_up_down,
        cos(angle) * r * offset_in_out,
        sin(angle) * r * offset_in_out,
    );

    // # normal
    var normal_angle= 0.0;
    if smooth_normals {
        normal_angle = angle; // TODO: cos, sin already computed
    } else {
        normal_angle = (f32(angle_index) + 0.5) * resolution_angle;
    }

    var normal = normals[offset_index] * vec3<f32>(
        1.0,
        cos(normal_angle),
        sin(normal_angle),
    );

    // # mvp
    var model = mat4x4<f32>(in.model_a, in.model_b, in.model_c, in.model_d);
    var model_it3 = mat3x3<f32>(in.model_it_a.xyz, in.model_it_b.xyz, in.model_it_c.xyz);

    var model_pos = vec4<f32>(position, 1.0);
    var proj_pos = projection * view * model * model_pos;

    var world_normal = normalize(model_it3 * normal);

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