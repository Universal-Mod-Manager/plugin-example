/**
 * Example UMM game plugin for SCP - Containment Breach (v1.3.12, the free Steam release).
 *
 * UMM loads the compiled plugin.wasm through Extism and calls the two exported
 * functions. Both exchange JSON strings (plugin API v2), read with
 * `Host.inputString()` and returned with `Host.outputString()`. The plugin never
 * touches the disk: it only describes the game and returns file contents, and
 * UMM performs the writes.
 */

const GAME_ROOT_ID = "game";
const APP_DATA_ROOT_ID = "app_data";
const MODS_INI = "mods.ini";
/** The game identifies a mod from its `Mods` folder as `local <folder name>`. */
const LOCAL_MOD_ID_PREFIX = "local ";

// Output of `get_game_metadata`.

interface GameMetadata {
  api_version: number;
  name: string;
  executable: string;
  path_roots: GamePathRoot[];
  mod_discovery: ModDiscovery;
  load_order_writes: LoadOrderWriteTarget[];
}

/** A folder the user points UMM to. Every other path is relative to a root. */
interface GamePathRoot {
  id: string;
  name: string;
  description: string;
}

/** Where UMM looks for mods and how it recognises them. */
interface ModDiscovery {
  root_id: string;
  relative_path: string;
  mode: ModDiscoveryMode;
}

/**
 * Each folder is a mod and its folder name is the mod id. UMM can read a JSON
 * `metadata_file`, but the game describes mods in `info.ini`, so UMM shows
 * folder names.
 */
interface ModDiscoveryMode {
  type: "directory_mods";
  required_prefix: string | null;
  metadata_file: string | null;
}

/** A file the plugin is allowed to write. `build_load_order` may only return these. */
interface LoadOrderWriteTarget {
  root_id: string;
  relative_path: string;
}

// Input and output of `build_load_order`.

interface ModEntry {
  id: string;
  enabled: boolean;
  priority: number;
}

interface BuildLoadOrderInput {
  mods: ModEntry[];
}

interface BuildLoadOrderOutput {
  writes: GameFileWrite[];
}

interface GameFileWrite {
  root_id: string;
  relative_path: string;
  content: string;
}

export function get_game_metadata() {
  Host.outputString(JSON.stringify(gameMetadata()));
}

export function build_load_order() {
  const input: BuildLoadOrderInput = JSON.parse(Host.inputString());
  const output: BuildLoadOrderOutput = {
    writes: [
      {
        root_id: APP_DATA_ROOT_ID,
        relative_path: MODS_INI,
        content: formatModsIni(input.mods),
      },
    ],
  };
  Host.outputString(JSON.stringify(output));
}

function gameMetadata(): GameMetadata {
  return {
    api_version: 2,
    name: "SCP - Containment Breach",
    executable: "SCP - Containment Breach.exe",
    path_roots: [
      {
        id: GAME_ROOT_ID,
        name: "SCP - Containment Breach install folder",
        description: "Folder containing the game executable and the Mods directory.",
      },
      {
        id: APP_DATA_ROOT_ID,
        name: "SCP - Containment Breach data folder",
        description:
          "Folder containing options.ini and mods.ini: " +
          "%APPDATA%\\Undertow Games\\SCP - Containment Breach.",
      },
    ],
    mod_discovery: {
      root_id: GAME_ROOT_ID,
      relative_path: "Mods",
      mode: { type: "directory_mods", required_prefix: null, metadata_file: null },
    },
    load_order_writes: [{ root_id: APP_DATA_ROOT_ID, relative_path: MODS_INI }],
  };
}

/**
 * One `local <folder>=1` (enabled) or `=0` (disabled) line per mod, lowest
 * priority first. The game loads mods in this order, so later mods override
 * earlier ones. Lines end with CRLF, as when the game saves the file itself.
 */
function formatModsIni(mods: ModEntry[]): string {
  return [...mods]
    .sort((a, b) => a.priority - b.priority)
    .map((gameMod) => `${LOCAL_MOD_ID_PREFIX}${gameMod.id}=${gameMod.enabled ? 1 : 0}\r\n`)
    .join("");
}
