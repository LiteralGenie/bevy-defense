<script context="module" lang="ts">
	interface Tower {
		name: string
	}
</script>

<script lang="ts">
	import { drawCursor } from '$lib/game/handlers/draw-cursor'
	import { spawnTower } from '$lib/game/handlers/spawn-tower'

	import { writable } from 'svelte/store'

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

	let isDragging = writable(false)
	let isValidDropPos = writable(false)

	async function handleDragStart(ev: DragEvent) {
		isDragging.set(true)
		isValidDropPos.set(false)

		// Delete default cursor ghost, wasm will handle this
		ev.dataTransfer?.setDragImage(document.createElement('image'), 0, 0)
	}

	async function handleDrag(ev: DragEvent) {
		const { clientX: x, clientY: y } = ev

		// Ignore final drag event which because it's always at (0,0) for some reason
		if (x === 0 && y === 0) {
			return
		}

		isValidDropPos.set(
			// Note: Cursor coordinates need to be provided by the JS-side
			//       because Bevy can't read cursor position while an HTML element is mid-drag
			await drawCursor({
				type: 'tower',
				position: { x, y }
			})
		)
	}

	async function handleDragEnd(ev: DragEvent) {
		isDragging.set(false)

		if (isValidDropPos) {
			const { clientX: x, clientY: y } = ev

			// Only spawn tower if the drop target is the canvas and not the GUI
			const target = document.elementFromPoint(x, y)
			if (target?.id === 'game-canvas') {
				await spawnTower({ x, y })
			}

			// To prevent flickering, don't despawn cursor until after tower is created
			await drawCursor(null)
		}
	}
</script>

<div class="container">
	<div class="grid">
		{#each towers as tower}
			<button
				class="cell"
				disabled={$isDragging}
				draggable="true"
				on:dragstart={handleDragStart}
				on:drag={handleDrag}
				on:dragend={handleDragEnd}
			>
				{tower.name}
			</button>
		{/each}
	</div>
</div>

<style lang="scss">
	.container {
		height: 100%;

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
