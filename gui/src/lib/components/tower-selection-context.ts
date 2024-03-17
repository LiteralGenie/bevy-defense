import { getContext, setContext } from 'svelte'
import { derived, writable, type Readable } from 'svelte/store'

export interface TowerSelection {
    id_tower: number
}

export type TowerSelectionValue = Readable<TowerSelection | null>

const KEY = 'tower_selection'

export function setTowerSelectionContext() {
    let idTower = writable(null)
    window.addEventListener('towerclick', onTowerClick)

    let value = derived([idTower, window.game.state.towers], ([id, towers]) => {
        if (id === null) {
            return null
        }

        return towers[id] ?? null
    })
    setContext(KEY, value)

    let deleteContext = () => {
        window.removeEventListener('towerclick', onTowerClick)
    }
    return { value, deleteContext }

    function onTowerClick(ev: any) {
        let id = (ev as CustomEvent).detail.tower
        idTower.set(id)
    }
}

export function getTowerSelectionContext() {
    return getContext(KEY) as TowerSelectionValue
}
