interface Position {
    x: number
    y: number
}

export async function spawnTower(pos: Position) {
    await window.game.pushCommand('spawn_tower', pos)
}
