<img src="./assets/img/logos/readme.png" width="100%">

# Deep Breath


[![Build](https://github.com/Ewpratten/ludum-dare-48/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/ludum-dare-48/actions/workflows/build.yml)
[![Bundle](https://github.com/Ewpratten/ludum-dare-48/actions/workflows/bundle.yml/badge.svg)](https://github.com/Ewpratten/ludum-dare-48/actions/workflows/bundle.yml)
[![Ludum Dare 48](https://img.shields.io/badge/Ludum%20Dare-48-orange)](https://ldjam.com/events/ludum-dare/48/$236526)
[![Rust 1.51](https://img.shields.io/badge/Rust-1.51-orange)](https://www.rust-lang.org/)
[![Made with Raylib](https://img.shields.io/badge/Made%20With-raylib-blue)](https://www.raylib.com/)

**Deep Breath** is an exploration game where you explore an underwater cave in hopes of finding your lost transponder. Items and upgrades can be acquired along the way to assist your search.

This game was written in [Rust](https://www.rust-lang.org/), on top of [Rust bindings](https://github.com/deltaphc/raylib-rs) to the [`raylib`](https://github.com/raysan5/raylib) graphics library. For most of the team, this has been our first big Rust project.

This has been our second game produced for Ludum Dare. Check out the first [here](https://ldjam.com/events/ludum-dare/46/micromanaged-mike).

## Development Resources

Documentation:

 - [Raylib C documentation](https://www.raylib.com/cheatsheet/cheatsheet.html)
 - [Raylib C examples](https://www.raylib.com/examples.html)
 - [Raylib Rust documentation](https://docs.rs/raylib/3.5.0/raylib/)
 - [Raylib Rust examples](https://github.com/deltaphc/raylib-rs/tree/master/samples)
 - ["Are We Game Yet?"](https://arewegameyet.rs/#ecosystem)
 - [`cross` cross-compiler tool](https://github.com/rust-embedded/cross)

Core libraries:

 - [`raylib-rs`](https://github.com/deltaphc/raylib-rs)
 - [`serde`](https://serde.rs/)
 - [`serialstudio-rs`](https://github.com/Ewpratten/serialstudio-rs)

Sound Samples:
 - [JavierZumer](https://freesound.org/people/JavierZumer/sounds/257236/)
 - [Noted451](https://freesound.org/people/Noted451/sounds/531015/)

### VSCode Setup

If using VSCode, disable the `Rust` extension, and install everything in the **Workspace Recommendations** (You will see this list by searching `@recommended` in the extensions panel)

### Attaching to the in-game profiler

When the game is ran in its `dev` profile (using `cargo run`), the internal profiler is exposed on `127.0.0.1:8019`.

To connect to this, install [Serial Studio](https://www.alex-spataru.com/serial-studio.html) from [here](https://github.com/Serial-Studio/Serial-Studio/releases/tag/v1.0.20), and point it at `127.0.0.1:8019` by selecting the `auto` communication mode, then selecting `Network>TCP` for the connection type. 

### Cross-compiling builds for other platforms

On linux, you can run `./bundle/create-releases.sh` to automatically cross-compile to all supported platforms as long as you have [`cross`](https://github.com/rust-embedded/cross) installed on your system. This is also done by the [Bundle CI task](https://github.com/Ewpratten/ludum-dare-48/actions/workflows/bundle.yml) every time code is pushed to `master`. 