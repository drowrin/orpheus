import type { MarkdownHeading } from 'astro'
import type { AstroComponentFactory } from 'astro/runtime/server/index.js'
import type { CollectionEntry } from 'astro:content'
import { getCollection, getEntry, render, z } from 'astro:content'

const frontmatterSchema = z.object({
  title: z.string(),
  tagline: z.string().optional(),
  series: z
    .object({
      name: z.string(),
      slug: z.string(),
    })
    .optional(),
  tags: z.array(z.string()).default([]),
  published: z.string(),
  updated: z.string().optional(),
  revisions: z.string().optional(),
  readingTime: z.object({
    text: z.string(),
    time: z.number(),
    words: z.number(),
    minutes: z.number(),
  }),
  brief: z
    .object({
      html: z.string(),
      text: z.string(),
    })
    .optional(),
  tocDepth: z.number().default(3),
})

export type PostMetadata = z.infer<typeof frontmatterSchema>

export type Post = PostMetadata & {
  id: string
  Content: AstroComponentFactory
  headings: MarkdownHeading[]
}

async function transformPost(p: CollectionEntry<'posts'>) {
  const { Content, headings, remarkPluginFrontmatter } = await render(p)

  const fm = frontmatterSchema.parse(remarkPluginFrontmatter)

  return {
    ...fm,
    id: p.id,
    Content,
    headings,
  } satisfies Post
}

export async function getPost(id: string) {
  const entry = await getEntry('posts', id)
  if (entry === undefined) {
    return undefined
  }
  return transformPost(entry)
}

export const getAllPosts = (() => {
  let allPosts: Post[] | undefined

  return async function () {
    if (allPosts !== undefined) {
      return allPosts
    }

    const collection = await getCollection('posts')

    allPosts = await Promise.all(collection.map(transformPost))

    allPosts.sort((a, b) => b.published.localeCompare(a.published))

    return allPosts
  }
})()

export async function getAllTags() {
  const posts = await getAllPosts()
  const tags = posts.flatMap(p => p.tags)
  const uniqueTags = [...new Set(tags)]
  uniqueTags.sort()
  return new Map(
    uniqueTags.map(t => ([
      t,
      {
        posts: posts.filter(p => p.tags.includes(t)),
      },
    ])),
  )
}

export async function getAllSeries() {
  const posts = await getAllPosts()
  const series = posts.flatMap(p => p.series)
  const uniqueSeries = [...new Set(series)]
  uniqueSeries.sort()
  return new Map(
    uniqueSeries
      .filter(s => s !== undefined)
      .map(s => ([
        s.slug,
        {
          name: s.name,
          posts: posts.filter(p => p.series?.slug === s.slug),
        },
      ])),
  )
}
