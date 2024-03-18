<script lang="ts">
    import type { TowerState } from '$lib/game/game_state'

    export let selection: TowerState
</script>

<div class="container">
    <p>{selection.id}</p>
    <p>{selection.base_damage}</p>
    <p>{selection.effective_damage}</p>
    <p>{selection.base_range}</p>
    <p>{selection.effective_range}</p>
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

        background: black;
    }
</style>
