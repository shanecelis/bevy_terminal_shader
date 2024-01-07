
#import bevy_pbr::{
    mesh_view_bindings::globals,
    mesh_view_bindings::view,
    forward_io::VertexOutput,
    utils::PI,
}
// #import bevy_render::globals::Globals
// #import bevy_pbr::forward_io::VertexOutput
// #import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER
// CC0: Another windows terminal shader
//  Created this based on an old shader as a background in windows terminal

// struct TerminalMaterial {
//     foreground: vec4<f32>,
//     background: vec4<f32>
// };

// [[block]] struct VertexInput {
//     [[location(0)]] fragCoord: vec2<f32>;
// };

// [[block]] struct FragmentOutput {
//     [[location(0)]] fragColor: vec4<f32>;
// };

// [[group(0), binding(0)]] var<uniform> globals: Constants;
// [[group(0), binding(1)]] var<uniform> vertexInput: VertexInput;
// [[group(0), binding(2)]] var<out> fragmentOutput: FragmentOutput;

@group(1) @binding(0) var<uniform> foreground: vec4<f32>;
@group(1) @binding(1) var<uniform> background: vec4<f32>;
// @group(0) @binding(0) var<uniform> material: TerminalMaterial;

// var<workgroup> g_rot0: mat2x2<f32> = mat2x2<f32>(1.0, 0.0, 0.0, 1.0);
// var<workgroup> g_rot1: mat2x2<f32> = mat2x2<f32>(1.0, 0.0, 0.0, 1.0);

// License: WTFPL, author: sam hocevar, found: https://stackoverflow.com/a/17897228/418488
// var<workgroup> hsv2rgb_K: vec4<f32> = vec4<f32>(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
const hsv2rgb_K: vec4<f32> = vec4<f32>(1.0, 0.666, 0.333, 3.0);

fn hsv2rgb(c: vec3<f32>) -> vec3<f32> {
    let p = abs(fract(c.xxx + hsv2rgb_K.xyz) * 6.0 - hsv2rgb_K.www);
    return c.z * mix(hsv2rgb_K.xxx, saturate(p - hsv2rgb_K.xxx), c.y);
}

// License: WTFPL, author: sam hocevar, found: https://stackoverflow.com/a/17897228/418488
//  Macro version of above to enable compile-time globals
// fn HSV2RGB(c: vec3<f32>) -> vec3<f32> {
//     return c.z * mix(hsv2rgb_K.xxx, saturate(abs(fract(c.xxx + hsv2rgb_K.xyz) * 6.0 - hsv2rgb_K.www) - hsv2rgb_K.xxx), c.y);
// }

// var<workgroup> rot0: mat2x2<f32> = mat2x2<f32>((1.0), (0.0), -0.0, (1.0));
//var<workgroup> rot1: mat2x2<f32> = mat2x2<f32>((1.0), (0.0), -0.0, (1.0));

// License: Unknown, author: nmz (twitter: @stormoid), found: https://www.shadertoy.com/view/NdfyRM
fn sRGB(t: vec3<f32>) -> vec3<f32> {
    return mix(1.055 * pow(t, vec3(1.0 / 2.4)) - 0.055, 12.92 * t, step(t, vec3(0.0031308)));
}

// License: Unknown, author: Matt Taylor (https://github.com/64), found: https://64.github.io/tonemapping/
fn aces_approx(v_: vec3<f32>) -> vec3<f32> {
    var v = max(v_, vec3<f32>(0.0));
    v *= 0.6;
    let a: f32 = 2.51;
    let b: f32 = 0.03;
    let c: f32 = 2.43;
    let d: f32 = 0.59;
    let e: f32 = 0.14;
    return saturate((v * (a * v + b)) / (v * (c * v + d) + e));
}

fn apolloian(p_: vec3<f32>, s: f32) -> f32 {
    var p = p_;
    var scale: f32 = 1.0;
    for (var i: i32 = 0; i < 5; i = i + 1) {
        p = -1.0 + 2.0 * fract(0.5 * p + 0.5);
        let r2: f32 = dot(p, p);
        let k: f32 = s / r2;
        p *= k;
        scale *= k;
    }

    let ap: vec3<f32> = abs(p / scale);
    var d: f32 = length(ap.xy);
    d = min(d, ap.z);

    return d;
}

fn df(p_: vec2<f32>, time: f32, g_rot0: mat2x2<f32>, g_rot1: mat2x2<f32>) -> f32 {
    var p = p_;
    let fz: f32 = mix(0.75, 1.0, smoothstep(-0.9, 0.9, cos(time * 2.0 * PI / 300.0)));
    let z: f32 = 1.55 * fz;
    p /= z;
    var p3: vec3<f32> = vec3<f32>(p, 0.1);
    var r = p3.xy * g_rot0;
    // var r = p3.xy;// * g_rot0;
    p3.x = r.x;
    p3.z = r.y;
    r = p3.yz * g_rot1;
    // r = p3.yz;// * g_rot1;
    p3.y = r.x;
    p3.z = r.y;
    // p3.yz *= g_rot1;
    var d: f32 = apolloian(p3, 1.0 / fz);
    d *= z;
    return d;
}

fn effect(p: vec2<f32>, pp: vec2<f32>, time: f32, resolution: vec2<f32>) -> vec3<f32> {
    let g_rot0 = mat2x2<f32>(cos(0.1 * time), sin(0.1 * time),
                            -sin(0.1 * time), cos(0.1 * time));
    let g_rot1 = mat2x2<f32>(cos(0.123 * time), sin(0.123 * time),
                            -sin(0.123 * time), cos(0.123 * time));

    // let resolution = vec2<f32>(320.0, 640.0);
    // let aa: f32 = 2.0 / globals.resolution.y;
    let aa: f32 = 2.0 / resolution.y;

    let d: f32 = df(p, time, g_rot0, g_rot1);
    var bg: vec3<f32> = hsv2rgb(vec3<f32>(0.55, 0.85, 0.85));
    var fg: vec3<f32> = hsv2rgb(vec3<f32>(0.33, 0.85, 0.025));
    // if (1 > 0) {
    //     return vec3<f32>(0.1,0.2,0.3);
    //     // return abs(bg - background.rgb);
    // }
    bg = background.rgb;
    fg = foreground.rgb;
    var col: vec3<f32> = 0.1 * bg.rgb;
    col += fg.rgb / sqrt(abs(d));
    col += bg.rgb * smoothstep(aa, -aa, (d - 0.001));

    col *= smoothstep(1.5, 0.5, length(pp));

    return col;
}

// [[stage(vertex)]]
// fn mainVertex(input: VertexInput) -> void {
// }

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let q: vec2<f32> = mesh.uv;
    var p: vec2<f32> = -1.0 + 2.0 * q;
    let resolution: vec2<f32> = view.viewport.zw;

    let pp: vec2<f32> = p;
    p.x = p.x * resolution.x / resolution.y;
    var col: vec3<f32> = effect(p, pp, globals.time, resolution);
    col = aces_approx(col);
    col = sqrt(col);
    return vec4<f32>(col.rgb, 1.);
}
