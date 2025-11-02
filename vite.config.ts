import { sveltekit } from '@sveltejs/kit/vite'
import { enhancedImages } from '@sveltejs/enhanced-img'
import devtoolsJson from 'vite-plugin-devtools-json'
import viteYaml from '@modyfi/vite-plugin-yaml'
import { defineConfig } from 'vite'

export default defineConfig({
	plugins: [
		enhancedImages(),
		sveltekit(),
		devtoolsJson(),
		viteYaml(),
		// {
		// 	name: 'orpheus-content-watcher-plugin',
		// 	async hotUpdate(ctx) {
		// 		if (ctx.file.endsWith('.md')) {
		// 			console.log(`${ctx.file} changed!`)
		// 			return ctx.modules.filter((module) => {
		// 				console.log(module.file)
		// 				const targets = ['']
		// 				return module.file ? targets.includes(module.file) : false
		// 			})
		// 		}
		// 		return []
		// 	},
		// },
	],
})
