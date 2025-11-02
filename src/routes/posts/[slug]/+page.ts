import { allPostSlugs, getPost } from '$lib/blog'
import type { EntryGenerator } from './$types'

export const load = async ({ params }) => {
	return await getPost(params.slug)
}

export const entries: EntryGenerator = () => {
	console.log(allPostSlugs.map((slug) => ({ slug })))
	return allPostSlugs.map((slug) => ({ slug }))
}
