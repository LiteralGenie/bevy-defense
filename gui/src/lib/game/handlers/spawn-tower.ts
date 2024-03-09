interface Position {
	x: number
	y: number
}

export async function spawnTower(pos: Position) {
	await window.game.pushRequest('spawn_tower', pos)
}
