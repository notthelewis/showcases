import { defineConfig } from "tsup";

export default defineConfig({
    clean: true,
    entry: ["src/**/*.ts"],
    format: ["cjs"],
    minify: true,
    env: { NODE_ENV: "production" },
});
