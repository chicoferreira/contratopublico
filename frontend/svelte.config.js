import adapter from "svelte-adapter-bun";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { mdsvex } from "mdsvex";

/** @type {import('@sveltejs/kit').Config} */
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
};

export default config;
