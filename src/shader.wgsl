struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Rotation {
    transform: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> rotation: Rotation;

struct Camera {
    view_position: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_pyramid(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var vertices = array<vec3<f32>, 9>(
        vec3<f32>(0.0, 0.5, 0.0),
        vec3<f32>(-0.5, -0.5, 0.5),
        vec3<f32>(0.5, -0.5, 0.5),

        vec3<f32>(0.0, 0.5, 0.0),
        vec3<f32>(0.5, -0.5, 0.5),
        vec3<f32>(0.0, -0.5, -0.5),

        vec3<f32>(0.0, 0.5, 0.0),
        vec3<f32>(0.0, -0.5, -0.5),
        vec3<f32>(-0.5, -0.5, 0.5),
    );

    var colors = array<vec4<f32>, 9>(
        vec4<f32>(1.0, 0.0, 0.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),

        vec4<f32>(0.0, 1.0, 0.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),

        vec4<f32>(0.0, 0.0, 1.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
    );

    let v = vertices[in_vertex_index];

    var out: VertexOutput;
    out.clip_position = camera.view_proj * rotation.transform * vec4<f32>(v, 1.0) + vec4<f32>(4.0, 0.0, 0.0, 0.0);
    out.color = colors[in_vertex_index];
    return out;
}

@vertex
fn vs_pyramid4(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var vertices = array<vec3<f32>, 12>(
        vec3<f32>(0.0, 0.5, 0.0),
        vec3<f32>(-0.5, -0.5, 0.5),
        vec3<f32>(0.5, -0.5, 0.5),

        vec3<f32>(0.0, 0.5, 0.0),
        vec3<f32>(0.5, -0.5, 0.5),
        vec3<f32>(0.5, -0.5, -0.5),

        vec3<f32>(0.0, 0.5, 0.0),
        vec3<f32>(0.5, -0.5, -0.5),
        vec3<f32>(-0.5, -0.5, -0.5),

        vec3<f32>(0.0, 0.5, 0.0),
        vec3<f32>(-0.5, -0.5, -0.5),
        vec3<f32>(-0.5, -0.5, 0.5),
    );

    var colors = array<vec4<f32>, 12>(
        vec4<f32>(1.0, 0.0, 0.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),

        vec4<f32>(0.0, 1.0, 0.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),

        vec4<f32>(0.0, 0.0, 1.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),

        vec4<f32>(0.5, 0.5, 0.5, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
    );

    let v = vertices[in_vertex_index];

    var out: VertexOutput;
    out.clip_position = camera.view_proj * rotation.transform * vec4<f32>(v, 1.0) + vec4<f32>(-4.0, 0.0, 0.0, 0.0);
    out.color = colors[in_vertex_index];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
