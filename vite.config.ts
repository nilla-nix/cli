import { defineConfig } from "vite";

export default defineConfig({
	resolve: {
		alias: {
			"~": "/src",
		},
	},
	build: {
		lib: {
			name: "NillaCli",
			entry: "src/index.ts",
			formats: ["cjs"],
			fileName: "index",
		},
		rollupOptions: {
			external: (source, importer, isResolved) => {
				return source.startsWith("node:");
			},
		}
	},
});
