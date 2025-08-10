import type { Config } from "@sveltejs/kit";
import adapter from "svelte-adapter-bun";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { mdsvex } from "mdsvex";

const config = {
  extensions: [".svelte", ".md"],
  preprocess: [
    vitePreprocess(),
    mdsvex({
      extensions: [".md"],
      layout: import.meta.dirname + "/src/components/markdown/MarkdownLayout.svelte",
    }),
  ],

  kit: {
    adapter: adapter(),
  },
} as Config;

export default config;
