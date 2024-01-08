#![doc(html_root_url = "https://docs.rs/bevy_terminal_shader/0.1.0")]
#![doc = include_str!("../README.md")]
use bevy::{
    asset::load_internal_asset,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, SpecializedMeshPipelineError},
    render::{
        mesh::InnerMeshVertexBufferLayout,
        render_resource::{
            RenderPipelineDescriptor, Shader,
        },
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
    utils::Hashed,
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
        app.add_plugins((
            MaterialPlugin::<TerminalMaterial>::default(),
            Material2dPlugin::<TerminalMaterial>::default(),
        ));
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

impl Material2d for TerminalMaterial {
    fn fragment_shader() -> ShaderRef {
        TERMINAL_SHADER_HANDLE.into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &Hashed<InnerMeshVertexBufferLayout>,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // if key.bind_group_data.is_red {
        let fragment = descriptor.fragment.as_mut().unwrap();
        fragment.shader_defs.push("IS_2D".into());
        // }
        Ok(())
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TerminalMaterial {
    #[uniform(0)]
    pub foreground: Color,
    // pub foreground: Vec4,
    #[uniform(1)]
    // pub background: Vec4,
    pub background: Color,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
    // alpha_mode: AlphaMode,
}

impl TerminalMaterial {
    /// Standardize the luminance for the foreground and background to the same
    /// luminance values used in [green()][].
    pub fn standardize(mut self) -> Self {
        self.foreground.set_l(0.109);
        self.background.set_l(0.658);
        self.foreground = self.foreground.as_rgba();
        self.background = self.background.as_rgba();
        self
    }

    pub fn green() -> Self {
        Self {
            foreground:
            // Color::hsl(0.33, 0.85, 0.025).as_rgba(),                   // HSL value as given in the original shader.
            // Color::hsl(118.8, 0.74, 0.015).as_rgba(),                  // Web, convert HSL to RGB. Nope.
            // Color::hsl(118.8, 0.85, 0.025).as_rgba(),                  // Siri, convert.... Nah uh.
            // Color::hsl(118.8, 0.50, 0.025).as_rgba(),                  // ChatGPT, convert.... Still no.
            // Color::hsl(101.0, 1.0, 0.012).as_rgba(),                   // Not even close.
            // Color::rgba(0.0042, 0.0238, 0.0150, 1.0),                  // I feel like I'm taking crazy pills!
            // Color::rgba_linear(0.0042, 0.0238, 0.004, 1.0),            // Do not talk to me or my son ever again!
            //
            // Eval shader's code: hsl2rgb(vec(0.33, 0.85, 0.025)) -> vec(0.0042, 0.0238, 0.0150)
            //
            // Color::rgba(0.0042, 0.0238, 0.0150, 1.0),                  // No? THIS IS THE RGB VALUE THE SHADER USES!
            // Color::rgba_linear(0.0042, 0.0238, 0.0150, 1.0).as_rgba(), // Oh, ok. It's linear in the shader.
            Color::hsl(118.882, 0.535, 0.109).as_rgba(),                  // Let's go back to use the correct HSL value.

            background:
            // Color::hsl(0.55, 0.85, 0.85).as_rgba(), // as given in the original
            // Color::hsl(198.0, 0.74, 0.49).as_rgba(),
            // Color::hsl(198.0, 0.85, 0.85).as_rgba(),
            // Color::rgba(0.1275, 0.8075, 0.765, 1.0),
            // Color::rgba_linear(0.122, 0.631, 0.851, 1.0).into(),
            // Color::rgba(0.1275, 0.8075, 0.765, 1.0).as_rgba(),
            // Color::rgba_linear(0.1275, 0.8075, 0.765, 1.0).as_rgba(),
            Color::hsl(192.671, 0.800, 0.658).as_rgba(),
        }
    }
}


impl Default for TerminalMaterial {
    fn default() -> Self {
        Self {
            foreground: Color::WHITE,
            background: Color::BLACK,
        }.standardize()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fg_to_hsl() {
        let x = Color::rgba_linear(0.0042, 0.0238, 0.004, 1.0);
        let y = x.as_hsla();
        assert_eq!(
            &format!("{:?}", y),
            "Hsla { hue: 118.882225, saturation: 0.5346773, lightness: 0.108975366, alpha: 1.0 }"
        );
    }

    #[test]
    fn test_bg_to_hsl() {
        let x = Color::rgba_linear(0.122, 0.631, 0.851, 1.0);
        let y = x.as_hsla();
        assert_eq!(
            &format!("{:?}", y),
            "Hsla { hue: 192.67107, saturation: 0.79958016, lightness: 0.6577566, alpha: 1.0 }"
        );
    }
}
