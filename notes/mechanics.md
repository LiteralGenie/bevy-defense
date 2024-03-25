# unit movement

Each unit travels along a sequence of contiguous tiles.
    eg . . . . . .
       . | - - - - (start)
       . | . . . .
       . | . . . .
       . | . . . .
        (end)

Each tick, the unit moves at most one tile. 
This constraint exists because...
    Units that drop buffs along their path need to do this for every tile.
    And rendering movement around corners is just easier this way.
The downside is that this puts a cap on the movement speed. If we want faster units, we have to up the tick rate, making other systems run more frequently.

How movement actually works is that the unit has a movement "accumulator", an integer counter. Each tick the accumulator is incremented by the unit's speed. If the accumulator is over whatever threshold (100), the unit is moved to the next tile and the accumulator is decremented by the threshold amount.
    eg, assuming a unit speed of 40
    tick:                1       2       3       4       5
    accumulator:        40      80      20      60       0
    tiles moved:         0       0       1       1       2
