#![doc(html_root_url = "https://docs.rs/bevy_terminal_shader/0.2.0")]
#![doc = include_str!("../README.md")]
use bevy::{
    asset::load_internal_asset,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, SpecializedMeshPipelineError},
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{RenderPipelineDescriptor, Shader},
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
};

// $ cargo install uuid-tools && uuid -o simple
pub const TERMINAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0xd695671d24e94abab6e1eba7d79e8095u128);

/// Provides a [TerminalMaterial][] that can be applied to 2D and 3D objects.
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

impl Material for TerminalMaterial {
    fn fragment_shader() -> ShaderRef {
        TERMINAL_SHADER_HANDLE.into()
    }
}

impl Material2d for TerminalMaterial {
    fn fragment_shader() -> ShaderRef {
        TERMINAL_SHADER_HANDLE.into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // if key.bind_group_data.is_red {
        let fragment = descriptor.fragment.as_mut().unwrap();
        fragment.shader_defs.push("IS_2D".into());
        // }
        Ok(())
    }
}

// Define foreground and background color of the terminal material. Because the
// shader processes things as in a Signed Distance Function (SDF), the default
// luminance may seem awkward especially for the foreground. It's suggested you
// provide whatever colors you like and then run [standardize()][].
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TerminalMaterial {
    /// The foreground color.
    #[uniform(0)]
    pub foreground: LinearRgba,
    /// The background color.
    #[uniform(1)]
    pub background: LinearRgba,
}

impl TerminalMaterial {
    /// Standardize the luminance for the foreground and background to the same
    /// luminance values used in [Self::green()].
    ///
    /// NOTE: The luminance for the foreground is absurdly low. It's ~0.1 which
    /// looks black in most cases.
    ///
    /// TODO: Change the shader such that the foreground color might specify its
    /// brightest point rather than its darkest point.
    pub fn standardize(mut self) -> Self {
        self.foreground = self.foreground.with_luminance(0.109);
        self.background = self.background.with_luminance(0.109);
        self
    }

    /// A green terminal as shown in the [original](https://www.shadertoy.com/view/DdSGzy).
    pub fn green() -> Self {
        Self {
            foreground: Color::hsl(118.882, 0.535, 0.109).into(),
            background: Color::hsl(192.671, 0.800, 0.658).into(),
        }
    }
}

impl Default for TerminalMaterial {
    /// Provides white on black default that is [Self::standardize()]'d.
    fn default() -> Self {
        Self {
            foreground: Color::WHITE.into(),
            background: Color::BLACK.into(),
        }
        .standardize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fg_to_hsl() {
        let x = LinearRgba::new(0.0042, 0.0238, 0.004, 1.0);
        let y = Hsla::from(x);
        assert_eq!(
            &format!("{:?}", y),
            "Hsla { hue: 118.882225, saturation: 0.5346774, lightness: 0.10897538, alpha: 1.0 }"
        );
    }

    #[test]
    fn test_bg_to_hsl() {
        let x = LinearRgba::new(0.122, 0.631, 0.851, 1.0);
        let y = Hsla::from(x);
        assert_eq!(
            &format!("{:?}", y),
            "Hsla { hue: 192.67107, saturation: 0.79958016, lightness: 0.6577566, alpha: 1.0 }"
        );
    }
}
