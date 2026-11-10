// Bundles the TypeScript sources into a single CommonJS file for extism-js.
const esbuild = require("esbuild");

esbuild.build({
  entryPoints: ["src/index.ts"],
  outdir: "build",
  bundle: true,
  format: "cjs", // extism-js only accepts CommonJS
  target: ["es2020"], // QuickJS, the engine inside the plugin, supports up to ES2020
});
