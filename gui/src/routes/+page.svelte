<script lang="ts">
	import { Game } from '$lib/game'
	import { onMount } from 'svelte'
	import { type Readable } from 'svelte/store'

	let game: Game
	let gold: Readable<number>
	let health: Readable<number>

	let pollId: number

	onMount(() => {
		// @fixme: This breaks hot-reloading for some reason
		//   	   The wasm probably isn't reloading despite the canvas being destroyed / recreated
		// 		   Probably need to move the canvas + Game's init logic to app.html
		game = Game.initSingleton()
		gold = game.state.gold
		health = game.state.health
		
		setTimeout(() => {
			game.spawnTower()
		}, 3000)
	})
</script>

{#if game}
	Gold: {$gold}
	<br />
	Health: {$health}
{/if}

<div class="game-canvas-container">
	<canvas id="game-canvas"></canvas>
</div>

<style lang="scss">
	.game-canvas-container {
		width: 90vw;
		height: 90vh;
	}

	#game-canvas {
		width: 100% !important;
		height: 100% !important;
	}
</style>
