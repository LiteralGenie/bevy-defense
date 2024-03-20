import { drawCursor } from '$lib/game/command_handlers/draw-cursor'
import { spawnTower } from '$lib/game/command_handlers/spawn-tower'
import { get, writable } from 'svelte/store'

export function useTowerDrag() {
    let isDragging = writable(false)
    let isValidDropPos = writable(false)

    async function handleDragStart(ev: DragEvent) {
        isDragging.set(true)
        isValidDropPos.set(false)

        // Delete default cursor ghost, wasm will handle this
        ev.dataTransfer?.setDragImage(document.createElement('image'), 0, 0)
    }

    async function handleDrag(ev: DragEvent) {
        const { clientX: x, clientY: y } = ev

        // Ignore final drag event because it's always at (0,0) for some reason
        if (x === 0 && y === 0) {
            return
        }

        isValidDropPos.set(
            // Note: Cursor coordinates need to be provided by the JS-side
            //       because Bevy can't read cursor position while an HTML element is mid-drag
            await drawCursor({
                id_tower: 0,
                position: { x, y }
            })
        )
    }

    async function handleDragEnd(ev: DragEvent) {
        isDragging.set(false)

        if (get(isValidDropPos)) {
            const { clientX: x, clientY: y } = ev

            // Only spawn tower if the drop target is the canvas and not the GUI
            const target = document.elementFromPoint(x, y)
            if (target?.id === 'game-canvas') {
                await spawnTower({ x, y })
            }
        }

        // To prevent flickering, don't despawn cursor until after tower is created
        await drawCursor(null)
    }

    return {
        isDragging,
        isValidDropPos,
        handleDragStart,
        handleDrag,
        handleDragEnd
    }
}
