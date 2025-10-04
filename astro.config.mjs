// @ts-check
import { defineConfig, fontProviders } from 'astro/config'
import emdash from './src/plugins/emdash'
import quoteCitation from './src/plugins/quote-citation'
import removeNewlines from './src/plugins/remove-newlines'
import spoilers from './src/plugins/spoilers'

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
    smartypants: true,
    shikiConfig: {
      themes: {
        light: 'catppuccin-latte',
        dark: 'catppuccin-mocha',
      },
    },
    rehypePlugins: [emdash, removeNewlines, spoilers, quoteCitation],
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
})
