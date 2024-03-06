export async function spawnTower(pos: { x: number; y: number }) {
	await window.game.pushRequest('spawn_tower', pos)
}
