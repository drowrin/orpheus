<script>
	import { resolve } from '$app/paths'
</script>

<nav>
	<ul>
		<li><a href={resolve('/')}>Home</a></li>
		<li><a href={resolve('/posts')}>Posts</a></li>
		<li><a href={resolve('/projects')}>Projects</a></li>
	</ul>
</nav>

<style>
	nav {
		position: fixed;
		width: 100%;
		top: 0;
		left: 0;
		background-color: var(--color-crust);
		z-index: 90;
	}

	ul {
		margin: auto;
		padding: 0;
		width: 100%;
		max-width: var(--main-width);
		display: flex;
		list-style-type: none;
	}

	li {
		margin: 0.5rem 0 0.5rem 1rem;
	}

	a {
		text-decoration: none;
		color: var(--color-lavender);
	}
</style>
