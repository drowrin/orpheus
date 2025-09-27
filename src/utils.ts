import { getCollection } from 'astro:content'
import slugify from 'slugify'

export async function getAllTags() {
  const posts = await getCollection('posts')
  const tags = posts.flatMap(p => p.data.tags)
  const uniqueTags = [...new Set(tags)]
  uniqueTags.sort()
  return new Map(
    uniqueTags.map(t => ([
      t,
      {
        posts: posts.filter(p => p.data.tags.includes(t)),
      },
    ])),
  )
}

export async function getAllSeries() {
  const posts = await getCollection('posts')
  const series = posts.flatMap(p => p.data.series)
  const uniqueSeries = [...new Set(series)]
  uniqueSeries.sort()
  return new Map(
    uniqueSeries
      .filter(s => s !== undefined)
      .map(s => ([
        slugify(s, { lower: true }),
        {
          name: s,
          posts: posts.filter(p => p.data.series === s),
        },
      ])),
  )
}
