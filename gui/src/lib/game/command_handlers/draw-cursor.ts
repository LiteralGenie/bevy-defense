export interface TowerCursor {
    id_tower: number
    position: {
        x: number
        y: number
    }
}

export async function drawCursor(cursor: TowerCursor | null) {
    return await window.game.pushCommand<boolean>('draw_cursor', cursor)
}
