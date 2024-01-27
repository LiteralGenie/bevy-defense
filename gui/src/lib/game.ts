import loadWasm from '$lib/assets/wasm/bevy-defense'
import { writable } from 'svelte/store'

export const EVENT_TYPES = ['gold', 'health'] as const

export type EventType = (typeof EVENT_TYPES)[number]

export interface Event<T = any> {
	type: EventType
	resolve: (text: unknown) => void
	reject: (text: unknown) => void
	data: T
}

export class Game {
	player = {
		gold: writable(0),
		health: writable(0)
	}

	fromGui = [] as Event[]

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

	private async pushRequest<TOut, TIn = any>(type: EventType, data: TIn) {
		let resolve, reject
		const p = new Promise((rs, rj) => {
			resolve = rs
			reject = rj
		})
		this.fromGui.push({
			type: 'gold',
			resolve: resolve as any,
			reject: reject as any,
			data
		})
		return (await p) as TOut
	}

	async requestGold() {
		const gold = await this.pushRequest<number>('gold', null)
		this.player.gold.set(gold)
	}

	async requestHealth() {
		const health = await this.pushRequest<number>('health', null)
		this.player.health.set(health)
	}
}
