import loadWasm from '$lib/assets/wasm/bevy-defense'
import type { CommandMessage, CommandType } from './commands'
import { GameState } from './game_state'

declare global {
    interface Window {
        game: Game
    }
}

export class Game {
    state = new GameState()

    // Requests from the gui for the engine
    pending_commands = [] as CommandMessage[]

    /**
     * Add Game instance to window, for GUI (js) <-> Engine (wasm) communication
     */
    static async initSingleton() {
        const game = new Game()
        window.game = game

        await loadWasm().catch((error) => {
            if (
                !error.message.startsWith(
                    "Using exceptions for control flow, don't mind me. This isn't actually an error!"
                )
            ) {
                throw error
            }
        })

        game.pushCommand('start_game', null)

        return game
    }

    /**
     * Engine calls this to update game state
     */
    updateState(key: keyof GameState, value: any) {
        this.state.update(key, value)
    }

    /**
     * Engine calls this on interaction with the 3d models in the canvas
     */
    dispatchEvent(name: string, detail: any) {
        console.debug('dispatching event', name, detail)
        window.dispatchEvent(new CustomEvent(name, { detail }))
    }

    /**
     * GUI calls this to send message to Engine
     */
    async pushCommand<TOut, TIn = any>(type: CommandType, data: TIn) {
        console.debug('Submitting wasm request', type, data)

        let resolve, reject
        const p = new Promise((rs, rj) => {
            resolve = rs
            reject = rj
        })
        this.pending_commands.push({
            type,
            resolve: resolve as any,
            reject: reject as any,
            data
        })
        return (await p) as TOut
    }
}
