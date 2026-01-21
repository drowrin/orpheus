import fs from 'node:fs'
import mdx from '@astrojs/mdx'
import sitemap from '@astrojs/sitemap'
import svelte from '@astrojs/svelte'
import rehypeFigure from '@microflash/rehype-figure'
import ViteYaml from '@modyfi/vite-plugin-yaml'
import opengraphImages from 'astro-opengraph-images'
import { defineConfig, fontProviders } from 'astro/config'
import rehypeShiftHeading from 'rehype-shift-heading'
import remarkAttributes from 'remark-attributes'
import { ogRender } from './src/ogRender'
import rehypeBrief from './src/plugins/brief'
import rehypeDetailsBlock from './src/plugins/details-block'
import rehypeEmdash from './src/plugins/emdash'
import remarkGitHistory from './src/plugins/git-history'
import rehypeQuoteCitation from './src/plugins/quote-citation'
import remarkReadingTime from './src/plugins/reading-time'
import rehypeRemoveNewlines from './src/plugins/remove-newlines'
import remarkSeriesSlug from './src/plugins/series-slug'

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
      remarkGitHistory,
      remarkReadingTime,
      remarkSeriesSlug,
    ],
    rehypePlugins: [
      rehypeQuoteCitation,
      rehypeBrief,
      rehypeFigure,
      rehypeRemoveNewlines,
      rehypeEmdash,
      rehypeDetailsBlock,
      [rehypeShiftHeading, { shift: 1 }],
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
  }), mdx(), svelte()],
})
