<script lang="ts">
	import TOC from './TOC.svelte'
	import { resolve } from '$app/paths'
	import { type Post } from '$lib/blog'

	interface Props {
		post: Post
		card?: boolean
	}

	const { post, card }: Props = $props()
</script>

<article class="post" data-pagefind-body>
	<hgroup>
		{#if card}
			<h3>
				<a href={resolve(`/posts/${post.slug}/`)}>{post.title}</a>
			</h3>
		{:else}
			<h1 data-pagefind-meta="title">{post.title}</h1>
		{/if}

		{#if post.tagline}
			<p>{post.tagline}</p>
		{/if}

		<small class="tags">
			{#if post.tags.length > 0 || post.series}
				<ul data-pagefind-meta={post.tags.length > 0 ? `tags:${post.tags.join(', ')}` : null}>
					{#if post.series}
						<li
							data-pagefind-filter={`series:${post.series.slug}`}
							data-pagefind-meta={`series:${post.series.name}`}
						>
							<a href={resolve(`/posts/series/${post.series.slug}/`)}>{post.series.name}</a>
						</li>
					{/if}

					{#if post.tags.length > 0}
						{#each post.tags as string[] as tag (tag)}
							<li data-pagefind-filter={`tag:${tag}`}>
								<a href={resolve(`/posts/tags/${tag}/`)}>{'#' + tag}</a>
							</li>
						{/each}
					{/if}
				</ul>
			{/if}
			<span data-pagefind-meta="published" data-pagefind-sort="published">{post.published}</span>
			-
			<span data-pagefind-meta="reading time">{post.readingTime.text}</span>
			-
			<span data-pagefind-meta="word count" data-pagefind-sort="word count">
				{post.readingTime.words} words
			</span>
		</small>
	</hgroup>
	<hr />
	<prose>
		{#if card}
			{@html post.brief}
		{:else}
			<TOC headings={post.headings} tocDepth={post.tocDepth} />
			<post.Content />
		{/if}
	</prose>
</article>

<style>
	article.post {
		hgroup {
			margin-inline: 1rem;
			margin-block: 0;
		}

		.tags ul {
			display: flex;
			flex-wrap: wrap;
			column-gap: 0.25rem;
			list-style: none;
			padding: 0;
			margin-block: 0;
		}

		hr {
			margin-block: 0.5rem;
			margin-inline: 1rem;
		}
	}
</style>
