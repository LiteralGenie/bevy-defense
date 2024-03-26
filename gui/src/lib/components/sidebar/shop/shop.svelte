<script context="module" lang="ts">
    interface Tower {
        id: number
        name: string
    }
</script>

<script lang="ts">
    import { startRound } from '$lib/game/command_handlers/start-round'
    import TowerTile from './tower-tile.svelte'

    // @todo: rust should submit this
    const towers: Tower[] = [
        {
            id: 0,
            name: 'Basic Tower'
        },
        {
            id: 1,
            name: 'Fast Tower'
        }
    ]
</script>

<div class="container">
    <div class="grid">
        {#each towers as tower}
            <div class="cell">
                <TowerTile id={tower.id} name={tower.name} />
            </div>
        {/each}
    </div>

    <div class="actions">
        <button on:click={() => startRound()}>Start Round</button>
    </div>
</div>

<style lang="scss">
    .container {
        height: 100%;
        width: 20vw;
        min-width: 200px;

        background: black;
    }

    .grid {
        display: grid;
        grid-template-columns: 50% 50%;
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

    .actions {
        padding-top: 1rem;
        width: 100%;

        display: flex;
        justify-content: center;
    }
</style>
