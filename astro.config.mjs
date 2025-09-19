// @ts-check
import { defineConfig, fontProviders } from 'astro/config'
import firstparagraph from './src/plugins/firstparagraph.mjs'
import gitdates from './src/plugins/gitdates.mjs'
import wordcount from './src/plugins/wordcount.mjs'

// https://astro.build/config
export default defineConfig({
  image: {
    layout: 'constrained',
    responsiveStyles: true,
  },
  markdown: {
    remarkPlugins: [wordcount, gitdates, firstparagraph],
  },
  experimental: {
    fonts: [
      {
        provider: fontProviders.fontsource(),
        name: 'Atkinson Hyperlegible Next',
        cssVariable: '--font-hyperlegible',
      },
      {
        provider: fontProviders.fontsource(),
        name: 'Atkinson Hyperlegible Mono',
        cssVariable: '--font-hyperlegible-mono',
      },
    ],
  },
})
