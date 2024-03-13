export async function startRound() {
    return await window.game.pushRequest('start_round', null)
}
