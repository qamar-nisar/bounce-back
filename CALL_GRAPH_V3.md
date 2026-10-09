# Bounce Back v1.13 — reconstructed project call graph

This is the high-level project-class graph recovered from direct bytecode calls. It intentionally collapses library/API calls so the gameplay architecture is readable.

## Application/control

`CrystalMidlet.b(boolean)` → `i` (main menu)

`CrystalMidlet.a()` → `i` (level select)

`CrystalMidlet.i()` → `i` (options)

`CrystalMidlet.j()` → `i` (records)

`CrystalMidlet.e()` → `i` (help)

`CrystalMidlet.g()` → `i` (controls)

`CrystalMidlet.c()` → `h` + `i` (level-complete scoring/result)

`CrystalMidlet.f()` → `h` + `i` (game-over result)

`CrystalMidlet.a(boolean)` → `i` (completion message)

`CrystalMidlet.commandAction(...)` → dispatches among `b(boolean)`, `a()`, `i()`, `j()`, `e()`, `g()`, `c()`, `f()`, `a(boolean)`, `d()`, `h()`.

## Gameplay

`h(CrystalMidlet)` → `CrystalMidlet.a(int,int)` + resource/game initialization

`h.run()` → `d.a(byte[])` → `CrystalMidlet.e()` (when replay reaches end)

`h.run()` → `a.d()` → `h.d()`/dynamic object update → `h.g()` → `g.c(int,int)` → `g.d()` → render/repaint

`h.a()` → `f(Runnable,50)` + camera/world initialization

`h.c()` → `a.c(int)` / `a.b(int)`

`h.keyPressed(int)` → `a.l()` / `a.b()` / replay/menu state transitions

`h.a(int,int)` → `a.c(int)` / `a.b(int)` when an input slot is active

## Player/world interaction

`a(int,int,int)` → `h.Z` (`g` world layer)

`a(int)` → `a(int,int,boolean)` + collision-aware movement

`a(int,int,boolean)` → `g.a(...)` collision query + dynamic-object collision query in `h`

`a(int,int,int,int,boolean)` → `h.a(...)` dynamic-rectangle collision

`a.d()` → reads/modifies `h.s`, `h.f`, `h.k`, `h.ag`, `h.ao` and world object state

`a.b(int,int)` → modifies `g.R`, score/counters in `CrystalMidlet`/`h`

`a.c(int,int)` → modifies `g.R`, player state and invokes `CrystalMidlet` result/state methods

`a.a(Graphics)` → `g.a(int)` / `g.b(int)` for camera-relative player drawing

## World/resource

`g(...)` constructor → `c(...)` packed-resource reader → `Image.createImage(...)`

`g.a(...)` → tile/mask collision

`g.c(int,int)` → camera/scroll coordinate update

`g.d()` → world/object animation/update

`g.a(Graphics)` → world-layer rendering

`c(index)` → `c.a(index)` + `c.a()`; packed chunk extraction

## Replay

`d.a()` → reads `/res/r` → creates `b` nodes → linked list

`d.a(b)` → append `b` node

`d.a(byte[])` → consumes current `b` nodes and writes state into `h.b[0..5]`

`b` → next-node link

`e` → second event/replay state container used by `h`

## Timing

`f` constructor → schedules a `TimerTask` immediately at a fixed period

`f.run()` → target `Runnable.run()` (in gameplay, `h.run()`)
