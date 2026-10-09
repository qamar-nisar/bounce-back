# Bounce Back v1.13 — Code Reconstruction Map v3

> Scope: **Bounce Back** only. `bounceback_64b1jou2.jar`. This map is derived from the shipped class bytecode, direct method call relationships, constants, resource loading, persistence code, and input handling. It does **not** assume any relationship to Bounce Boing Voyage.

## 1. Runtime architecture

```text
                 +----------------------+
                 |    CrystalMidlet     |
                 | MIDlet/state/menus   |
                 | RMS + sound/vibration|
                 +----------+-----------+
                            |
                +-----------+-----------+
                |                       |
             creates                 creates
                v                       v
        +---------------+       +---------------+
        |       h       |       |       i       |
        | GameCanvas    |       | MenuCanvas/UI |
        | loop/input    |       | menus/text    |
        +---+-------+---+       +-------+-------+
            |       |                   |
         owns     owns              callbacks
            |       |
            v       v
        +-----+   +-----+
        |  a  |   |  g  |
        |Player|  |World |
        |physics| |tile/map/render|
        +--+--+   +--+--+
           |         |
           +----+----+
                |
                v
               c
       packed resource reader

       d -> b : replay/event list
       e    : replay/runtime state holder
       f    : repeating TimerTask
```

## 2. Class-by-class mapping

### `CrystalMidlet` — application controller
**Confidence: very high**

Evidence: extends `MIDlet`; calls `Display.getDisplay`, creates `h` and `i`, handles `CommandListener`, writes/reads RMS `CrystalRMS`, controls Nokia sound/device APIs, and owns application flags/counters.

**Mapped responsibilities**
- lifecycle: `startApp`, `pauseApp`, `destroyApp`
- main menu and screen navigation
- level selection
- options / sound / vibration state
- help / controls / records screens
- level-complete and game-over presentation
- persistent data via RMS
- sound playback (`/res/s`)
- game canvas creation/reinitialization

**Useful state meaning**
- `B:h` = active game canvas
- `D:i` = active menu/UI canvas
- `a:Display` = MIDP display
- `k:int` = high-level application/screen state discriminator (values 1..5 are explicitly used; exact labels are reconstructed below)
- `c:int[5]` = five persisted per-record/level integer slots
- `s:int` = saved integer from RMS record 1
- `y:boolean`, `A:boolean` = persisted option flags (exact original labels not recoverable)
- `q:byte`, `t:int`, `h:int`, `n:int` = end/record/session values used by result screens
- `g/f/m:boolean` and `i/v/o/C/y/A` = sound/vibration/menu/game-flow flags; exact original names are lost

**Menu/data strings recovered exactly**
- Main: `Continue`, `New Game`, `Options`, `Records`, `Help`, `Exit`
- Help: `Game Help`, `Controls`
- Controls: `Move Left: 4`, `Move Right: 6`, `Jump: 2 or 5`, `or use the joystick.`
- Result: `Points: {0}`, `Bonus: {0}`, `Total: {0}`
- Records: `Levels Completed: {0}`, `Score: {0}`, `New Record!`
- Completion: `Congratulations! You have completed the last level of Bounce Back!`
- Full completion: `Congratulations! You have completed all the levels of Bounce Back in one go!`

**High-value method mapping**

| Original | Reconstructed responsibility | Evidence |
|---|---|---|
| `startApp()` | initialize display/session and show main menu | sets display, `x=0`, `t=0`, `s=1`, calls `b()` and `b(true)` |
| `pauseApp()` | pause game/menu canvases and notify MIDlet | calls `h.h()`, `i.d()`, `notifyPaused()` |
| `destroyApp(boolean)` | persist state and terminate/clean game | calls `d(2)` and may `d(3)` |
| `b(boolean)` | open/refresh main menu | instantiates/configures `i` with six main commands |
| `a()` | level-selection screen | uses `Level {0}`, `Select`, `Back`, `New Game` |
| `i()` | options screen | toggles `Sounds On/Off`, `Done`, `Select`, `Back` |
| `j()` | records screen | uses `Records`, `Back`, `.` and saved score data |
| `e()` | help screen | uses `Game Help`, `Help`, `Select`, `Back` |
| `g()` | controls screen | uses exact control string above |
| `c()` | level-complete result | computes score/bonus/total; screen title `Level complete!` |
| `f()` | game-over result | screen title `Game Over!`; reports levels completed/score/new record |
| `a(boolean)` | final congratulations screen | selects one of two completion messages |
| `a(boolean,boolean)` | game/menu transition with game flags | stops/starts game canvas and updates display/state; exact original label uncertain |
| `d()` | persistence / game-state transition helper | calls RMS/game methods; state-dependent |
| `h()` | pause/resume/restore transition helper | interacts with `h`/replay/menu state |
| `commandAction(...)` | central menu command dispatcher | references every major screen method |
| `b()` | load `CrystalRMS` from RMS | reads 3 records |
| `d(int)` | save one RMS record | record 1=session int, 2=options+five ints, 3=run/game state |
| `a(int,int)` | load/play one sound slot | constructs `Sound` from `/res/s` packed resources |

