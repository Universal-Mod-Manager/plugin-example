// Loads build/plugin.wasm through Extism, the runtime UMM uses, and checks both exports.
import assert from "node:assert/strict";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import createPlugin from "@extism/extism";

const WASM_PATH = fileURLToPath(new URL("../build/plugin.wasm", import.meta.url));

async function call(functionName, input = "") {
  const plugin = await createPlugin(WASM_PATH, { useWasi: true });
  try {
    const output = await plugin.call(functionName, input);
    return JSON.parse(output.text());
  } finally {
    await plugin.close();
  }
}

test("mods.ini follows priority and marks enabled mods", async () => {
  const input = {
    mods: [
      { id: "Old Textures", enabled: false, priority: 20 },
      { id: "HD Loading Screens", enabled: true, priority: 0 },
      { id: "Extra Items", enabled: true, priority: 10 },
    ],
  };

  const output = await call("build_load_order", JSON.stringify(input));

  assert.deepEqual(output.writes, [
    {
      root_id: "app_data",
      relative_path: "mods.ini",
      content: "local HD Loading Screens=1\r\nlocal Extra Items=1\r\nlocal Old Textures=0\r\n",
    },
  ]);
});

test("mods.ini is empty without mods", async () => {
  const output = await call("build_load_order", JSON.stringify({ mods: [] }));

  assert.equal(output.writes[0].content, "");
});

test("metadata discovers mod folders and declares mods.ini", async () => {
  const metadata = await call("get_game_metadata");

  assert.equal(metadata.api_version, 2);
  assert.deepEqual(metadata.mod_discovery, {
    root_id: "game",
    relative_path: "Mods",
    mode: { type: "directory_mods", required_prefix: null, metadata_file: null },
  });
  assert.deepEqual(metadata.load_order_writes, [
    { root_id: "app_data", relative_path: "mods.ini" },
  ]);
});
