# CapiTube

This is a PNGtuber app made for streamers (like me) inspired on [VeadoTube](https://veado.tube).

***

## Compile by yourself.
If you want to compile the CapiTube yourself, you will need to:
 - Follow the [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)
 - [Node.js](https://nodejs.org/en)
 - Have installed [RustLang](https://www.rust-lang.org/)
 - And have the Tauri CLI in your environment:
    - Run `cargo install tauri-cli` to install in the `cargo` (the Rust package manager).

After you install all the rrerequisites, to compile you will this commands on the terminal:
 - `cd app`
 - `npm install`
 - `cargo tauri dev` or `npm run tauri dev` to build.
 - `cargo tauri build` or `npm run tauri build` to build as a release.
 