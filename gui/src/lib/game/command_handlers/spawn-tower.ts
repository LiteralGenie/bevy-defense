interface Position {
    x: number
    y: number
}

export async function spawnTower(id: number, position: Position) {
    await window.game.pushCommand('spawn_tower', { id, position })
}
