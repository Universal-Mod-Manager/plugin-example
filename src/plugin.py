"""Example UMM game plugin for SCP - Containment Breach (v1.3.12, the free Steam release).

UMM loads the compiled plugin.wasm through Extism and calls the two exported
functions. Both exchange JSON strings (plugin API v2), read with
`extism.input_str()` and returned with `extism.output_str()`. The plugin never
touches the disk: it only describes the game and returns file contents, and
UMM performs the writes.
"""

import json

import extism

GAME_ROOT_ID = "game"
APP_DATA_ROOT_ID = "app_data"
MODS_INI = "mods.ini"
# The game identifies a mod from its `Mods` folder as `local <folder name>`.
LOCAL_MOD_ID_PREFIX = "local "


@extism.plugin_fn
def get_game_metadata():
    output_json(game_metadata())


@extism.plugin_fn
def build_load_order():
    load_order_input = json.loads(extism.input_str())
    output_json(
        {
            "writes": [
                {
                    "root_id": APP_DATA_ROOT_ID,
                    "relative_path": MODS_INI,
                    "content": format_mods_ini(load_order_input["mods"]),
                }
            ]
        }
    )


def output_json(value):
    # Compact JSON, like the Rust and TypeScript versions.
    extism.output_str(json.dumps(value, separators=(",", ":"), ensure_ascii=False))


def game_metadata():
    return {
        "api_version": 2,
        "name": "SCP - Containment Breach",
        "executable": "SCP - Containment Breach.exe",
        "path_roots": [
            {
                "id": GAME_ROOT_ID,
                "name": "SCP - Containment Breach install folder",
                "description": "Folder containing the game executable and the Mods directory.",
            },
            {
                "id": APP_DATA_ROOT_ID,
                "name": "SCP - Containment Breach data folder",
                "description": "Folder containing options.ini and mods.ini: "
                "%APPDATA%\\Undertow Games\\SCP - Containment Breach.",
            },
        ],
        # Each folder is a mod and its folder name is the mod id. UMM can read a JSON
        # metadata_file, but the game describes mods in info.ini, so UMM shows folder names.
        "mod_discovery": {
            "root_id": GAME_ROOT_ID,
            "relative_path": "Mods",
            "mode": {"type": "directory_mods", "required_prefix": None, "metadata_file": None},
        },
        "load_order_writes": [{"root_id": APP_DATA_ROOT_ID, "relative_path": MODS_INI}],
    }


def format_mods_ini(mods):
    """One `local <folder>=1` (enabled) or `=0` (disabled) line per mod, lowest
    priority first. The game loads mods in this order, so later mods override
    earlier ones. Lines end with CRLF, as when the game saves the file itself.
    """
    return "".join(
        f"{LOCAL_MOD_ID_PREFIX}{game_mod['id']}={1 if game_mod['enabled'] else 0}\r\n"
        for game_mod in sorted(mods, key=lambda game_mod: game_mod["priority"])
    )
