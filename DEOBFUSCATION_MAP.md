# Bounce Back v1.13 — Clean Deobfuscation

The source files in `src/` contain clean Java declarations with role-based names. All one-letter game classes were renamed. High-confidence members use semantic names; uncertain members use deterministic `state...` / `unknownMethod...` names so no unsupported guess is presented as fact.

## CrystalMidlet → CrystalMidlet

### Fields
- `D` → `menuCanvas` — high
- `B` → `gameCanvas` — high
- `a` → `display` — high
- `c` → `savedValues` — high
- `p` → `sounds` — high
- `e` → `currentSound` — high
- `k` → `screenState` — high
- `r` → `stringArray01` — conservative
- `z` → `stringArray02` — conservative
- `E` → `stringArray03` — conservative
- `l` → `stringArray04` — conservative
- `w` → `stringArray05` — conservative
- `d` → `stringArray06` — conservative
- `b` → `stringArray07` — conservative
- `C` → `stateFlag01` — conservative
- `i` → `stateFlag02` — conservative
- `v` → `stateFlag03` — conservative
- `q` → `stateByte01` — conservative
- `s` → `stateInt01` — conservative
- `t` → `stateInt02` — conservative
- `x` → `stateInt03` — conservative
- `o` → `stateFlag04` — conservative
- `y` → `stateFlag05` — conservative
- `A` → `stateFlag06` — conservative
- `g` → `stateFlag07` — conservative
- `f` → `stateFlag08` — conservative
- `m` → `stateFlag09` — conservative
- `u` → `stateFlag10` — conservative
- `j` → `stateFlag11` — conservative
- `h` → `stateInt04` — conservative
- `n` → `stateInt05` — conservative

### Methods
- `CrystalMidlet()` → `CrystalMidlet`
- `startApp()` → `unknownMethod01`
- `pauseApp()` → `unknownMethod02`
- `destroyApp(boolean)` → `unknownMethod03`
- `a(int)` → `switchScreen`
- `b(boolean)` → `showMainMenu`
- `a()` → `showLevelSelect`
- `i()` → `showOptions`
- `j()` → `showRecords`
- `e()` → `showHelp`
- `g()` → `showControls`
- `c()` → `showLevelResult`
- `a(boolean)` → `showCompletionMessage`
- `f()` → `showGameOver`
- `a(boolean,boolean)` → `startGame`
- `d()` → `loadPreferences`
- `h()` → `savePreferences`
- `commandAction(javax.microedition.lcdui.Command,javax.microedition.lcdui.Displayable)` → `commandAction`
- `b()` → `resetGame`
- `d(int)` → `setLevel`
- `a(java.lang.String,int)` → `formatString`
- `a(int,int)` → `updateScore`
- `b(int)` → `updateBonus`
- `c(int)` → `setCompletedLevels`

## a → PlayerController

### Fields
- `o` → `midlet` — high
- `E` → `gameCanvas` — high
- `v` → `world` — high
- `D` → `x` — high
- `i` → `y` — high
- `H` → `tileX` — high
- `n` → `tileY` — high
- `d` → `spriteWidth` — high
- `u` → `spriteHeight` — high
- `J` → `halfWidth` — high
- `c` → `halfHeight` — high
- `g` → `spriteIndex` — high
- `w` → `sprites` — high
- `y` → `sharedSprites` — high
- `s` → `stateInt01` — conservative
- `h` → `stateInt02` — conservative
- `G` → `stateInt03` — conservative
- `z` → `stateInt04` — conservative
- `m` → `stateInt05` — conservative
- `k` → `stateInt06` — conservative
- `A` → `stateInt07` — conservative
- `f` → `stateFlag01` — conservative
- `x` → `stateFlag02` — conservative
- `t` → `stateFlag03` — conservative
- `I` → `stateFlag04` — conservative
- `F` → `stateFlag05` — conservative
- `e` → `stateFlag06` — conservative
- `p` → `stateFlag07` — conservative
- `j` → `stateFlag08` — conservative
- `C` → `stateFlag09` — conservative
- `l` → `stateFlag10` — conservative
- `q` → `stateFlag11` — conservative
- `B` → `stateInt08` — conservative
- `b` → `stateInt09` — conservative
- `a` → `stateByte01` — conservative
- `r` → `stateByte02` — conservative

