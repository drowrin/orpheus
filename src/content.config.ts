import { glob } from 'astro/loaders'
import { defineCollection } from 'astro:content'

const posts = defineCollection({
  loader: glob({
    base: 'src/content/posts',
    pattern: '**/*.{md,mdx}',
    generateId: ({ entry }) => entry.split('/').at(-1)!.split('.').at(0)!,
  }),
})

export const collections = { posts }
