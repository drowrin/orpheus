import { mdsvex } from 'mdsvex'
import adapter from '@sveltejs/adapter-static'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

import rehypeDetailsBlock from './src/plugins/details-block.js'
import rehypeEmdash from './src/plugins/emdash.js'
import rehypeQuoteCitation from './src/plugins/quote-citation.js'
import rehypeRemoveNewlines from './src/plugins/remove-newlines.js'
import rehypeHeadings from './src/plugins/headings.js'
import rehypeFigure from '@microflash/rehype-figure'
import rehypeBrief from './src/plugins/brief.js'
import rehypeGitHistory from './src/plugins/git-history.js'
import rehypeTitleSlug from './src/plugins/title-slug.js'
import rehypeSeriesSlug from './src/plugins/series-slug.js'
import rehypeReadingTime from './src/plugins/reading-time.js'

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: [
		vitePreprocess(),
		mdsvex({
			extensions: ['.md'],
			remarkPlugins: [],
			rehypePlugins: [
				rehypeQuoteCitation,
				rehypeHeadings,
				rehypeFigure,
				rehypeRemoveNewlines,
				rehypeEmdash,
				rehypeDetailsBlock,
				rehypeBrief,
				rehypeGitHistory,
				rehypeTitleSlug,
				rehypeReadingTime,
				rehypeSeriesSlug,
			],
			smartypants: false,
			layout: './src/lib/mdsvex-layout.svelte',
		}),
	],
	kit: {
		paths: {
			assets: 'https://drowrin.com',
		},
		adapter: adapter({
			pages: 'dist',
			assets: 'dist',
		}),
		typescript: {
			config: (config) => {
				config.compilerOptions.types = ['@modyfi/vite-plugin-yaml/modules']
			},
		},
	},
	extensions: ['.svelte', '.md'],
}

export default config
