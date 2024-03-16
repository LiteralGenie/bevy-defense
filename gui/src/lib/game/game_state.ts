import { writable } from 'svelte/store'

export class GameState {
    gold = writable(0)
    health = writable(0)

    round = writable(0)
    tick = writable(0)
    phase = writable<'INIT' | 'BUILD' | 'COMBAT'>('INIT')

    towers = writable<Record<number, TowerState>>({})

    update(key: keyof GameState, value: any) {
        switch (key) {
            case 'gold':
                this.gold.set(value)
                break
            case 'health':
                this.health.set(value)
                break
            case 'round':
                this.round.set(value)
                break
            case 'tick':
                this.tick.set(value)
                break
            case 'phase':
                this.phase.set(value)
                break
            case 'towers':
                this.updateTowerState(value)
                break
            default:
                console.error('invalid updateState key', key, value)
        }
    }

    private updateTowerState(tower: TowerState) {
        this.towers.update((current) => ({ ...current, [tower.id]: tower }))
    }
}

interface TowerState {
    id: number

    damage: number
}