### `h` — gameplay Canvas / orchestrator
**Confidence: very high**

Evidence: extends Nokia `FullCanvas`, implements `Runnable`; `run()` calls `a.d()`, `g()`, world camera methods, repaint/serviceRepaints; input methods map keys into a six-byte state array; persistence serializes player/world state.

**Core ownership**
- `e:a` = player/actor
- `Z:g` = primary world layer
- `A:g` = secondary/overlay world layer
- `au:d` = replay/event stream currently being consumed
- `aa:e` = alternate runtime event state holder
- `aj:f` = 50 ms repeating timer
- `b:byte[6]` = live input/event state
- `B:byte[]`, `v:int[]`, `J:int[]` = dynamic object/entity slots used by `g()`
- `f:[[I]`, `k:[[I]`, `ag:[[I]`, `s:[[B]` = per-object/runtime transform/state tables
- `ao:byte[]` = movement/animation/object type metadata

**Important methods**

| Original | Reconstructed responsibility |
|---|---|
| `h(CrystalMidlet)` | initialize canvas, font, timing arrays and application link |
| `run()` | **main 50ms gameplay tick / render orchestration**; consumes replay input, updates player, updates dynamic objects, recenters world, updates secondary layer, repaints |
| `a()` | create/start repeating 50ms timer and prime camera/world |
| `h()` | stop/cancel repeating timer and update timing state |
| `c()` | translate `b[0..3]` states into player action masks 8/4/1/2; also handles replay flags |
| `keyPressed(int)` | raw numeric-key + joystick dispatch; supports 4/6 movement and 2/5 jump; `*`/`#`/other raw codes have state/menu/debug behaviors |
| `keyReleased(int)` | clear/release movement states; replay mode ignores physical input |
| `keyRepeated(int)` | intentionally empty |
| `a(int,int)` | apply a directional action only when corresponding live input state is pressed |
| `paint(Graphics)` | top-level gameplay rendering, loading/result overlays |
| `a(Graphics)` | HUD/static UI rendering |
| `c(Graphics)` | gameplay/world compositing |
| `a(Graphics,DirectGraphics)` | direct Nokia rendering path for world layers |
| `g()` | update dynamic world objects/timers; when counters reach zero, restore tile codes 105/106/0 |
| `d()` | update moving/physics objects using `f/k/ag/s/ao` tables and player collision checks |
| `a(DataOutputStream)` | serialize active run state into RMS record 3 |
| `a(short,short,int[][])` | lookup an encoded tile/event entry at `(x,y)` from a list |
| `a(int,boolean)` / `b(int,boolean)` | decode movement/object dimension values from `ao[]` (type-dependent constants) |

### `a` — player/actor physics + animation + tile interaction
**Confidence: very high**

Evidence: constructed from `/res/b`; stores position as pixel coordinates; converts pixels to 16-pixel cells; collision probes call both `g.a(...)` and `h.a(...)`; draws the active sprite frame; modifies world tile bytes and score counters.

**Field semantics (reconstructed, not original names)**
- `D:int` = player X position in pixels
- `i:int` = player Y position in pixels
- `n:int` = X cell (`D / 16`)
- `H:int` = Y cell (`i / 16`)
- `w:Image[]` = player/actor animation images (25 sprites are loaded after a header/count byte)
- `d/u:int` = current sprite width/height
- `J/c:int` = half-width/half-height
- `m:int` = directional input bitmask (only 1/2/4/8 accepted)
- `v:g` = current collision/world layer
- `E:h`, `o:CrystalMidlet` = engine/application references
- `f:[[[Z` = collision masks for actor sprite types/groups
- `g:int` = active sprite/animation frame index

**Important methods**

| Original | Reconstructed responsibility |
|---|---|
| `a(int,int,int)` | set/reset player position + sprite type; converts position to 16px cells |
| `e()` | reset/reposition player from current cell data; resets movement/game flags |
| `k()` | enter a special state using frame `25` and application counters/sound hooks |
| `a(int)` | change active sprite frame and perform collision-aware movement/placement |
| `a(int,int)` | mutate a world tile/object at a coordinate and place/activate a dynamic slot |
| `c(int)` | set directional action bits in `m` |
| `b(int)` | clear directional action bits in `m` |
| `i()` | clear all four movement bits |
| private `a(int,int,int,int,boolean)` | collision test against other dynamic objects in `h.f/h.ag` and `h.a(...)`; can record hit index in `k` |
| private `a(int,int,boolean)` | collision test against current world layer using the active sprite mask |
| `b(int,int)` | **tile trigger/collection handler**; updates score/counters and rewrites tile codes |
| `c(int,int)` | **tile interaction/goal/hazard handler**; changes player state/game status based on tile code |
| `g()/a()/j()/l()/b()/c()` | player state/animation transitions (exact original labels lost) |
| `d(int)` | set sprite/state and enter a transient action state |
| `d()` | per-tick player physics/animation step, including world-object interaction |
| `a(Graphics)` | draw current player sprite centered using world camera offsets |

