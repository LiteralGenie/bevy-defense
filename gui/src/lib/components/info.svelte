<script lang="ts">
    let { gold, health, tick } = window.game.state
</script>

<div class="container">
    <span>Gold: {$gold}</span>
    <span>Health: {$health}</span>
    <span>Tick: {$tick}</span>
</div>

<style lang="scss">
    .container {
        padding: 1rem;

        display: flex;
        flex-flow: column;
        gap: 0.25rem;
    }
</style>
