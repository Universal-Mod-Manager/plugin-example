//! Example UMM game plugin for SCP - Containment Breach (v1.3.12, the free Steam release).
//!
//! UMM loads this crate as a WebAssembly module through Extism and calls the two
//! functions marked `#[plugin_fn]`. Both exchange JSON strings (plugin API v2).
//! The plugin never touches the disk: it only describes the game and returns
//! file contents, and UMM performs the writes.

use extism_pdk::*;
use serde::{Deserialize, Serialize};

const GAME_ROOT_ID: &str = "game";
const APP_DATA_ROOT_ID: &str = "app_data";
const MODS_INI: &str = "mods.ini";
/// The game identifies a mod from its `Mods` folder as `local <folder name>`.
const LOCAL_MOD_ID_PREFIX: &str = "local ";

// Output of `get_game_metadata`.

#[derive(Serialize)]
struct GameMetadata {
    api_version: u32,
    name: String,
    executable: String,
    path_roots: Vec<GamePathRoot>,
    mod_discovery: ModDiscovery,
    load_order_writes: Vec<LoadOrderWriteTarget>,
}

/// A folder the user points UMM to. Every other path is relative to a root.
#[derive(Serialize)]
struct GamePathRoot {
    id: String,
    name: String,
    description: String,
}

/// Where UMM looks for mods and how it recognises them.
#[derive(Serialize)]
struct ModDiscovery {
    root_id: String,
    relative_path: String,
    mode: ModDiscoveryMode,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ModDiscoveryMode {
    /// Each folder is a mod and its folder name is the mod id. UMM can read a
    /// JSON `metadata_file`, but the game describes mods in `info.ini`, so UMM
    /// shows folder names.
    DirectoryMods {
        required_prefix: Option<String>,
        metadata_file: Option<String>,
    },
}

/// A file the plugin is allowed to write. `build_load_order` may only return these.
#[derive(Serialize)]
struct LoadOrderWriteTarget {
    root_id: String,
    relative_path: String,
}

// Input and output of `build_load_order`.

#[derive(Deserialize)]
struct ModEntry {
    id: String,
    enabled: bool,
    priority: u32,
}

#[derive(Deserialize)]
struct BuildLoadOrderInput {
    mods: Vec<ModEntry>,
}

#[derive(Serialize)]
struct BuildLoadOrderOutput {
    writes: Vec<GameFileWrite>,
}

#[derive(Serialize)]
struct GameFileWrite {
    root_id: String,
    relative_path: String,
    content: String,
}

#[plugin_fn]
pub fn get_game_metadata() -> FnResult<String> {
    Ok(serde_json::to_string(&game_metadata())?)
}

#[plugin_fn]
pub fn build_load_order(input: String) -> FnResult<String> {
    let input: BuildLoadOrderInput = serde_json::from_str(&input)?;
    let output = BuildLoadOrderOutput {
        writes: vec![GameFileWrite {
            root_id: APP_DATA_ROOT_ID.to_string(),
            relative_path: MODS_INI.to_string(),
            content: format_mods_ini(&input.mods),
        }],
    };
    Ok(serde_json::to_string(&output)?)
}

fn game_metadata() -> GameMetadata {
    GameMetadata {
        api_version: 2,
        name: "SCP - Containment Breach".to_string(),
        executable: "SCP - Containment Breach.exe".to_string(),
        path_roots: vec![
            GamePathRoot {
                id: GAME_ROOT_ID.to_string(),
                name: "SCP - Containment Breach install folder".to_string(),
                description: "Folder containing the game executable and the Mods directory."
                    .to_string(),
            },
            GamePathRoot {
                id: APP_DATA_ROOT_ID.to_string(),
                name: "SCP - Containment Breach data folder".to_string(),
                description: "Folder containing options.ini and mods.ini: \
                              %APPDATA%\\Undertow Games\\SCP - Containment Breach."
                    .to_string(),
            },
        ],
        mod_discovery: ModDiscovery {
            root_id: GAME_ROOT_ID.to_string(),
            relative_path: "Mods".to_string(),
            mode: ModDiscoveryMode::DirectoryMods {
                required_prefix: None,
                metadata_file: None,
            },
        },
        load_order_writes: vec![LoadOrderWriteTarget {
            root_id: APP_DATA_ROOT_ID.to_string(),
            relative_path: MODS_INI.to_string(),
        }],
    }
}

/// One `local <folder>=1` (enabled) or `=0` (disabled) line per mod, lowest
/// priority first. The game loads mods in this order, so later mods override
/// earlier ones. Lines end with CRLF, as when the game saves the file itself.
fn format_mods_ini(mods: &[ModEntry]) -> String {
    let mut sorted: Vec<&ModEntry> = mods.iter().collect();
    sorted.sort_by_key(|game_mod| game_mod.priority);

    let mut content = String::new();
    for game_mod in sorted {
        let active = if game_mod.enabled { 1 } else { 0 };
        content.push_str(&format!(
            "{LOCAL_MOD_ID_PREFIX}{}={active}\r\n",
            game_mod.id
        ));
    }
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mod_entry(id: &str, enabled: bool, priority: u32) -> ModEntry {
        ModEntry {
            id: id.to_string(),
            enabled,
            priority,
        }
    }

    #[test]
    fn mods_ini_follows_priority_and_marks_enabled_mods() {
        let mods = vec![
            mod_entry("Old Textures", false, 20),
            mod_entry("HD Loading Screens", true, 0),
            mod_entry("Extra Items", true, 10),
        ];

        assert_eq!(
            format_mods_ini(&mods),
            "local HD Loading Screens=1\r\nlocal Extra Items=1\r\nlocal Old Textures=0\r\n"
        );
    }

    #[test]
    fn mods_ini_is_empty_without_mods() {
        assert_eq!(format_mods_ini(&[]), "");
    }

    #[test]
    fn metadata_discovers_mod_folders_and_declares_mods_ini() {
        let metadata = serde_json::to_value(game_metadata()).unwrap();

        assert_eq!(
            metadata["mod_discovery"],
            serde_json::json!({
                "root_id": "game",
                "relative_path": "Mods",
                "mode": { "type": "directory_mods", "required_prefix": null, "metadata_file": null }
            })
        );
        assert_eq!(
            metadata["load_order_writes"],
            serde_json::json!([{ "root_id": "app_data", "relative_path": "mods.ini" }])
        );
    }
}
