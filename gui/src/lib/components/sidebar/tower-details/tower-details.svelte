<script lang="ts">
    import type { TowerState } from '$lib/game/game_state'

    export let selection: TowerState
</script>

<div class="container">
    <p>{selection.id}</p>

    {#if 'damage' in selection}
        <div>
            damage: {selection.damage.base} -> {selection.damage.effective}
        </div>
    {/if}

    <div>
        range: {selection.range.base} -> {selection.range.effective}
    </div>

    {#if 'attack_speed' in selection}
        <div>
            speed: {selection.attack_speed.base} -> {selection.attack_speed
                .effective}
        </div>
    {/if}
</div>

<style lang="scss">
    .container {
        height: 100%;
        width: 20vw;
        min-width: 200px;
        padding: 1rem;
        box-sizing: border-box;

        display: flex;
        flex-flow: column;
        gap: 1rem;

        background: black;
    }
</style>