### Methods
- `a(int,int,int,h,CrystalMidlet)` → `PlayerController`
- `a(int,int,int)` → `initialize`
- `e()` → `resetState`
- `k()` → `initializeSpriteState`
- `a(int,int)` → `setPosition`
- `c(int)` → `setDirection`
- `b(int)` → `setAction`
- `i()` → `refreshWorldReference`
- `a(int,int,int,int,boolean)` → `unknownMethod01`
- `a(int,int,boolean)` → `unknownMethod02`
- `a(int)` → `updateMovement`
- `b(int,int)` → `unknownMethod03`
- `g()` → `unknownMethod04`
- `a()` → `unknownMethod05`
- `j()` → `unknownMethod06`
- `l()` → `jump`
- `b()` → `moveLeft`
- `c()` → `moveRight`
- `c(int,int)` → `setVelocity`
- `d(int)` → `setAnimation`
- `h()` → `unknownMethod07`
- `f()` → `unknownMethod08`
- `a(int,boolean)` → `unknownMethod09`
- `d()` → `update`
- `a(javax.microedition.lcdui.Graphics)` → `draw`

## b → ReplayEvent

### Fields
- `d` → `frame` — high
- `b` → `eventType` — high
- `c` → `value` — high
- `a` → `next` — high

### Methods
- `b(short,int,byte)` → `ReplayEvent`

## c → PackedResourceReader

### Fields
- `b` → `resourcePath` — high
- `f` → `chunkCount` — high
- `a` → `chunkOffsets` — high
- `i` → `chunkLengths` — high
- `d` → `currentChunk` — high
- `c` → `streamPosition` — high
- `g` → `dataStartOffset` — high
- `e` → `inputStream` — high
- `h` → `chunkBuffer` — high

### Methods
- `c(java.lang.String)` → `PackedResourceReader`
- `a()` → `readNextChunk`
- `c(int)` → `readChunk`
- `a(int)` → `seekChunk`

## d → ReplayReader

### Fields
- `d` → `finished` — high
- `b` → `currentFrame` — high
- `a` → `currentEvent` — high
- `c` → `lastEvent` — high

### Methods
- `d()` → `ReplayReader`
- `a()` → `loadReplay`
- `a(b)` → `appendEvent`
- `a(byte[])` → `applyEvents`

## e → ReplayState

### Fields
- `c` → `frame` — high
- `a` → `value` — high
- `e` → `enabled` — high
- `b` → `currentEvent` — high
- `d` → `nextEvent` — high

### Methods
- `e()` → `ReplayState`

## f → GameTimerTask

### Fields
- `b` → `task` — high
- `a` → `timer` — high

### Methods
- `f(java.lang.Runnable,int)` → `GameTimerTask`
- `run()` → `run`
- `a()` → `stop`

## g → WorldMap

