import fs from 'node:fs'
import sitemap from '@astrojs/sitemap'
import rehypeFigure from '@microflash/rehype-figure'
import ViteYaml from '@modyfi/vite-plugin-yaml'
import opengraphImages from 'astro-opengraph-images'
import { defineConfig, fontProviders } from 'astro/config'
import remarkAttributes from 'remark-attributes'
import { ogRender } from './src//ogRender'
import detailsBlock from './src/plugins/details-block'
import emdash from './src/plugins/emdash'
import quoteCitation from './src/plugins/quote-citation'
import removeNewlines from './src/plugins/remove-newlines'

import mdx from '@astrojs/mdx';

export default defineConfig({
  site: 'https://drowrin.com',

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
    remarkPlugins: [
      remarkAttributes,
    ],
    rehypePlugins: [
      emdash,
      removeNewlines,
      quoteCitation,
      detailsBlock,
      rehypeFigure,
    ],
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

  vite: {
    plugins: [
      ViteYaml(),
    ],
  },

  integrations: [sitemap(), opengraphImages({
    options: {
      fonts: [
        {
          name: 'Atkinson Hyperlegible Next',
          weight: 400,
          style: 'normal',
          data: fs.readFileSync(
            'node_modules/@fontsource/atkinson-hyperlegible-next/files/atkinson-hyperlegible-next-latin-400-normal.woff',
          ),
        },
      ],
    },
    render: ogRender,
  }), mdx()],
})