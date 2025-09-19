import { glob } from 'astro/loaders'
import { defineCollection, z } from 'astro:content'

const posts = defineCollection({
  loader: glob({
    base: 'src/content/posts',
    pattern: '**/*.md',
    generateId: ({ entry }) => entry.split('/').at(-1)!.split('.').at(0)!,
  }),
  schema: z.object({
    title: z.string(),
    tagline: z.string().optional(),
    series: z.string().optional(),
    tags: z.array(z.string()).default([]),
  }),
})

export const collections = { posts }
