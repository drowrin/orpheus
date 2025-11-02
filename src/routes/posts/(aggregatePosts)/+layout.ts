import { allPostSlugs, getPost, type Post } from '$lib/blog'

interface Tag {
	name: string
	posts: Post[]
}

interface Series {
	name: string
	slug: string
	posts: Post[]
}

export const load = async () => {
	// console.log('loading all posts... this might take a bit')
	const posts: Record<string, Post> = Object.fromEntries(
		(await Promise.all(allPostSlugs.map(getPost))).map((p) => [p.slug, p]),
	)

	// console.log('got posts!')

	const tags: Record<string, Tag> = {}
	const series: Record<string, Series> = {}

	for (const post of Object.values(posts)) {
		for (const t of post.tags) {
			if (!(t in tags)) {
				tags[t] = {
					name: t,
					posts: [],
				}
			}

			tags[t].posts.push(post)
		}

		if (post.series) {
			if (!(post.series.slug in series)) {
				series[post.series.slug] = {
					...post.series,
					posts: [],
				}
			}

			series[post.series.slug].posts.push(post)
		}
	}

	return { tags, series, posts }
}
