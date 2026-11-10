// Functions exported to UMM. extism-js reads this file to generate the Wasm exports.
declare module "main" {
  export function get_game_metadata(): I32;
  export function build_load_order(): I32;
}
