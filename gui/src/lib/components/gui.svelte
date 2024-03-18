<script lang="ts">
    import Info from './info.svelte'
    import Sidebar from './sidebar/sidebar.svelte'
</script>

<div class="container">
    <div class="info">
        <Info />
    </div>

    <div class="sidebar">
        <Sidebar />
    </div>
</div>

<style lang="scss">
    .container {
        width: 100%;
        height: 100%;

        display: grid;
        grid-template-columns: 1fr max-content;

        color: white;

        .info {
            height: max-content;
            width: max-content;
            justify-self: flex-end;
        }

        .sidebar {
            height: 100%;
        }
    }

    // For drag-and-drop purposes, ignore most of the gui overlay
    // when calculating the drop target (document.elementFromPoint())
    .sidebar {
        pointer-events: all;
    }
</style>
