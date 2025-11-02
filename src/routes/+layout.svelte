<script lang="ts">
	import favicon from '$lib/assets/favicon.svg'
	import '@fontsource/atkinson-hyperlegible-next/400'
	import '@fontsource/fira-code/400'
	import 'modern-normalize/modern-normalize.css'
	import '$lib/css/global.css'
	import Nav from '$lib/components/Nav.svelte'

	let { children } = $props()
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<header>
	<Nav />
</header>
<main>
	{@render children()}
</main>