### Fields
- `ad` → `resourceName` — high
- `N` → `backgroundImage` — high
- `V` → `tilesetImages` — high
- `v` → `collisionData` — high
- `T` → `tileOffsets` — high
- `b` → `tileData` — high
- `E` → `objectImages` — high
- `l` → `layerData` — high
- `s` → `collisionMasks` — high
- `af` → `layerOffsetsA` — high
- `Y` → `layerOffsets` — high
- `P` → `tileProperties` — high
- `R` → `tileMap` — high
- `e` → `graphics` — high
- `w` → `directGraphics` — high
- `Z` → `collisionGrid` — high
- `p` → `objectPositions` — high
- `F` → `foregroundImage` — high
- `X` → `stateFlag01` — conservative
- `d` → `stateFlag02` — conservative
- `C` → `stateFlag03` — conservative
- `ab` → `stateFlag04` — conservative
- `t` → `stateFlag05` — conservative
- `J` → `stateFlag06` — conservative
- `z` → `stateInt01` — conservative
- `q` → `stateInt02` — conservative
- `i` → `stateInt03` — conservative
- `f` → `stateInt04` — conservative
- `A` → `stateInt05` — conservative
- `h` → `stateInt06` — conservative
- `U` → `stateInt07` — conservative
- `O` → `stateByte01` — conservative
- `m` → `stateByte02` — conservative
- `aa` → `stateByte03` — conservative
- `ai` → `stateByte04` — conservative
- `n` → `stateInt08` — conservative
- `a` → `stateInt09` — conservative
- `ac` → `stateInt10` — conservative
- `ae` → `stateInt11` — conservative
- `r` → `stateInt12` — conservative
- `y` → `stateInt13` — conservative
- `u` → `stateInt14` — conservative
- `o` → `stateInt15` — conservative
- `j` → `stateInt16` — conservative
- `Q` → `stateInt17` — conservative
- `L` → `stateInt18` — conservative
- `M` → `stateInt19` — conservative
- `g` → `stateInt20` — conservative
- `K` → `stateInt21` — conservative
- `I` → `stateInt22` — conservative
- `W` → `stateInt23` — conservative
- `D` → `stateInt24` — conservative
- `G` → `stateInt25` — conservative
- `x` → `stateInt26` — conservative
- `aj` → `stateInt27` — conservative
- `H` → `stateInt28` — conservative
- `k` → `stateInt29` — conservative
- `B` → `stateInt30` — conservative
- `ah` → `stateInt31` — conservative
- `ag` → `stateInt32` — conservative
- `c` → `stateInt33` — conservative
- `S` → `stateInt34` — conservative
- `ak` → `stateInt35` — conservative

### Methods
- `g(byte[],int,byte[],int,byte[],int,java.lang.String,java.lang.String,int,int,int,int,boolean,boolean)` → `WorldMap`
- `c(int,int)` → `setCamera`
- `d()` → `update`
- `a(int,int,int,int,boolean[][],boolean)` → `checkCollision`
- `a(int,int,int,int,int,int,boolean[][])` → `unknownMethod01`
- `a(javax.microedition.lcdui.Graphics)` → `draw`
- `a(int)` → `getTileProperty`
- `b(int)` → `getTileType`
- `c()` → `reset`
- `b()` → `isReady`
- `b(boolean)` → `setCollisionMode`
- `a(boolean)` → `setVisible`
- `e()` → `unknownMethod02`
- `a(int,int,int,boolean,int)` → `updateTile`
- `a()` → `unknownMethod03`
- `b(int,int)` → `staticHelperB`
- `a(int,int)` → `staticHelperA`

## h → BounceBackGameCanvas

### Fields
- `H` → `midlet` — high
- `e` → `player` — high
- `Z` → `world` — high
- `A` → `overlayWorld` — high
- `aj` → `gameTimer` — high
- `au` → `replayReader` — high
- `aa` → `replayState` — high
- `K` → `directGraphics` — high
- `S` → `backgroundImages` — high
- `x` → `gameImages` — high
- `ac` → `uiImages` — high
- `F` → `font` — high
- `r` → `levelNames` — high
- `at` → `unknownField01` — conservative
- `C` → `stateInt01` — conservative
- `h` → `stateInt02` — conservative
- `X` → `stateInt03` — conservative
- `c` → `stateInt04` — conservative
- `z` → `stateInt05` — conservative
- `y` → `stateByte01` — conservative
- `t` → `stateLong01` — conservative
- `g` → `stateLong02` — conservative
- `a` → `stateInt06` — conservative
- `as` → `stateInt07` — conservative
- `ai` → `stateInt08` — conservative
- `aq` → `stateInt09` — conservative
- `n` → `stateFlag01` — conservative
- `U` → `stateInt10` — conservative
- `w` → `stateInt11` — conservative
- `p` → `stateFlag02` — conservative
- `af` → `stateInt12` — conservative
- `M` → `stateFlag03` — conservative
- `ae` → `stateInt13` — conservative
- `d` → `stateInt14` — conservative
- `W` → `stateInt15` — conservative
- `ab` → `stateFlag04` — conservative
- `V` → `stateFlag05` — conservative
- `ad` → `stateFlag06` — conservative
- `L` → `stateInt16` — conservative
- `ap` → `stateFlag07` — conservative
- `b` → `stateByte02` — conservative
- `ar` → `stateInt17` — conservative
- `D` → `stateInt18` — conservative
- `Q` → `image01` — conservative
- `I` → `stateInt19` — conservative
- `o` → `stateFlag08` — conservative
- `P` → `stateInt20` — conservative
- `R` → `stateInt21` — conservative
- `q` → `stateFlag09` — conservative
- `E` → `stateFlag10` — conservative
- `i` → `stateInt22` — conservative
- `av` → `image02` — conservative
- `T` → `stateInt23` — conservative
- `Y` → `stateInt24` — conservative
- `G` → `stateInt25` — conservative
- `j` → `stateInt26` — conservative
- `ao` → `stateByte03` — conservative
- `ag` → `stateInt27` — conservative
- `s` → `stateByte04` — conservative
- `f` → `stateInt28` — conservative
- `k` → `stateInt29` — conservative
- `B` → `stateByte05` — conservative
- `v` → `stateInt30` — conservative
- `J` → `stateInt31` — conservative
- `m` → `imageSet01` — conservative
- `N` → `stateLong03` — conservative
- `ah` → `stateChar01` — conservative
- `O` → `stateInt32` — conservative
- `al` → `imageSet02` — conservative
- `ak` → `imageSet03` — conservative
- `u` → `imageSet04` — conservative
- `l` → `image03` — conservative
- `am` → `imageSet05` — conservative
- `an` → `imageSet06` — conservative

