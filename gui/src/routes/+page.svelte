<script lang="ts">
	import Info from '$lib/components/info.svelte'
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

{#if game}
	<div class="container">
		<div class="info">
			<Info />
		</div>

		<div class="sidebar">
			<Sidebar />
		</div>
	</div>
{/if}

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
	.container {
		position: absolute;
		height: 100%;
		width: 100%;
	}

	// For drag-and-drop purposes, ignore most of the gui overlay
	// when calculating the drop target (document.elementFromPoint())
	.container {
		pointer-events: none;

		.sidebar {
			pointer-events: all;
		}
	}

	.container {
		display: grid;
		grid-template-columns: 1fr max(200px, 20vw);

		color: white;

		.info {
			height: max-content;
			width: max-content;
			justify-self: flex-end;
		}

		.sidebar {
			height: 100%;
		}
	}
</style>
