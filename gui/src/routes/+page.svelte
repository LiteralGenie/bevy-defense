<script lang="ts">
	import Sidebar from '$lib/components/sidebar.svelte'
	import { Game } from '$lib/game/game'
	import { onMount } from 'svelte'
	import { type Readable } from 'svelte/store'

	let game: Game
	let gold: Readable<number>
	let health: Readable<number>

	onMount(() => {
		game = window.game ?? Game.initSingleton()
		gold = game.state.gold
		health = game.state.health
	})
</script>

<div id="gui">
	{#if game}
		<div class="container">
			<div class="info">
				<span>Gold: {$gold}</span>
				<span>Health: {$health}</span>
			</div>

			<Sidebar />
		</div>
	{/if}
</div>

<style lang="scss">
	// Full-screen the game canvas
	:global(body) {
		margin: 0;
		overflow: hidden;
	}
	:global(.game-canvas-container) {
		height: 100vh;
		width: 100vw;

		:global(#game-canvas) {
			height: 100% !important;
			width: 100% !important;
		}
	}

	// Make the gui an overlay
	#gui {
		height: 100%;
		width: 100%;

		position: absolute;
	}

	.container {
		height: 100%;

		display: grid;
		grid-template-columns: 1fr max(200px, 20vw);

		color: white;

		.info {
			height: max-content;
			width: max-content;
			justify-self: flex-end;
			padding: 1rem;

			display: flex;
			flex-flow: column;
			align-items: flex-end;
			gap: 0.25rem;
		}
	}
</style>
