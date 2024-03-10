<script lang="ts">
	let gold = window.game.state.gold
	let health = window.game.state.health
</script>

<div class="container">
	<span>Gold: {$gold}</span>
	<span>Health: {$health}</span>
</div>

<style lang="scss">
	.container {
		padding: 1rem;

		display: flex;
		flex-flow: column;
		gap: 0.25rem;
	}
</style>