#### Tile/object code behavior directly recovered
The low 7 bits are compared; bit `0x80` is preserved as an extra per-tile flag.

**`b(x,y)` collection/score path:**
- `34` → add **200** to score-like counter `CrystalMidlet.n`; add 200 to `h.C`; mark tile consumed while retaining high bit.
- `12` → add **1000**; if `CrystalMidlet.h < 5`, increment it; mark tile consumed.
- `30` → add **2500**; mark tile consumed.
- `93,94,97,98,101,102` → add **500** and decrement `h.W`; mark/use tile state accordingly.

**`c(x,y)` interaction path:**
- `39` → enters a terminal/hazard-like state (`j=true`, `B=450`, clears some active-state flags); may trigger application state 6.
- `26` → enters alternate active-state (`C=true`, `B=450`).
- `15` → sets another active-state (`p=true`, `B=450`).
- `22` → conditionally triggers application state 5 or a delayed state (`b=550`), then calls local state-reset helpers.
- `18` → starts a state transition via `c(); l();` and application state 3 when allowed.
- `11` → starts a state transition via `c(); l();` and application state 4 when the relevant flag is active.

The exact human-readable names of these world codes should be treated as **unknown until the rendered extracted tiles/levels are correlated**.

### `g` — world layer / tile map / scrolling renderer
**Confidence: very high**

Evidence: constructor reads binary level data, loads packed image sets with `c`, constructs `R:[[B]` tile matrix and object metadata; collision method takes a boolean mask; render method uses Graphics/DirectGraphics; scrolling methods compute camera offsets; static functions are clamp/wrap helpers.

**Core field semantics**
- `R:[[B]` = **primary tile/object grid**
- `V:Image[]` = tiles/images loaded from a packed resource
- `E:Image[]` = another image frame set / rendered layer
- `N:Image` = optional base/background image
- `m:[[B]`, `O/aa/ai:byte[]` = object/auxiliary level data
- `U:int` = number of auxiliary records
- `n:int`, `ac:int` = world cell dimensions
- `ae:int` / `a/int` / `o/int` etc. = world/render offsets (exact original labels lost)
- `f:int`, `A:int`, `B:int`, `C:boolean`, `X:boolean`, `d:boolean` = rendering/tiling configuration
- `Y/P/T:int[]` = per-layer configuration/offset tables
- `s:[[[Z` = collision/mask data used by collision queries

**Important methods**
- constructor = parse level header + dimensions + image sets + tile/object tables
- `c(int,int)` = compute/update camera/scroll offsets from focus position
- `d()` = update/animate layer state and auxiliary bytes
- `a(int,int,int,int,boolean[][],boolean)` = public collision query for a rectangle/mask against the tile map
- private `a(...)` = lower-level tile/mask collision routine
- `a(Graphics)` = draw world layer
- `a(int)` = map coordinate conversion/clamp
- `b(int)` = second coordinate conversion/wrap helper
- `c()` = lazy-load/reload additional image/frame data
- `b()/b(boolean)/a(boolean)/e()/a(int,int,int,boolean,int)/a()` = internal tile animation, object update and rendering helpers
- static `a(value,max)` = clamp into `[0,max-1]`
- static `b(value,size)` = positive wrap/modulo into `[0,size-1]`

### `i` — menu/UI Canvas framework
**Confidence: very high**

- extends Nokia `FullCanvas`
- supports MIDP `Command` objects and `CommandListener`
- draws selectable text lists and optional images
- maintains selected row/index and screen mode
- receives configuration arrays from `CrystalMidlet`
- `/res/im` is loaded by its image setup path

### `c` — packed resource reader
**Confidence: exact**

Resource format recovered from bytecode:

```text
u16 BE count
repeat count times:
    u16 BE chunkLength

chunks are concatenated immediately after the table
```

The reader stores absolute offsets, can seek forward with `skip()`, and resets/reopens the resource stream if asked for an earlier chunk. `a()` returns the current chunk bytes; `c(index)` selects then reads an indexed chunk.

### `d` — replay/event stream
**Confidence: exact**

Reads `/res/r` until EOF. Every record is:

```text
int16 tick
int32 eventCode
int8  value
```

Records are stored as linked `b` nodes. During playback, event records are emitted when their `tick == playbackCounter`.

