<script lang="ts">
	let { children } = $props()
</script>

<svelte:head>
	<title>AI File Processing</title>
</svelte:head>

<main>
	{@render children()}
</main>

<style>
	:global(body) {
		font-family:
			-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
		margin: 0;
		padding: 20px;
		background: #f5f5f5;
	}

	main {
		max-width: 800px;
		margin: 0 auto;
	}
</style>
