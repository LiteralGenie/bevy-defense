import loadWasm from '$lib/assets/wasm/bevy-defense'
import { writable, type Writable } from 'svelte/store'

export const REQUEST_TYPES = ['spawn_tower'] as const
export type RequestType = (typeof REQUEST_TYPES)[number]

export interface Request<T = any> {
	type: RequestType
	resolve: (text: unknown) => void
	reject: (text: unknown) => void
	data: T
}

export interface GameState {
	gold: Writable<number>
	health: Writable<number>
}

export class Game {
	state: GameState = {
		gold: writable(0),
		health: writable(0)
	}

	guiRequests = [] as Request[]

	static initSingleton() {
		const game = new Game()
		;(window as any).game = game

		loadWasm().catch((error) => {
			if (
				!error.message.startsWith(
					"Using exceptions for control flow, don't mind me. This isn't actually an error!"
				)
			) {
				throw error
			}
		})

		return game
	}

	updateState(key: keyof GameState, value: any) {
		switch (key) {
			case 'gold':
				this.state.gold.set(value)
				break
			case 'health':
				this.state.health.set(value)
				break
			default:
				console.error('invalid updateState key', key, value)
		}
	}

	private async pushRequest<TOut, TIn = any>(type: RequestType, data: TIn) {
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

	async spawnTower() {
		await this.pushRequest('spawn_tower', null)
	}
}
