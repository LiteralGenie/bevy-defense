import { getContext, setContext } from 'svelte'

export interface TowerSelection {
    id_tower: number
}

export type TowerSelectionValue = TowerSelection | null

const KEY = 'tower_selection'

export function setTowerSelectionContext() {
    setContext(KEY, null as TowerSelectionValue)
}

export function getTowerSelectionContext() {
    return getContext(KEY) as TowerSelectionValue
}
