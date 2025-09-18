// @ts-check
import { defineConfig } from 'astro/config'
import firstparagraph from './src/plugins/firstparagraph.mjs'
import gitdates from './src/plugins/gitdates.mjs'
import wordcount from './src/plugins/wordcount.mjs'

// https://astro.build/config
export default defineConfig({
  image: {
    layout: 'constrained',
  },
  markdown: {
    remarkPlugins: [wordcount, gitdates, firstparagraph],
  },
})
