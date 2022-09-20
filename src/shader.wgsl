struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct Rotation {
    transform: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> rotation: Rotation;

struct Camera {
    view_position: vec4<f32>,
    view_proj: mat4x4<f32>,
    proj: mat4x4<f32>,
    proj_inv: mat4x4<f32>,
    view: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_background(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var vertices = array<vec3<f32>, 6>(
        vec3<f32>(-10.0, 10.0, -10.0),
        vec3<f32>(-10.0, -10.0, -10.0),
        vec3<f32>(10.0, 10.0, -10.0),

        vec3<f32>(10.0, 10.0, -10.0),
        vec3<f32>(-10.0, -10.0, -10.0),
        vec3<f32>(10.0, -10.0, -10.0),
    );

    var colors = array<vec4<f32>, 6>(
        vec4<f32>(1.0, 1.0, 0.0, 0.75),
        vec4<f32>(0.0, 1.0, 1.0, 0.75),
        vec4<f32>(1.0, 0.0, 1.0, 0.75),

        vec4<f32>(1.0, 0.0, 1.0, 0.75),
        vec4<f32>(0.0, 1.0, 1.0, 0.75),
        vec4<f32>(0.5, 0.5, 0.5, 0.75),
    );

    var coords = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 1.0),
    );
    /*
    var colors = array<vec4<f32>, 6>(
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),

        vec4<f32>(0.0, 0.0, 0.0, 1.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
    */

    let v = vertices[in_vertex_index];

    var out: VertexOutput;
    out.clip_position =
        camera.view_proj * vec4<f32>(v, 1.0) + vec4<f32>(4.0, 0.0, 0.0, 0.0);
    out.color = colors[in_vertex_index];
    out.tex_coords = coords[in_vertex_index];
    return out;
}

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
        vec4<f32>(1.0, 0.0, 0.0, 0.75),
        vec4<f32>(1.0, 0.0, 0.0, 0.0),

        vec4<f32>(0.0, 1.0, 0.0, 1.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.75),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),

        vec4<f32>(0.0, 0.0, 1.0, 1.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.75),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
    );

    let v = vertices[in_vertex_index];

    var out: VertexOutput;
    out.clip_position =
        camera.view_proj * rotation.transform * vec4<f32>(v, 1.0) + vec4<f32>(4.0, 0.0, 0.0, 0.0);
    out.color = colors[in_vertex_index];
    out.tex_coords = vec2<f32>(0.0, 0.0);
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
        vec4<f32>(1.0, 0.0, 1.0, 1.0),
        vec4<f32>(0.0, 1.0, 0.0, 1.0),
        vec4<f32>(1.0, 0.0, 1.0, 1.0),

        vec4<f32>(0.0, 1.0, 0.0, 1.0),
        vec4<f32>(1.0, 0.0, 1.0, 1.0),
        vec4<f32>(0.0, 1.0, 0.0, 1.0),

        vec4<f32>(0.0, 0.0, 0.0, 1.0),
        vec4<f32>(0.5, 0.5, 0.5, 1.0),
        vec4<f32>(1.0, 1.0, 1.0, 1.0),

        vec4<f32>(1.0, 1.0, 1.0, 1.0),
        vec4<f32>(0.5, 0.5, 0.5, 1.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );

    let v = vertices[in_vertex_index];

    var out: VertexOutput;
    out.clip_position =
        camera.view_proj * rotation.transform * vec4<f32>(v, 1.0) + vec4<f32>(-4.0, 0.0, 0.0, 0.0);
    out.color = colors[in_vertex_index];
    out.tex_coords = vec2<f32>(0.0, 0.0);
    return out;
}

struct SkyOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec3<f32>,
};

@vertex
fn vs_sky(@builtin(vertex_index) vertex_index: u32) -> SkyOutput {
    // hacky way to draw a large triangle
    let tmp1 = i32(vertex_index) / 2;
    let tmp2 = i32(vertex_index) & 1;
    let pos = vec4<f32>(
        f32(tmp1) * 4.0 - 1.0,
        f32(tmp2) * 4.0 - 1.0,
        1.0,
        1.0
    );

    // transposition = inversion for this orthonormal matrix
    let inv_model_view = transpose(mat3x3<f32>(camera.view.x.xyz, camera.view.y.xyz, camera.view.z.xyz));
    let unprojected = camera.proj_inv * pos;

    var result: SkyOutput;
    result.uv = inv_model_view * unprojected.xyz;
    result.position = pos;
    return result;
}

@group(2) @binding(0)
var r_texture: texture_cube<f32>;
@group(2) @binding(1)
var r_sampler: sampler;

@group(3) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(3) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}

@fragment
fn fs_texture(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

@fragment
fn fs_sky(vertex: SkyOutput) -> @location(0) vec4<f32> {
    return textureSample(r_texture, r_sampler, vertex.uv);
}
