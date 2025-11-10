import rss from '@astrojs/rss'
import { getAllPosts } from '@/posts'

export async function GET(context) {
  const posts = await getAllPosts()
  return rss({
    title: 'Drowrin\'s Blog',
    description: 'I review games, talk about TTRPGs, and write about the tech I\'m using',
    site: context.site,
    items: posts.map((post) => {
      return {
        title: post.title,
        pubDate: post.published,
        description: post.brief?.text,
        link: `/posts/${post.id}/`,
      }
    }),
  })
}
