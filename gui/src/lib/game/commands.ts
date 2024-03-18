export const COMMAND_TYPES = [
    'spawn_tower',
    'draw_cursor',
    'start_game',
    'start_round',
    'draw_range'
] as const

export type CommandType = (typeof COMMAND_TYPES)[number]

export interface CommandMessage<TIn = unknown, TOut = unknown, TErr = unknown> {
    type: CommandType
    resolve: (x: TOut) => void
    reject: (x: TErr) => void
    data: TIn
}
