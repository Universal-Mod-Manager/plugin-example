# UMM plugin examples

Example game plugins for [Universal Mod Manager](https://github.com/Universal-Mod-Manager/desktop) (UMM).
Each language lives on its own branch, so you can start from the one you know:

| Branch | Language | Game | Status |
|---|---|---|---|
| [`rust`](https://github.com/Universal-Mod-Manager/plugin-example/tree/rust) | Rust | SCP - Containment Breach | Available |
| [`typescript`](https://github.com/Universal-Mod-Manager/plugin-example/tree/typescript) | TypeScript | SCP - Containment Breach | Available |
| [`python`](https://github.com/Universal-Mod-Manager/plugin-example/tree/python) | Python | SCP - Containment Breach | Available |

All examples implement the same game and produce the same output, so you can compare them side by side.

## How a UMM plugin works

A plugin is a WebAssembly module loaded through [Extism](https://extism.org). UMM calls two exported
functions that take and return JSON strings (plugin API v2):

- `get_game_metadata()` describes the game: its folders, where mods are discovered and which load-order
  files the plugin writes.
- `build_load_order(input)` receives the mod list chosen in UMM (`id`, `enabled`, `priority`) and returns the
  content of those load-order files. UMM performs the writes; the plugin never touches the disk.

The host-side definitions live in
[`desktop/src-tauri/src/models/plugin_api.rs`](https://github.com/Universal-Mod-Manager/desktop/blob/develop/src-tauri/src/models/plugin_api.rs).

## Installing a plugin in UMM

A plugin is a folder containing `metadata.json`, `plugin.wasm` and `icon.png`. Copy it into UMM's plugin directory
and restart UMM:

- Windows: `%APPDATA%\com.umm.universal-mod-manager\plugins\<plugin-id>\`
- Linux: `~/.local/share/com.umm.universal-mod-manager/plugins/<plugin-id>/`

Each branch explains how to build its `plugin.wasm`.
