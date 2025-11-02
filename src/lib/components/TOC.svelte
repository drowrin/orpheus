<script lang="ts">
	import type { MarkdownHeading } from '$lib/blog'
	interface Props {
		headings: MarkdownHeading[]
		tocDepth: number
	}
	const { headings, tocDepth }: Props = $props()

	const displayHeadings = headings.filter((h) => h.depth <= tocDepth)

	type TocEntry = {
		slug: string
		text: string
		depth: number
		children: TocEntry[]
	}

	const toc: TocEntry[] = []
	const stack = []

	for (const h of displayHeadings) {
		const node: TocEntry = { ...h, children: [] }

		while (stack.length && stack[stack.length - 1].depth >= node.depth) {
			stack.pop()
		}

		if (stack.length === 0) {
			toc.push(node)
		} else {
			const parent = stack[stack.length - 1]
			parent.children.push(node)
		}

		stack.push(node)
	}
</script>

{#snippet tocNodes(nodes: TocEntry[])}
	{#if nodes.length > 0}
		<ol>
			{#each nodes as n (n.slug)}
				<li>
					<a href="#{n.slug}">{n.text}</a>
					{@render tocNodes(n.children)}
				</li>
			{/each}
		</ol>
	{/if}
{/snippet}

<details open>
	<summary>Table of Contents</summary>
	<nav aria-label="Table of contents">
		{@render tocNodes(toc)}
	</nav>
</details>
<hr />

<style>
	details {
		margin-inline: 1rem;
		margin-block: 0.5rem;
		ol {
			list-style: none;
			padding-inline-start: 1rem;
		}
		nav > ol:first-of-type {
			margin-block-start: 0.5rem;
		}
	}

	@media (min-width: 128ch) {
		hr {
			display: none;
		}

		details {
			width: 12rem;
			padding: 0.5rem;
			position: fixed;
			transform: translateX(calc(-100% - 2rem));
			top: var(--navbar-height);
			margin-block: 0;
		}
	}
</style>
