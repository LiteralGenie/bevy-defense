<script lang="ts">
    let { gold, health, round, tick, phase } = window.game.state
</script>

<div class="container">
    <span>Gold: {$gold}</span>
    <span>Health: {$health}</span>
    <span>Round: {$round}</span>
    <span>Tick: {$tick}</span>
    <span>Phase: {$phase}</span>
</div>

<style lang="scss">
    .container {
        padding: 1rem;

        display: flex;
        flex-flow: column;
        gap: 0.25rem;
    }
</style>
