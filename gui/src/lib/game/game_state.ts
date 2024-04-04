import { writable } from 'svelte/store'

export class GameState {
    gold = writable(0)
    health = writable(0)

    round = writable(0)
    tick = writable(0)
    phase = writable<'INIT' | 'BUILD' | 'COMBAT'>('INIT')

    tower_types = writable<Map<number, TowerType>>(new Map())
    towers = writable<Map<bigint, TowerState>>(new Map())

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
            case 'tower_types':
                this.updateTowerType(value)
                break
            case 'towers':
                this.updateTower(value)
                break
            default:
                console.error('invalid updateState key', key, value)
        }
    }

    private updateTower(tower: TowerState) {
        this.towers.update((map) => map.set(tower.id, tower))
    }

    private updateTowerType(type: TowerType) {
        this.tower_types.update((map) => map.set(type.id, type))
    }
}

// Class stats
export interface BaseTowerType {
    id: number
    range_radius: number
}

export type OffensiveTowerType = BaseTowerType & {
    damage: number
    speed: number
}

export type TowerType = BaseTowerType | OffensiveTowerType

// Instance stats
export interface TowerStat {
    base: number
    effective: number
}

export interface BaseTowerState {
    id: bigint

    range: TowerStat
}

export type OffensiveTowerState = BaseTowerState & {
    damage: TowerStat
    attack_speed: TowerStat
}

export type TowerState = BaseTowerState | OffensiveTowerState
