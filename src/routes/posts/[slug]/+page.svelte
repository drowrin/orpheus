<script lang="ts">
	import Meta from '$lib/components/Meta.svelte'
	import Post from '$lib/components/Post.svelte'

	const { data } = $props()
</script>

<Meta title={data.title} description={data.brief?.text} />

<Post post={data} />
