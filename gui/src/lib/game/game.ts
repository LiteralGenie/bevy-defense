import loadWasm from '$lib/assets/wasm/bevy-defense'
import { writable, type Writable } from 'svelte/store'

declare global {
    interface Window {
        game: Game
    }
}

export const REQUEST_TYPES = [
    'spawn_tower',
    'draw_cursor',
    'start_game',
    'start_round'
] as const
export type RequestType = (typeof REQUEST_TYPES)[number]

export interface Request<TIn = unknown, TOut = unknown, TErr = unknown> {
    type: RequestType
    resolve: (x: TOut) => void
    reject: (x: TErr) => void
    data: TIn
}

export interface GameState {
    gold: Writable<number>
    health: Writable<number>

    round: Writable<number>
    tick: Writable<number>
    phase: Writable<'INIT' | 'BUILD' | 'COMBAT'>
}

export class Game {
    state: GameState = {
        gold: writable(0),
        health: writable(0),

        round: writable(0),
        tick: writable(0),
        phase: writable('INIT')
    }

    guiRequests = [] as Request[]

    /**
     * Add Game instance to window, for WASM <-> JS communication
     */
    static initSingleton() {
        const game = new Game()
        ;(window as any).game = game

        loadWasm()
            .catch((error) => {
                if (
                    !error.message.startsWith(
                        "Using exceptions for control flow, don't mind me. This isn't actually an error!"
                    )
                ) {
                    throw error
                }
            })
            .then(() => game.pushRequest('start_game', null))

        return game
    }

    /**
     * WASM-land calls this to update game state
     */
    updateState(key: keyof GameState, value: any) {
        switch (key) {
            case 'gold':
                this.state.gold.set(value)
                break
            case 'health':
                this.state.health.set(value)
                break
            case 'round':
                this.state.round.set(value)
                break
            case 'tick':
                this.state.tick.set(value)
                break
            case 'phase':
                this.state.phase.set(value)
                break
            default:
                console.error('invalid updateState key', key, value)
        }
    }

    /**
     * WASM-land calls this on interaction with the 3d models in the canvas
     */
    dispatchEvent(name: string, detail: any) {
        console.log('dispatching event', name, detail)
    }

    /**
     * GUI calls this to send message to WASM-land
     */
    async pushRequest<TOut, TIn = any>(type: RequestType, data: TIn) {
        console.debug('Submitting wasm request', type, data)

        let resolve, reject
        const p = new Promise((rs, rj) => {
            resolve = rs
            reject = rj
        })
        this.guiRequests.push({
            type,
            resolve: resolve as any,
            reject: reject as any,
            data
        })
        return (await p) as TOut
    }
}
