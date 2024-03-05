# setup

cargo run --target wasm32-unknown-unknown

---

# architecture

## systems

(logic - fixed timestep)
    time system
    [...]

(rendering - per frame)
    render events
    gui events

### Game logic / state
Every N ms ("tick") the game state is updated.
Each update can trigger zero or more render events (eg "tower purchased", "enemy spawned").
Each frame the render system will consume each event and draw / update the necessary models.

### GUI
Assume the GUI is running on a separate thread (so that game logic doesn't block gui updates like drag & drop).
No shared memory (idk how, rust skill issue), GUI must poll for state like gold.
(GUI -> ECS) Each frame ECS will check for messages from GUI (eg requests for data, requests to delete unit X)
    At worst there'll be a tick of delay between the player input (eg tower purchase) and when the thing is rendered. The ECS side probably needs to do optimistic state updates and placeholder rendering.
(ECS -> GUI) Each frame ECS will emit GUI-specific render events (eg "show details for unit X")

### Validation server
Render systems will be swapped out for a validation system.
Client will notify server of some of the GUI -> ECS events.
Server will replay these events and mirror the state changes using the validation system.
    Multiplayer logic needs thinking. (Multiplayer needs real time validation + syncing btwn players. Single player can be validated at later time.)
        How to handle latency?
            Will server accept events from N ticks ago?
            Can clients be N ticks ahead? (With events marked for that future tick.)
            What if client states need syncing? Load new state and re-render from scratch?

### Misc
Probably best to run every system in sequence. Can maybe parallelize logic and rendering systems in future but KISS.
Avoid batch update optimizations, added too much complexity in panda3d impl


---

# TODO

## targeting

## rounds

## upgrades