**Exact event-code mapping into the six-byte state array**
- event `8` → state byte `[0] = value`
- event `4` → state byte `[1] = value`
- event `2` → state byte `[2] = value`
- event `1` → state byte `[3] = value`
- event `20` → state byte `[4] = 1`
- event `21` → state byte `[5] = value`

### `b` — replay/event linked-list node
**Confidence: exact**

```text
short d  = tick
int   b  = eventCode
byte  c  = value
b     a  = next
```

### `e` — replay/runtime event state holder
**Confidence: medium-high**

A small state container with a current `short` counter, one `byte` mode/state, two `b` references and a boolean completion/active flag. It is used by `h` as a second replay/input state mechanism.

### `f` — repeating timer task
**Confidence: exact**

Wraps a `Runnable` in a `TimerTask`; constructor schedules itself immediately with a fixed period. `run()` calls the target runnable. Package-private `a()` cancels the task and timer.

## 3. Physical input map

Recovered from `h.keyPressed/keyReleased` + `h.c()` + `CrystalMidlet` controls string.

| Physical input | Runtime interpretation |
|---|---|
| joystick UP / game action 1 | `b[0]` pressed/released |
| joystick DOWN / game action 6 | `b[1]` pressed/released |
| joystick LEFT / game action 2 | `b[2]` pressed/released |
| joystick RIGHT / game action 5 | `b[3]` pressed/released |
| keypad `4` (raw 52) | left input |
| keypad `6` (raw 54) | right input |
| keypad `2` / `5` (raw 50/53) | jump/up input (consistent with in-game controls text) |
| `*` / `#` and several raw numeric codes | special/menu/replay/state controls, not ordinary movement |

`h.c()` converts the four live state bytes to player action-mask bits:

```text
player action bit 8 <- b[0]
player action bit 4 <- b[1]
player action bit 1 <- b[2]
player action bit 2 <- b[3]
```

## 4. Level/world representation

The important map is the byte matrix in `g.R`.

- one byte per map cell/object slot
- low 7 bits select tile/object behavior
- high bit (`0x80`) is an additional runtime flag
- player coordinates are converted using **16 px grid cells**
- `g` supplies collision geometry and rendering
- `a` supplies player-sized collision masks and player/world interactions
- `h` supplies per-object dynamic state arrays and camera composition

## 5. Persistence format (`CrystalRMS`)

Three RMS records are maintained.

### Record 1
One `int` (`CrystalMidlet.s`).

### Record 2
```text
boolean y
boolean A
int c[0]
int c[1]
int c[2]
int c[3]
int c[4]
```

This is the five-slot persistent integer array plus two options/flags.

### Record 3
Starts with one byte describing run/state status, then `h.a(DataOutputStream)` serializes:
- current application level/progress values
- timer/score-like values
- player position/state fields
- active world tile/event entries
- dynamic object positions and two-byte object state entries

The record is therefore a complete resumable game-state snapshot, not merely a high score.

## 6. Reconstructed naming proposal

These names are **new reconstruction names**, not claims about original source names.

| Original | Recommended source name |
|---|---|
| `h` | `GameCanvas` |
| `a` | `Player` |
| `g` | `WorldLayer` / `TileMap` |
| `i` | `MenuCanvas` |
| `c` | `PackedResourceReader` |
| `d` | `ReplayStream` |
| `b` | `ReplayEvent` |
| `e` | `ReplayState` |
| `f` | `RepeatingTimer` |
| `CrystalMidlet` | keep original class name |

## 7. What is now confirmed vs still uncertain

### Confirmed from bytecode
- six-byte live input array
- 16 px world grid
- player and world split (`a` vs `g`)
- 50 ms repeating timer
- `/res/r` event record structure
- packed-resource table/chunk format
- RMS record count and record 1/2 layouts
- exact control labels
- several exact tile score/interactions
- `g.R` is a mutable runtime map/object grid

### Needs one more correlation pass
- human-readable names for tile codes 11/12/15/18/22/26/30/34/39/93..106
- exact names of player physics fields (velocity vs animation/state vs friction/acceleration)
- exact names of the two world layers (`Z` and `A`)
- level file decomposition across `/res/lf`, `/res/if*`, `/res/kjmGlobal_*`, `/res/bg`, `/res/s`, `/res/tf`
- exact numeric meaning of `CrystalMidlet.k/q/h/n/t/x` beyond their observed data flow

## 8. Next source-reconstruction order

1. `a` — complete player state machine + physics, because its call sites are now mapped.
2. `g` — decode level constructor and `R` map/object encoding.
3. `h` — rebuild the game loop using the mapped `a/g` interfaces.
4. `CrystalMidlet` — replace menu/state glue around the reconstructed engine.
5. `i` — rebuild the UI framework.
6. resource decoders and compatibility layer.
7. compile-check and compare behavior/visuals against the original JAR.
