interface ICursor {
	type: string
	position: {
		x: number
		y: number
	}
}

export interface TowerCursor extends ICursor {
	type: 'tower'
}

export type CursorType = TowerCursor

export async function drawCursor(cursor: CursorType | null) {
	return await window.game.pushRequest<boolean>('draw_cursor', cursor)
}
