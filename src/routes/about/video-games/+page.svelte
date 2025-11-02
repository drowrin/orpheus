<script lang="ts">
	import Meta from '$lib/components/Meta.svelte'
	import data from './data.yml'
</script>

<Meta title="About Me: Video Games" />

<h1>About Me: Video Games</h1>
<div>
	{#each Object.entries(data) as category (category[0])}
		<game-category>
			<category-content>
				{#each category[1] as entry (entry.name)}
					<img src={entry.image} alt={'Cover image for ' + entry.name} title={entry.name} />
					{#if entry.label}
						<p>{entry.label}</p>
					{/if}
				{/each}
			</category-content>
			<category-label>
				<p>{category[0]}</p>
			</category-label>
		</game-category>
	{/each}
</div>

<style>
	div {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}
	game-category {
		max-width: 10rem;
		width: 100%;
		border: 1px solid var(--color-text);
		border-radius: 0.25rem;
		overflow: hidden;
		position: relative;
	}
	category-label {
		height: 3rem;
		width: 100%;
		display: flex;
		align-items: center;
		padding: 0.3rem 0.5rem 0.5rem;
		p {
			text-align: center;
			margin: 0;
			width: 100%;
		}
	}
	category-content {
		position: relative;
		display: block;
		width: 100%;
		max-width: 100%;
		height: 0;
		padding-block-end: 133%;
		img {
			position: absolute;
			max-width: 10rem;
			transition: clip-path 0.25s ease-in-out;
		}
		img:nth-of-type(2) {
			clip-path: polygon(100% 0%, 100% 100%, 0% 100%);
			&:hover,
			&:active {
				clip-path: polygon(100% -100%, 100% 100%, -100% 100%);
			}
			img:hover + &,
			img:active + & {
				clip-path: polygon(100% 100%, 100% 100%, 100% 100%);
			}
		}
		p {
			position: absolute;
			bottom: 0.125rem;
			left: 0.25rem;
			margin: 0;
			padding: 0.1rem 0.2rem 0.2rem;
			background-color: var(--color-base);
			border-radius: 0.25rem;
		}
	}
</style>
