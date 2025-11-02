import * as z from 'zod'
import type { Component } from 'svelte'
import { error } from '@sveltejs/kit'

const markdownHeadingSchema = z.object({
	depth: z.number(),
	slug: z.string(),
	text: z.string(),
})

const frontmatterSchema = z.object({
	title: z.string(),
	slug: z.string(),
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
	headings: z.array(markdownHeadingSchema),
})

export interface ImportedPost {
	default: Component
	metadata: Record<string, string>
}

export type PostMetaData = z.infer<typeof frontmatterSchema>
export type MarkdownHeading = z.infer<typeof markdownHeadingSchema>

export type Post = {
	Content: Component
} & PostMetaData

const importPosts = import.meta.glob<ImportedPost>('../content/posts/**/*.md')

const slugToPath = Object.fromEntries(
	Object.keys(importPosts).map((filePath) => {
		const slug = filePath.split('/').at(-1)!.split('.').at(0)!
		return [slug, filePath]
	}),
)

export const allPostSlugs = Object.keys(slugToPath)

export async function getPost(slug: string) {
	const resolver = importPosts[slugToPath[slug]]
	if (resolver === undefined) {
		error(404, `${slug} not found`)
	}
	const module = await resolver()

	return {
		Content: module.default,
		...frontmatterSchema.parse(module.metadata),
	} satisfies Post
}
