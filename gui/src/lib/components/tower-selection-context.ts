import { drawRange } from '$lib/game/command_handlers/draw-range'
import type { TowerState } from '$lib/game/game_state'
import { getContext, setContext } from 'svelte'
import { derived, writable, type Readable } from 'svelte/store'

export type TowerSelectionValue = Readable<TowerState | null>

const KEY = 'tower_selection'

export function setTowerSelectionContext() {
    let idTower = writable(null)
    window.addEventListener('gameclick', handleClick)

    let value = derived([idTower, window.game.state.towers], ([id, towers]) => {
        if (id === null) {
            return null
        }

        return towers.get(id) ?? null
    })
    setContext<TowerSelectionValue>(KEY, value)

    let deleteContext = () => {
        window.removeEventListener('gameclick', handleClick)
    }
    return { selection: value, deleteContext }

    function handleClick(ev: any) {
        let id = (ev as CustomEvent).detail.tower ?? null
        idTower.set(id)
        drawRange(id)
    }
}

export function getTowerSelectionContext() {
    return getContext(KEY) as TowerSelectionValue
}
