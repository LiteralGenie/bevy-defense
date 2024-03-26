<script lang="ts">
    import { useTowerDrag } from './use-tower-drag'

    export let id: number
    export let name: string

    const { isDragging, handleDragStart, handleDrag, handleDragEnd } =
        useTowerDrag(id)
</script>

<button
    class="container"
    disabled={$isDragging}
    draggable="true"
    on:dragstart={handleDragStart}
    on:drag={handleDrag}
    on:dragend={handleDragEnd}
>
    {name}
</button>

<style lang="scss">
    .container {
        height: 100%;
        width: 100%;
        padding: 1rem;

        border: 0;
    }
</style>
