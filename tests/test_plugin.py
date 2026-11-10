# Loads build/plugin.wasm through Extism, the runtime UMM uses, and checks both exports.
import json
import unittest
from pathlib import Path

import extism

WASM_PATH = Path(__file__).resolve().parent.parent / "build" / "plugin.wasm"


def call(function_name, plugin_input=""):
    with extism.Plugin(WASM_PATH.read_bytes(), wasi=True) as plugin:
        return json.loads(plugin.call(function_name, plugin_input))


class PluginTest(unittest.TestCase):
    def test_mods_ini_follows_priority_and_marks_enabled_mods(self):
        plugin_input = {
            "mods": [
                {"id": "Old Textures", "enabled": False, "priority": 20},
                {"id": "HD Loading Screens", "enabled": True, "priority": 0},
                {"id": "Extra Items", "enabled": True, "priority": 10},
            ]
        }

        output = call("build_load_order", json.dumps(plugin_input))

        self.assertEqual(
            output["writes"],
            [
                {
                    "root_id": "app_data",
                    "relative_path": "mods.ini",
                    "content": "local HD Loading Screens=1\r\nlocal Extra Items=1\r\nlocal Old Textures=0\r\n",
                }
            ],
        )

    def test_mods_ini_is_empty_without_mods(self):
        output = call("build_load_order", json.dumps({"mods": []}))

        self.assertEqual(output["writes"][0]["content"], "")

    def test_metadata_discovers_mod_folders_and_declares_mods_ini(self):
        metadata = call("get_game_metadata")

        self.assertEqual(metadata["api_version"], 2)
        self.assertEqual(
            metadata["mod_discovery"],
            {
                "root_id": "game",
                "relative_path": "Mods",
                "mode": {"type": "directory_mods", "required_prefix": None, "metadata_file": None},
            },
        )
        self.assertEqual(metadata["load_order_writes"], [{"root_id": "app_data", "relative_path": "mods.ini"}])


if __name__ == "__main__":
    unittest.main()
