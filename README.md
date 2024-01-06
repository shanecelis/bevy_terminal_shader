# bevy_video_glitch
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/bevy_video_glitch/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/bevy_video_glitch/actions)
  [![crates-io](https://img.shields.io/crates/v/bevy_video_glitch.svg)](https://crates.io/crates/bevy_video_glitch)
  [![api-docs](https://docs.rs/bevy_video_glitch/badge.svg)](https://docs.rs/bevy_video_glitch)

This crate provides a post processing video glitch effect for the [bevy game engine](https://bevyengine.org). 

![Cube example](https://github.com/shanecelis/bevy_video_glitch/assets/54390/95100192-b1eb-4797-bce7-0c71b4f842f4)

# Install

``` sh
cargo add bevy_video_glitch
```

# Usage

## Add plugin to app
```compile
use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(bevy_video_glitch::VideoGlitchPlugin)
        .run()
}
```

## Add settings to camera

```compile
use bevy::prelude::*;
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        // This component is also used to determine on which camera to run the post processing effect.
        bevy_video_glitch::VideoGlitchSettings {
            intensity: 1.0,
            color_aberration: Mat3::IDENTITY
        },
    ));
```

# Example

Run the example like so:

``` sh
cargo run --example cube
```

This will show a rotating cube like the one shown at the beginning of this README.

# License

This crate is licensed under the MIT License or the Apache License 2.0.

# Acknowlegments

* [Video Glitch](https://www.shadertoy.com/view/XtK3W3) by [dyvoid](https://www.shadertoy.com/user/dyvoid).

* [Post Processing](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/shader/post_processing.rs) example from [bevy](https://bevyengine.org), which I wrote a series of toots about [here](https://mastodon.gamedev.place/@shanecelis/111583689226043395).
