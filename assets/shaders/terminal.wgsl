// CC0: Another windows terminal shader by
// Created this based on an old shader as a background in windows terminal.
#import bevy_render::{
    view::View,
    globals::Globals,
}

#ifdef IS_2D
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#else
#import bevy_pbr::{
    forward_io::VertexOutput,
}
#endif


#import bevy_render::maths::PI
#import bevy_core_pipeline::tonemapping::tone_mapping

@group(0) @binding(0) var<uniform> view: View;
#ifdef IS_2D
@group(0) @binding(1) var<uniform> globals: Globals; // Works on 2d.
#else
@group(0) @binding(9) var<uniform> globals: Globals; // Works on 3d.
#endif

// Bindings for our TerminalMaterial struct.
@group(2) @binding(0) var<uniform> foreground: vec4<f32>;
@group(2) @binding(1) var<uniform> background: vec4<f32>;

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

    // p3.xy *= g_rot0;
    var r: vec2<f32> = p3.xy * g_rot0;
    p3.x = r.x;
    p3.y = r.y;

    //p3.yz *= g_rot1;
    r = p3.yz * g_rot1;
    p3.y = r.x;
    p3.z = r.y;

    var d: f32 = apolloian(p3, 1.0 / fz);
    d *= z;
    return d;
}

fn effect(p: vec2<f32>, pp: vec2<f32>, time: f32, resolution: vec2<f32>) -> vec3<f32> {
    let g_rot0 = mat2x2<f32>(cos(0.1 * time), sin(0.1 * time),
                            -sin(0.1 * time), cos(0.1 * time));
    let g_rot1 = mat2x2<f32>(cos(0.123 * time), sin(0.123 * time),
                            -sin(0.123 * time), cos(0.123 * time));
    let aa: f32 = 2.0 / resolution.y;

    let d: f32 = df(p, time, g_rot0, g_rot1);
    // Green foreground and blue background HSV colors from the original.
    // var fg: vec3<f32> = hsv2rgb(vec3<f32>(0.33, 0.85, 0.025));
    // var bg: vec3<f32> = hsv2rgb(vec3<f32>(0.55, 0.85, 0.85));
    let fg = foreground.rgb;
    let bg = background.rgb;
    var col: vec3<f32> = 0.1 * bg.rgb;
    col += fg.rgb / sqrt(abs(d));
    col += bg.rgb * smoothstep(aa, -aa, (d - 0.001));

    col *= smoothstep(1.5, 0.5, length(pp));

    return col;
}

// https://github.com/bevyengine/bevy/discussions/8937
fn from_linear(linear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(linear, vec4<f32>(0.0031308));
    let higher = vec4<f32>(1.055) * pow(linear, vec4(1.0 / 2.4)) - vec4(0.055);
    let lower = linear * vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let q: vec2<f32> = in.uv;
    var p: vec2<f32> = -1.0 + 2.0 * q;
    let resolution: vec2<f32> = view.viewport.zw;

    let pp: vec2<f32> = p;
    p.x = p.x * resolution.x / resolution.y;
    var col: vec3<f32> = effect(p, pp, globals.time, resolution);
    // var col: vec3<f32> = effect(p, pp, 4.2, resolution);
    col = aces_approx(col);
    col = sqrt(col);
    // col = vec3<f32>(0.1, 0.2, 0.3);
    let c = to_linear(vec4<f32>(col.rgb, 1.));
#ifdef TONEMAP_IN_SHADER
    return tone_mapping(c, view.color_grading);
#else
    return c;
#endif
}
