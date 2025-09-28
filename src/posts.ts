import type { AstroComponentFactory } from 'astro/runtime/server/index.js'
import type { CollectionEntry } from 'astro:content'
import type { ReadTimeResults } from 'reading-time'
import { execSync } from 'node:child_process'
import fs from 'node:fs'
import { getCollection, getEntry, render } from 'astro:content'
import getReadingTime from 'reading-time'
import slugify from 'slugify'

export interface Post {
  id: string
  title: string
  tagline?: string
  series?: {
    name: string
    slug: string
  }
  tags: string[]
  published: string
  updated?: string
  revisions?: string
  readingTime: ReadTimeResults
  brief?: string
  Content: AstroComponentFactory
}

function gitHistory(p: CollectionEntry<'posts'>) {
  let published
  try {
    published = p.data.published
      ?? execSync(
        `git log --follow --diff-filter=A --format="%as" -1 -- "${p.filePath}"`,
      ).toString().trim()
  }
  catch {
    published = new Date().toISOString().split('T')[0]
  }

  const content = fs.readFileSync(p.filePath!, 'utf8')

  const fmLines = 2 + content.split('---')[1].split('\n').length
  const totalLines = content.split('\n').length

  let revisions
  let updated
  try {
    revisions = execSync(
      `git log --diff-filter=M --format="%as: %s" --no-patch "-L${fmLines},${totalLines}:${p.filePath}"`,
    ).toString().trim()
    updated = revisions.split('\n')[0].split(':')[0]
  }
  catch {}

  return { published, revisions, updated }
}

function getBrief(p: CollectionEntry<'posts'>) {
  if (p.data.brief !== undefined) {
    return `<p>${p.data.brief}</p>`
  }
  return p.rendered!.html.substring(
    p.rendered!.html.indexOf('<p'),
    p.rendered!.html.indexOf('</p>'),
  )
}

async function transformPost(p: CollectionEntry<'posts'>) {
  const { Content } = await render(p)

  return {
    ...gitHistory(p),
    title: p.data.title,
    tagline: p.data.tagline,
    tags: p.data.tags,
    series: p.data.series === undefined
      ? undefined
      : {
          name: p.data.series,
          slug: slugify(p.data.series, { lower: true }),
        },
    id: p.id,
    Content,
    readingTime: getReadingTime(p.body!),
    brief: getBrief(p),
  } satisfies Post
}

export async function getPost(id: string) {
  const entry = await getEntry('posts', id)
  if (entry === undefined) {
    return undefined
  }
  return transformPost(entry)
}

export async function getAllPosts() {
  const collection = await getCollection('posts')

  const posts: Post[] = await Promise.all(collection.map(transformPost))

  posts.sort((a, b) => b.published.localeCompare(a.published))

  return posts
}

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
