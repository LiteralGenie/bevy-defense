export async function startRound() {
    return await window.game.pushCommand('start_round', null)
}
