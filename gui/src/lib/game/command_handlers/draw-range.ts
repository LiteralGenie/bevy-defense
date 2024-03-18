export async function drawRange(id_tower: bigint | null) {
    // @todo: How to convert from bigint to (whatever int type) on the rust side?
    return await window.game.pushCommand(
        'draw_range',
        id_tower ? Number(id_tower) : null
    )
}