### Methods
- `h(CrystalMidlet)` → `BounceBackGameCanvas`
- `a(int)` → `unknownMethod01`
- `b(int)` → `unknownMethod02`
- `i()` → `unknownMethod03`
- `f()` → `unknownMethod04`
- `b()` → `unknownMethod05`
- `a(short,short,int[][])` → `readCellValue`
- `a(java.io.DataOutputStream)` → `writeSaveData`
- `paint(javax.microedition.lcdui.Graphics)` → `unknownMethod06`
- `a(javax.microedition.lcdui.Graphics)` → `unknownMethod07`
- `c(javax.microedition.lcdui.Graphics)` → `unknownMethod08`
- `a(javax.microedition.lcdui.Graphics,com.nokia.mid.ui.DirectGraphics)` → `unknownMethod09`
- `run()` → `unknownMethod10`
- `a()` → `unknownMethod11`
- `hideNotify()` → `unknownMethod12`
- `h()` → `unknownMethod13`
- `keyPressed(int)` → `unknownMethod14`
- `keyReleased(int)` → `unknownMethod15`
- `keyRepeated(int)` → `unknownMethod16`
- `a(int,int)` → `unknownMethod17`
- `c()` → `unknownMethod18`
- `a(int,int,boolean,boolean)` → `unknownMethod19`
- `e()` → `unknownMethod20`
- `a(java.lang.String,int,int)` → `loadImage`
- `g()` → `unknownMethod21`
- `a(int,boolean)` → `getAnimatedFrameA`
- `b(int,boolean)` → `getAnimatedFrameB`
- `d()` → `unknownMethod22`
- `b(javax.microedition.lcdui.Graphics)` → `unknownMethod23`
- `a(int,int,int,int,int,int,int,int)` → `intersects`

## i → GameMenuCanvas

