<script lang="ts">
    import { onDestroy } from 'svelte'

    import { setTowerSelectionContext } from '../tower-selection-context'
    import Shop from './shop/shop.svelte'
    import TowerDetails from './tower-details/tower-details.svelte'

    const { selection, deleteContext } = setTowerSelectionContext()

    onDestroy(() => {
        deleteContext()
    })
</script>

{#if $selection}
    <TowerDetails selection={$selection} />
{:else}
    <Shop />
{/if}
