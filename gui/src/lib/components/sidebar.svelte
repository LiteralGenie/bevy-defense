<script context="module" lang="ts">
	interface Tower {
		name: string
	}
</script>

<script lang="ts">
	const towers: Tower[] = [
		{
			name: '1'
		},
		{
			name: '2'
		},
		{
			name: '3'
		},
		{
			name: '4'
		}
	]
</script>

<div class="container">
	<div class="grid">
		{#each towers as tower}
			<button class="cell">{tower.name}</button>
		{/each}
	</div>
</div>

<style lang="scss">
	.container {
		background: black;
	}

	.grid {
		display: grid;
		grid-template-columns: 50% 50%;
	}

	.cell {
		padding: 1rem;

		text-align: center;
	}

	// Borders
	.grid {
		border: 1px solid white;
		border-right: 0;
		border-top: 0;

		.cell {
			border: 1px solid white;
			border-left: 0;
			border-bottom: 0;
		}
	}
</style>
