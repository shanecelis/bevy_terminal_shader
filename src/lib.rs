#![doc(html_root_url = "https://docs.rs/bevy_terminal_shader/0.1.0")]
#![doc = include_str!("../README.md")]
use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    asset::load_internal_asset,
    core_pipeline::{core_3d, fullscreen_vertex_shader::fullscreen_shader_vertex_state},
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{
            ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
        },
        globals::{GlobalsBuffer, GlobalsUniform},
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            BindGroupEntries, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
            BindingType, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState,
            MultisampleState, Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment,
            RenderPassDescriptor, RenderPipelineDescriptor, Sampler, SamplerBindingType,
            SamplerDescriptor, ShaderStages, ShaderType, TextureFormat, TextureSampleType,
            TextureViewDimension, Shader,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
        RenderApp,
    },
};

// $ cargo install uuid-tools && uuid -o simple
pub const TERMINAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0xd695671d24e94abab6e1eba7d79e8095u128);

pub struct TerminalShaderPlugin;

impl Plugin for TerminalShaderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            TERMINAL_SHADER_HANDLE,
            "../assets/shaders/terminal.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<TerminalMaterial>::default());
        // app.add_plugins((
        //     // The settings will be a component that lives in the main world but will
        //     // be extracted to the render world every frame.
        //     // This makes it possible to control the effect from the main world.
        //     // This plugin will take care of extracting it automatically.
        //     // It's important to derive [`ExtractComponent`] on [`TerminalShaderSettings`]
        //     // for this plugin to work correctly.
        //     ExtractComponentPlugin::<TerminalShaderSettings>::default(),
        //     // The settings will also be the data used in the shader.
        //     // This plugin will prepare the component for the GPU by creating a uniform buffer
        //     // and writing the data to that buffer every frame.
        //     UniformComponentPlugin::<TerminalShaderSettings>::default(),
        // ));
    }
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for TerminalMaterial {
    fn fragment_shader() -> ShaderRef {
        // "shaders/custom_material.wgsl".into()
        // "shaders/terminal.wgsl".into()
        TERMINAL_SHADER_HANDLE.into()
    }

    // fn alpha_mode(&self) -> AlphaMode {
    //     self.alpha_mode
    // }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TerminalMaterial {
    // #[uniform(0)]
    // color: Color,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
    // alpha_mode: AlphaMode,
}

// This is the component that will get passed to the shader
// #[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
// pub struct TerminalShaderSettings {
//     /// Set the intensity of this glitch effect from [0, 1]. By default it has a
//     /// value of 1.
//     pub intensity: f32,
//     /// This shader uses a color aberration matrix C in the following way: The
//     /// first column `C[0] . color` selects the primary color, which is used to
//     /// mix the other two. In practice this means one will not see the primary
//     /// color in the color aberrations but will instead see traces of the
//     /// secondary colors: `C[1] . color` and `C[2] . color`.
//     ///
//     /// The default value is an identity matrix, which specifies red as the
//     /// primary color. Typically this matrix will be a doubly stochastic matrix
//     /// meaning the columns and rows each sum to 1.
//     pub color_aberration: Mat3,
//     // WebGL2 structs must be 16 byte aligned.
//     #[cfg(feature = "webgl2")]
//     webgl2_padding: Vec2,
// }

// impl Default for TerminalShaderSettings {
//     fn default() -> Self {
//         Self {
//             intensity: 1.0,
//             color_aberration: Mat3::IDENTITY,
//         }
//     }
// }
