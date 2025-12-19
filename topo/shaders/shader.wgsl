@group(0) @binding(0) var<storage, read> heights: array<f32>;

struct PushConstants {
    window_width: u32,
    window_height: u32,
};
var<push_constant> constants: PushConstants;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let pos = array<vec2<f32>, 3>(
        vec2<f32>(-1., -3.),
        vec2<f32>(-1., 1.),
        vec2<f32>(3., 1.),
    );

    return vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
}

fn height(in: f32) -> u32 {
    let step = 0.75;
    return u32((in - step * round(in / step) > 0.));
    // return u32(sign(in));
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    var marching: u32 = 0;
    var i: u32 = 0;
    marching = marching | (height((heights[(u32(frag_coord.y) + 0) * (constants.window_width + 1) + u32(frag_coord.x) + 0])) << 0);
    marching = marching | (height((heights[(u32(frag_coord.y) + 0) * (constants.window_width + 1) + u32(frag_coord.x) + 1])) << 1);
    marching = marching | (height((heights[(u32(frag_coord.y) + 1) * (constants.window_width + 1) + u32(frag_coord.x) + 1])) << 2);
    marching = marching | (height((heights[(u32(frag_coord.y) + 1) * (constants.window_width + 1) + u32(frag_coord.x) + 0])) << 3);

    let table = array<f32, 16>(
        0.,
        1.,
        1.,
        1.,
        1.,
        1.,
        1.,
        1.,
        1.,
        1.,
        1.,
        1.,
        1.,
        1.,
        1.,
        0.,
    );

    return vec4<f32>(table[marching], table[marching], table[marching], 1.0);
}

// @fragment
// fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
//     let size: u32 = 4;
//     var marching: u32 = 0;
//     for (var x: u32 = 0; x < size; x++) {
//         for (var y: u32 = 0; y < size; y++) {
//             marching                                     += height((heights[(u32(frag_coord.y) + y) * (constants.window_width + 1) + u32(frag_coord.x) + x]));
//         }
//     }

//     let col = 1. - abs(f32(marching) - f32(size) * f32(size) / 2.) / f32(size * size) * 2.;
//     return vec4<f32>(col, col, col, 1.0);
// }
