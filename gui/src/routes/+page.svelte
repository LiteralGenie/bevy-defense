<script lang="ts">
    import Gui from '$lib/components/gui.svelte'
    import { Game } from '$lib/game/game'
    import { onMount } from 'svelte'

    let game: Game

    onMount(async () => {
        // @fixme: 	Getting the wasm / canvas to work with hot-reloading is tricky
        //         		https://github.com/bevyengine/bevy/discussions/11619
        //       	which is why the canvas is outside any svelte components and this onMount skips re-initing Game
        // 	     	But this also means any changes to the Rust code / Game.ts requires manually refreshing the page
        game = window.game ?? (await Game.initSingleton())
    })
</script>

{#if game}
    <div class="container">
        <Gui />
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

    // For drag-and-drop purposes, make the gui invisible by default
    .container {
        pointer-events: none;
    }
</style>
