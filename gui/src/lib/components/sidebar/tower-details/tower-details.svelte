<script lang="ts">
    import type { TowerState } from '$lib/game/game_state'

    export let selection: TowerState
</script>

<div class="container">
    <p>{selection.id}</p>

    <div>
        damage: {selection.base_damage} -> {selection.effective_damage}
    </div>

    <div>
        range: {selection.base_range} -> {selection.effective_range}
    </div>

    <div>
        speed: {selection.base_attack_speed} -> {selection.effective_attack_speed}
    </div>
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
