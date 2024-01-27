<script lang="ts">
	import { Game } from '$lib/game'
	import { onMount } from 'svelte'
	import { type Readable } from 'svelte/store'

	let game: Game
	let gold: Readable<number>
	let health: Readable<number>

	function poll() {
		setTimeout(async () => {
			await game.requestGold()
			await game.requestHealth()
			poll()
		}, 500)
	}

	onMount(() => {
		game = Game.initSingleton()
		gold = game.player.gold
		health = game.player.health

		poll()
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
</style>