### Fields
- `H` → `midlet` — high
- `w` → `commandListener` — high
- `R` → `primaryCommand` — high
- `d` → `secondaryCommand` — high
- `m` → `backCommand` — high
- `B` → `selectCommand` — high
- `l` → `cancelCommand` — high
- `z` → `items` — high
- `q` → `secondaryItems` — high
- `Z` → `largeFont` — high
- `c` → `smallFont` — high
- `E` → `headerImage` — high
- `f` → `footerImage` — high
- `C` → `backgroundImage` — high
- `M` → `selectionImage` — high
- `u` → `gameTimerTask01` — conservative
- `D` → `stateInt01` — conservative
- `W` → `stateInt02` — conservative
- `K` → `stateInt03` — conservative
- `v` → `stateInt04` — conservative
- `L` → `stateInt05` — conservative
- `o` → `stateInt06` — conservative
- `k` → `stateInt07` — conservative
- `y` → `stateFlag01` — conservative
- `p` → `stateFlag02` — conservative
- `A` → `stateFlag03` — conservative
- `X` → `stateInt08` — conservative
- `F` → `stateFlag04` — conservative
- `e` → `stateInt09` — conservative
- `g` → `stateByte01` — conservative
- `V` → `stateByte02` — conservative
- `J` → `string01` — conservative
- `aa` → `stateFlag05` — conservative
- `I` → `string02` — conservative
- `U` → `string03` — conservative
- `j` → `stateInt10` — conservative
- `x` → `stateInt11` — conservative
- `ab` → `stateInt12` — conservative
- `G` → `stateFlag06` — conservative
- `O` → `stateInt13` — conservative
- `Y` → `stateInt14` — conservative
- `S` → `stateInt15` — conservative
- `n` → `stateInt16` — conservative
- `h` → `stateInt17` — conservative
- `t` → `stateInt18` — conservative
- `T` → `stateInt19` — conservative
- `a` → `stateFlag07` — conservative
- `P` → `stateFlag08` — conservative
- `Q` → `image01` — conservative
- `b` → `image02` — conservative
- `r` → `stateInt20` — conservative
- `i` → `stateInt21` — conservative
- `N` → `stateInt22` — conservative
- `s` → `stateInt23` — conservative

### Methods
- `i(int,int,int)` → `GameMenuCanvas`
- `a(int,int,int)` → `configureLayout`
- `a(int)` → `setMode`
- `setCommandListener(javax.microedition.lcdui.CommandListener)` → `setCommandListener`
- `paint(javax.microedition.lcdui.Graphics)` → `paint`
- `showNotify()` → `unknownMethod01`
- `run()` → `run`
- `a(java.lang.String[])` → `setItems`
- `addCommand(javax.microedition.lcdui.Command)` → `addCommand`
- `a(javax.microedition.lcdui.Graphics,java.lang.String,int,int,int,int,javax.microedition.lcdui.Font,int,boolean)` → `drawTextAligned`
- `a(javax.microedition.lcdui.Graphics,java.lang.String,int,int,int,int,javax.microedition.lcdui.Font,int,boolean,boolean)` → `drawTextAligned`
- `i(CrystalMidlet,java.lang.String[],byte,java.lang.String,java.lang.String,java.lang.String,byte,boolean)` → `GameMenuCanvas`
- `i(CrystalMidlet,java.lang.String[],byte,java.lang.String,java.lang.String,java.lang.String)` → `GameMenuCanvas`
- `a(java.lang.String[],byte,java.lang.String,java.lang.String,java.lang.String,byte)` → `configureMenu`
- `a(java.lang.String[],java.lang.String[],byte,java.lang.String,java.lang.String,java.lang.String)` → `configureMenuWithSubitems`
- `b()` → `unknownMethod02`
- `c(int)` → `unknownMethod03`
- `d(int)` → `unknownMethod04`
- `keyPressed(int)` → `keyPressed`
- `e(int)` → `unknownMethod05`
- `b(int)` → `unknownMethod06`
- `a(javax.microedition.lcdui.Graphics)` → `unknownMethod07`
- `b(javax.microedition.lcdui.Graphics)` → `unknownMethod08`
- `d(javax.microedition.lcdui.Graphics,int)` → `unknownMethod09`
- `c(javax.microedition.lcdui.Graphics,int)` → `unknownMethod10`
- `b(javax.microedition.lcdui.Graphics,int)` → `unknownMethod11`
- `a(javax.microedition.lcdui.Graphics,int)` → `unknownMethod12`
- `a(javax.microedition.lcdui.Command)` → `handleCommand`
- `a(boolean)` → `setEnabled`
- `b(boolean)` → `setSecondaryEnabled`
- `a(int,int,int,int)` → `unknownMethod13`
- `c(boolean)` → `unknownMethod14`
- `hideNotify()` → `hideNotify`
- `a()` → `reset`
- `d()` → `close`
- `c()` → `refresh`
- `a(java.lang.String)` → `findItem`
