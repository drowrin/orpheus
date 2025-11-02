<script lang="ts">
	import { page } from '$app/state'
	import { version } from '$app/environment'

	interface Props {
		title: string
		description?: string
		ogImage?: string
	}

	const { title, description, ogImage }: Props = $props()

	const origin = import.meta.env.DEV ? page.url.origin : 'https://drowrin.com'

	const ogImageURL = ogImage ? new URL(ogImage, origin) : undefined
	const ogURL = new URL(page.url, origin)
</script>

<svelte:head>
	<title>{title} - Drowrin.com</title>
	{#if description}
		<meta name="description" content={description} />
	{/if}

	<meta property="og:title" content={title} />
	<meta property="og:type" content="website" />
	<meta property="og:url" content={ogURL.toString()} />
	{#if description}
		<meta property="og:description" content={description} />
	{/if}
	{#if ogImageURL}
		<meta property="og:image" content={ogImageURL.toString()} />
	{/if}

	<meta name="robots" content="index, follow" />
	<!-- <link rel="sitemap" href="/sitemap-index.xml" /> -->

	<meta name="color-scheme" content="dark light" />

	<meta name="generator" content={`SvelteKit ${version}`} />
</svelte:head>
