// @ts-check
import compress from 'astro-compress'
import { defineConfig, fontProviders } from 'astro/config'
import firstparagraph from './src/plugins/firstparagraph.mjs'
import gitdates from './src/plugins/gitdates.mjs'

import wordcount from './src/plugins/wordcount.mjs'

// https://astro.build/config
export default defineConfig({
  prefetch: {
    defaultStrategy: 'viewport',
    prefetchAll: true,
  },

  image: {
    layout: 'constrained',
    responsiveStyles: true,
  },

  markdown: {
    remarkPlugins: [wordcount, gitdates, firstparagraph],
    shikiConfig: {
      themes: {
        light: 'catppuccin-latte',
        dark: 'catppuccin-mocha',
      },
    },
  },

  experimental: {
    fonts: [
      {
        provider: fontProviders.fontsource(),
        name: 'Atkinson Hyperlegible Next',
        cssVariable: '--font-hyperlegible',
        fallbacks: ['Tahoma', 'system-ui', 'sans-serif'],
        subsets: ['latin'],
      },
      {
        provider: fontProviders.fontsource(),
        name: 'Fira Code',
        cssVariable: '--font-hyperlegible-mono',
        fallbacks: ['Courier New', 'monospace'],
        subsets: ['latin'],
        featureSettings: 'liga on',
      },
    ],
  },

  integrations: [
    compress({
      CSS: false,
      HTML: false,
    }),
  ],
})
